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
use std::ptr;
use std::sync::Arc;
use x11::xlib;

pub struct GLRasterizationContext {
    pub gl_context: Arc<GLContext>,
    pub size: Size2D<i32>,
    pub framebuffer_id: gl::GLuint,
    pixmap: xlib::XID,
    texture_id: gl::GLuint,
    depth_stencil_renderbuffer_id: gl::GLuint,
}

impl Drop for GLRasterizationContext {
    fn drop(&mut self) {
        self.gl_context.make_current();
        gl_rasterization_context::destroy_framebuffer(self.framebuffer_id,
                                                      self.texture_id,
                                                      self.depth_stencil_renderbuffer_id);

    }
}

impl GLRasterizationContext {
    pub fn new(gl_context: Arc<GLContext>,
               pixmap: xlib::Pixmap,
               size: Size2D<i32>)
               -> Option<GLRasterizationContext> {
        gl_context.make_current();

        if let Some((framebuffer_id, texture_id, depth_stencil_renderbuffer_id)) =
            gl_rasterization_context::setup_framebuffer(gl::TEXTURE_2D,
                                                        size,
                                                        gl_context.gl_interface,
                                                        || {
            gl::tex_image_2d(gl::TEXTURE_2D, 0,
                             gl::RGBA as gl::GLint,
                             size.width, size.height, 0,
                             gl::RGBA, gl::UNSIGNED_BYTE, None);
        }) {
            return Some(GLRasterizationContext {
                gl_context: gl_context,
                size: size,
                pixmap: pixmap,
                framebuffer_id: framebuffer_id,
                texture_id: texture_id,
                depth_stencil_renderbuffer_id: depth_stencil_renderbuffer_id,
            });
        }

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
        gl::bind_framebuffer(gl::DRAW_FRAMEBUFFER, 0);

        unsafe {
            gl::BlitFramebuffer(0, 0,
                                self.size.width, self.size.height,
                                0, 0,
                                self.size.width, self.size.height,
                                gl::COLOR_BUFFER_BIT, gl::NEAREST);
        }

        gl::finish();
        self.gl_context.drop_current_context();

        // Since the GLRasterizationContext renders to a Pixmap that is owned by the
        // GLContext, we now need to copy the results to the target Pixmap. This means
        // we do an extra hardware copy, but allows us to reuse the same GLContext.
        let display = self.gl_context.platform_context.display;
        let source_pixmap = self.gl_context.platform_context.pixmap;
        unsafe {
            let gc = xlib::XCreateGC(display, self.pixmap, 0, ptr::null_mut());
            xlib::XCopyArea(display, source_pixmap,
                            self.pixmap,
                            gc,
                            0, (self.gl_context.size.height - self.size.height),
                            self.size.width as u32, self.size.height as u32,
                            0, 0);
            xlib::XFreeGC(display, gc);
        }
    }
}
