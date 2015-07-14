/*
 * Copyright 2013, 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use gl_rasterization_context;
use skia;

use cgl;
use euclid::size::Size2D;
use gleam::gl;
use io_surface;
use libc;
use std::ptr;

pub struct GLRasterizationContext {
    cgl_context: cgl::CGLContextObj,
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
        unsafe {
            cgl::CGLDestroyContext(self.cgl_context);
        }
    }
}

impl GLRasterizationContext {
    pub fn new(pixel_format: cgl::CGLPixelFormatObj,
               io_surface: io_surface::IOSurfaceRef,
               size: Size2D<i32>)
               -> Option<GLRasterizationContext> {
        unsafe {
            let mut cgl_context = ptr::null_mut();
            let _ = cgl::CGLCreateContext(pixel_format, ptr::null_mut(), &mut cgl_context);
            if ptr::null_mut() == cgl_context {
                return None;
            }

            // The Skia GL interface needs to be created while the context is active, so we
            // do that immediately after setting the context as the current one.
            cgl::CGLSetCurrentContext(cgl_context);
            gl::enable(gl::TEXTURE_RECTANGLE_ARB);

            let (framebuffer_id, texture_id, depth_stencil_renderbuffer_id, gr_context) =
                gl_rasterization_context::setup_framebuffer(gl::TEXTURE_RECTANGLE_ARB, size, || {
                    cgl::CGLTexImageIOSurface2D(cgl_context,
                                                gl::TEXTURE_RECTANGLE_ARB, gl::RGBA,
                                                size.width, size.height,
                                                gl::BGRA, gl::UNSIGNED_INT_8_8_8_8_REV,
                                                io_surface as *mut libc::c_void,
                                                0);
                });

            if gr_context == ptr::null_mut() {
                cgl::CGLDestroyContext(cgl_context);
                return None;
            }

            Some(GLRasterizationContext {
                cgl_context: cgl_context,
                size: size,
                framebuffer_id: framebuffer_id,
                texture_id: texture_id,
                depth_stencil_renderbuffer_id: depth_stencil_renderbuffer_id,
                gr_context: gr_context,
            })
        }
    }

    pub fn make_current(&self) {
        unsafe {
            cgl::CGLSetCurrentContext(self.cgl_context);
        }
    }

    pub fn flush(&self) {
        self.make_current();
        gl::flush();
    }

    pub fn flush_to_surface(&self) {
        gl::bind_framebuffer(gl::FRAMEBUFFER, self.framebuffer_id);
        gl::framebuffer_texture_2d(gl::FRAMEBUFFER,
                                   gl::COLOR_ATTACHMENT0,
                                   gl::TEXTURE_RECTANGLE_ARB,
                                   0,
                                   0);
    }
}
