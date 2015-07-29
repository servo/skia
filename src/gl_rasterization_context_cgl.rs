/*
 * Copyright 2013, 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use gl_context::GLContext;
use gl_rasterization_context;

use cgl;
use euclid::size::Size2D;
use gleam::gl;
use io_surface;
use libc;
use std::sync::Arc;

pub struct GLRasterizationContext {
    pub gl_context: Arc<GLContext>,
    pub size: Size2D<i32>,
    pub framebuffer_id: gl::GLuint,
}

impl GLRasterizationContext {
    pub fn new(gl_context: Arc<GLContext>,
               io_surface: io_surface::IOSurfaceRef,
               size: Size2D<i32>)
               -> Option<GLRasterizationContext> {
        if !gl_rasterization_context::finish_framebuffer_setup(size, || {
            unsafe {
                cgl::CGLTexImageIOSurface2D(gl_context.platform_context.cgl_context,
                                            gl::TEXTURE_RECTANGLE_ARB, gl::RGBA,
                                            size.width, size.height,
                                            gl::BGRA, gl::UNSIGNED_INT_8_8_8_8_REV,
                                            io_surface as *mut libc::c_void,
                                            0);
            }
        }) {
            return None;
        }

        Some(GLRasterizationContext {
            gl_context: gl_context.clone(),
            size: size,
            framebuffer_id: gl_context.platform_context.framebuffer_id,
        })
    }

    pub fn make_current(&self) {
        self.gl_context.make_current();
    }

    pub fn flush(&self) {
        self.make_current();
        gl::flush();
    }

    pub fn flush_to_surface(&self) {
        gl::bind_framebuffer(gl::FRAMEBUFFER, self.framebuffer_id);
    }
}
