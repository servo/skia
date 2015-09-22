/*
 * Copyright 2013, 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use skia;

use euclid::size::Size2D;
use gleam::gl;
use std::ffi::CString;

#[cfg(target_os="macos")]
pub use gl_rasterization_context_cgl::GLRasterizationContext;
#[cfg(target_os="linux")]
pub use gl_rasterization_context_glx::GLRasterizationContext;
#[cfg(target_os="android")]
pub use gl_rasterization_context_android::GLRasterizationContext;
#[cfg(target_os="windows")]
pub use gl_rasterization_context_wgl::GLRasterizationContext;

fn clear_gl_errors() {
    let mut error = gl::get_error();
    while error != gl::NO_ERROR {
        error = gl::get_error();
    }
}

#[cfg(not(target_os = "android"))]
fn create_and_bind_depth_stencil_buffer(gl_interface: skia::SkiaGrGLInterfaceRef,
                                        size: Size2D<i32>)
                                        -> gl::GLuint {
    unsafe {
        let ext_extension_string = CString::new("GL_EXT_packed_depth_stencil").unwrap();
        let arb_extension_string = CString::new("GL_ARB_framebuffer_object").unwrap();
        let supports_depth_stencil =
            skia::SkiaGrGLInterfaceGLVersionGreaterThanOrEqualTo(gl_interface, 3, 0) ||
                skia::SkiaGrGLInterfaceHasExtension(gl_interface, ext_extension_string.as_ptr()) ||
                skia::SkiaGrGLInterfaceHasExtension(gl_interface, arb_extension_string.as_ptr());
        create_and_bind_depth_stencil_buffer_with_formats(supports_depth_stencil,
                                                          gl::DEPTH_STENCIL,
                                                          gl::STENCIL_INDEX,
                                                          size)
    }
}

#[cfg(target_os = "android")]
fn create_and_bind_depth_stencil_buffer(gl_interface: skia::SkiaGrGLInterfaceRef,
                                        size: Size2D<i32>)
                                        -> gl::GLuint {
    unsafe {
        let oes_extension_string = CString::new("GL_OES_packed_depth_stencil").unwrap();
        let supports_depth_stencil =
           skia::SkiaGrGLInterfaceHasExtension(gl_interface, oes_extension_string.as_ptr());
        const GL_DEPTH24_STENCIL8_OES: u32 = 0x88F0;
        create_and_bind_depth_stencil_buffer_with_formats(supports_depth_stencil,
                                                          GL_DEPTH24_STENCIL8_OES,
                                                          gl::STENCIL_INDEX8,
                                                          size)
    }
}

fn create_and_bind_depth_stencil_buffer_with_formats(supports_depth_stencil: bool,
                                                     depth_stencil_format: gl::GLenum,
                                                     stencil_format: gl::GLenum,
                                                     size: Size2D<i32>)
                                                     -> gl::GLuint {
    let depth_stencil_renderbuffer_id = gl::gen_renderbuffers(1)[0];
    gl::bind_renderbuffer(gl::RENDERBUFFER, depth_stencil_renderbuffer_id);

    unsafe {
        if supports_depth_stencil {
            gl::RenderbufferStorage(gl::RENDERBUFFER,
                                    depth_stencil_format,
                                    size.width,
                                    size.height);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER,
                                        gl::DEPTH_ATTACHMENT,
                                        gl::RENDERBUFFER,
                                        depth_stencil_renderbuffer_id);
        } else {
            gl::RenderbufferStorage(gl::RENDERBUFFER, stencil_format, size.width, size.height);
        }

        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER,
                                    gl::STENCIL_ATTACHMENT,
                                    gl::RENDERBUFFER,
                                    depth_stencil_renderbuffer_id);
    }

    depth_stencil_renderbuffer_id
}

pub fn setup_framebuffer<F>(texture_target: gl::GLenum,
                            size: Size2D<i32>,
                            gl_interface: skia::SkiaGrGLInterfaceRef,
                            set_texture_image: F)
                            -> Option<(gl::GLuint, gl::GLuint, gl::GLuint)>
                            where F: Fn() {

    let (framebuffer_id, texture_id, depth_stencil_renderbuffer_id) =
        start_framebuffer_setup(texture_target, size, gl_interface);

    if !finish_framebuffer_setup(size, set_texture_image) {
        destroy_framebuffer(framebuffer_id, texture_id, depth_stencil_renderbuffer_id);
        return None;
    }

    Some((framebuffer_id, texture_id, depth_stencil_renderbuffer_id))
}

pub fn start_framebuffer_setup(texture_target: gl::GLenum,
                               size: Size2D<i32>,
                               gl_interface: skia::SkiaGrGLInterfaceRef)
                               -> (gl::GLuint, gl::GLuint, gl::GLuint) {
    clear_gl_errors();

    let framebuffer_id = gl::gen_framebuffers(1)[0];
    gl::bind_framebuffer(gl::FRAMEBUFFER, framebuffer_id);

    let texture_id = gl::gen_textures(1)[0];
    gl::bind_texture(texture_target, texture_id);

    gl::tex_parameter_i(texture_target, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as gl::GLint);
    gl::tex_parameter_i(texture_target, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as gl::GLint);
    gl::tex_parameter_i(texture_target, gl::TEXTURE_MAG_FILTER, gl::NEAREST as gl::GLint);
    gl::tex_parameter_i(texture_target, gl::TEXTURE_MIN_FILTER, gl::NEAREST as gl::GLint);
    gl::framebuffer_texture_2d(gl::FRAMEBUFFER,
                               gl::COLOR_ATTACHMENT0,
                               texture_target,
                               texture_id, 0);

    let depth_stencil_renderbuffer_id = create_and_bind_depth_stencil_buffer(gl_interface,
                                                                             size);

    (framebuffer_id, texture_id, depth_stencil_renderbuffer_id)
}

pub fn finish_framebuffer_setup<F>(size: Size2D<i32>,
                                   set_texture_image: F)
                                   -> bool
                                   where F: Fn() {
    set_texture_image();

    gl::viewport(0, 0, size.width, size.height);

    unsafe {
        gl::get_error() == gl::NO_ERROR &&
            gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE
    }
}

pub fn destroy_framebuffer(framebuffer_id: gl::GLuint,
                           texture_id: gl::GLuint,
                           depth_stencil_renderbuffer_id: gl::GLuint) {
    gl::delete_framebuffers(&[framebuffer_id]);
    gl::delete_textures(&[texture_id]);
    gl::delete_renderbuffers(&[depth_stencil_renderbuffer_id]);
}
