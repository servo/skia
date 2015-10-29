/*
 * Copyright 2013, 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use gl_context::GLContext;
use gl_rasterization_context;

use euclid::size::Size2D;
use gleam::gl;
use std::sync::Arc;

pub struct GLRasterizationContext {
    pub gl_context: Arc<GLContext>,
    pub size: Size2D<i32>,
    pub framebuffer_id: gl::GLuint,
    texture_id: gl::GLuint,
    depth_stencil_renderbuffer_id: gl::GLuint,
}

impl Drop for GLRasterizationContext {
    fn drop(&mut self) {
        self.make_current();

        gl_rasterization_context::destroy_framebuffer(self.framebuffer_id,
                                                      self.texture_id,
                                                      self.depth_stencil_renderbuffer_id);
    }
}

impl GLRasterizationContext {
    pub fn new(gl_context: Arc<GLContext>,
               size: Size2D<i32>)
               -> Option<GLRasterizationContext> {
        None
    }

    pub fn make_current(&self) {
        self.gl_context.make_current();
    }

    pub fn flush(&self) {
        self.make_current();
        gl::flush();
    }

    pub fn flush_to_surface(&self) {
        gl::bind_framebuffer(gl::READ_FRAMEBUFFER, self.framebuffer_id);
        gl::framebuffer_texture_2d(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, 0, 0);
    }
}
