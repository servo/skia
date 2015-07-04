/*
 * Copyright 2013, 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use gl_rasterization_context;
use skia;

use euclid::size::Size2D;
use egl::egl;
use egl::eglext;
use gleam::gl;
use std::ffi::CString;
use std::ptr;

pub struct GLRasterizationContext {
    display: egl::EGLDisplay,
    egl_context: egl::EGLContext,
    egl_surface: egl::EGLSurface,
    pub egl_image: eglext::EGLImageKHR,
    pub size: Size2D<i32>,

    pub framebuffer_id: gl::GLuint,
    texture_id: gl::GLuint,
    depth_stencil_renderbuffer_id: gl::GLuint,
    pub gr_context: skia::SkiaGrContextRef,
}

impl Drop for GLRasterizationContext {
    fn drop(&mut self) {
        self.make_current();

        gl_rasterization_context::destroy_framebuffer(self.gr_context,
                                                      self.framebuffer_id,
                                                      self.texture_id,
                                                      self.depth_stencil_renderbuffer_id);

        self.drop_current_context();
        egl::DestroyContext(self.display, self.egl_context);
        egl::DestroySurface(self.display, self.egl_surface);
    }
}

impl GLRasterizationContext {
    pub fn new(display: egl::EGLDisplay,
               size: Size2D<i32>)
               -> Option<GLRasterizationContext> {
        unsafe {
            let config_attributes = [
                egl::EGL_SURFACE_TYPE as i32, egl::EGL_PBUFFER_BIT as i32,
                egl::EGL_RENDERABLE_TYPE as i32, egl::EGL_OPENGL_ES2_BIT as i32,
                egl::EGL_RED_SIZE as i32, 8,
                egl::EGL_BLUE_SIZE as i32, 8,
                egl::EGL_ALPHA_SIZE as i32, 8,
                egl::EGL_NONE as i32,
            ];

            let mut surface_config = ptr::null_mut();
            let mut number_of_configs = 0;
            egl::ChooseConfig(display, config_attributes.as_ptr(),
                              &mut surface_config, 1, &mut number_of_configs);
            if number_of_configs == 0 {
                return None;
            }

            let context_attributes = [
                egl::EGL_CONTEXT_CLIENT_VERSION as i32, 2,
                egl::EGL_NONE as i32
            ];
            let egl_context = egl::CreateContext(display,
                                                 surface_config,
                                                 egl::EGL_NO_CONTEXT as egl::EGLContext,
                                                 context_attributes.as_ptr());
            if egl_context == egl::EGL_NO_CONTEXT as egl::EGLContext {
                return None;
            }

            let mut surface_attributes = [
                egl::EGL_WIDTH as i32, size.width,
                egl::EGL_HEIGHT as i32, size.height,
                egl::EGL_NONE as i32,
            ];
            let egl_surface = egl::CreatePbufferSurface(display,
                                                        surface_config,
                                                        &mut surface_attributes[0]);
            if egl_surface == egl::EGL_NO_SURFACE as egl::EGLSurface {
                egl::DestroyContext(display, egl_context);
                return None;
            }

            let (framebuffer_id, texture_id, depth_stencil_renderbuffer_id, gr_context) =
                gl_rasterization_context::setup_framebuffer(gl::TEXTURE_2D, size, || {
                    gl::tex_image_2d(gl::TEXTURE_2D, 0,
                                     gl::RGBA as gl::GLint,
                                     size.width, size.height, 0,
                                     gl::RGBA, gl::UNSIGNED_BYTE, None);
                });

            if gr_context == ptr::null() {
                egl::MakeCurrent(display, ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
                egl::DestroyContext(display, egl_context);
                egl::DestroySurface(display, egl_surface);
                return None;
            }

            let egl_image_attributes = [
                eglext::EGL_IMAGE_PRESERVED_KHR as i32, egl::EGL_TRUE as i32,
                egl::EGL_NONE as i32, egl::EGL_NONE as i32,
            ];
            let egl_image = eglext::CreateImageKHR(display,
                                                   egl_context,
                                                   eglext::EGL_GL_TEXTURE_2D_KHR,
                                                   texture_id as egl::EGLClientBuffer,
                                                   egl_image_attributes.as_ptr());

            Some(GLRasterizationContext {
                display: display,
                egl_context: egl_context,
                egl_surface: egl_surface,
                egl_image: egl_image,
                size: size,
                framebuffer_id: framebuffer_id,
                texture_id: texture_id,
                depth_stencil_renderbuffer_id: depth_stencil_renderbuffer_id,
                gr_context: gr_context,
            })
        }
    }

    pub fn drop_current_context(&self) {
        egl::MakeCurrent(self.display, ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    }

    pub fn make_current(&self) {
        egl::MakeCurrent(self.display, self.egl_surface, self.egl_surface, self.egl_context);
    }

    pub fn flush(&self) {
        self.make_current();
        gl::flush();
    }

    pub fn flush_to_surface(&self) {
        gl::bind_framebuffer(0x8CA8 as gl::GLenum, self.framebuffer_id);
        gl::framebuffer_texture_2d(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, 0, 0);
    }
}
