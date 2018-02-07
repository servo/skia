/*
 * Copyright 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

extern crate glutin;

use gl_rasterization_context;
use skia;

use euclid::Size2D;
use gl_context_wgl::glutin::GlContext;
use gleam::gl;
use std::ptr;
use std::rc::Rc;

pub struct PlatformDisplayData;

impl PlatformDisplayData {
    pub fn new() -> PlatformDisplayData {
        PlatformDisplayData
    }
}

pub struct GLPlatformContext {
    gl: Rc<gl::Gl>,
    pub context: glutin::HeadlessContext,

    pub framebuffer_id: gl::GLuint,
    texture_id: gl::GLuint,
    depth_stencil_renderbuffer_id: gl::GLuint,
}

impl Drop for GLPlatformContext {
    fn drop(&mut self) {
        self.make_current();
        gl_rasterization_context::destroy_framebuffer(self.gl(),
                                                      self.framebuffer_id,
                                                      self.texture_id,
                                                      self.depth_stencil_renderbuffer_id);
        self.destroy();
    }
}

impl GLPlatformContext {
    pub fn new(gl: Rc<gl::Gl>,
               _: PlatformDisplayData,
               size: Size2D<i32>)
               -> Option<GLPlatformContext> {
        unsafe {
            // 32x32 is just the size of the dummy underlying context; the real
            // size is used below when we create the FBO
            let cx = glutin::HeadlessRendererBuilder::new(32, 32).build().unwrap();
            cx.make_current().expect("make_current failed");

            let gl_interface = skia::SkiaGrGLCreateNativeInterface();
            if gl_interface == ptr::null_mut() {
                //cx.destroy();
                return None
            }

            let (framebuffer_id, texture_id, depth_stencil_renderbuffer_id) =
                gl_rasterization_context::setup_framebuffer(&*gl,
                                                            gl::TEXTURE_2D,
                                                            size,
                                                            gl_interface,
                                                            || {
                                                                gl.tex_image_2d(gl::TEXTURE_2D, 0,
                                                                                gl::RGBA as gl::GLint,
                                                                                size.width, size.height, 0,
                                                                                gl::RGBA, gl::UNSIGNED_BYTE, None);
                                                            }).unwrap();

            skia::SkiaGrGLInterfaceRelease(gl_interface);
            Some(GLPlatformContext {
                gl: gl,
                context: cx,
                framebuffer_id: framebuffer_id,
                texture_id: texture_id,
                depth_stencil_renderbuffer_id: depth_stencil_renderbuffer_id,
            })
        }
    }

    fn gl(&self) -> &gl::Gl {
        &*self.gl
    }

    pub fn drop_current_context(&self) {
        // TODO; should not be necessary
    }

    pub fn destroy(&self) {
        // TODO; need to extend glutin
    }

    pub fn make_current(&self) {
        unsafe {
            self.context.make_current().expect("make_current failed");
        }
    }
}
