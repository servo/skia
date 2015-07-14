/*
 * Copyright 2013, 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use gl_rasterization_context;
use skia;

use euclid::size::Size2D;
use gleam::gl;
use glx;
use std::ptr;
use x11::xlib;

pub struct GLRasterizationContext {
    display: *const xlib::Display,
    glx_context: xlib::XID,
    glx_pixmap: xlib::XID,
    pub size: Size2D<i32>,

    pub framebuffer_id: gl::GLuint,
    texture_id: gl::GLuint,
    depth_stencil_renderbuffer_id: gl::GLuint,
    pub gr_context: skia::SkiaGrContextRef,
}

impl Drop for GLRasterizationContext {
    fn drop(&mut self) {
        // We need this thread to grab the GLX context before we can make
        // OpenGL calls. But glXMakeCurrent() will flush the old context,
        // which might have been uninitialized. Dropping the current context
        // first solves this problem somehow.
        self.drop_current_context();
        self.make_current();

        gl_rasterization_context::destroy_framebuffer(self.gr_context,
                                                      self.framebuffer_id,
                                                      self.texture_id,
                                                      self.depth_stencil_renderbuffer_id);

        unsafe {
            let glx_display = self.display as *mut glx::types::Display;
            glx::MakeCurrent(glx_display, 0 /* None */, ptr::null_mut());
            glx::DestroyContext(glx_display, self.glx_context as glx::types::GLXContext);
            glx::DestroyGLXPixmap(glx_display, self.glx_pixmap);
        }
    }
}

impl GLRasterizationContext {
    pub fn new(display: *mut xlib::Display,
               visual_info: *mut xlib::XVisualInfo,
               pixmap: xlib::Pixmap,
               size: Size2D<i32>)
               -> Option<GLRasterizationContext> {
        unsafe {
            let glx_display = display as *mut glx::types::Display;
            let glx_visual_info = visual_info as *mut glx::types::XVisualInfo;
            let glx_pixmap = glx::CreateGLXPixmap(glx_display,
                                                  glx_visual_info,
                                                  pixmap);

            let glx_context = glx::CreateContext(glx_display,
                                                 glx_visual_info,
                                                 ptr::null_mut(),
                                                 1);

            if glx_context == ptr::null() {
                glx::DestroyGLXPixmap(glx_display, glx_pixmap);
                return None;
            }

            // The Skia GL interface needs to be created while the context is active, so we
            // do that immediately after setting the context as the current one.
            glx::MakeCurrent(glx_display, glx_pixmap, glx_context);

            let (framebuffer_id, texture_id, depth_stencil_renderbuffer_id, gr_context) =
                gl_rasterization_context::setup_framebuffer(gl::TEXTURE_2D, size, || {
                    gl::tex_image_2d(gl::TEXTURE_2D, 0,
                                     gl::RGBA as gl::GLint,
                                     size.width, size.height, 0,
                                     gl::RGBA, gl::UNSIGNED_BYTE, None);
                });

            if gr_context == ptr::null_mut() {
                glx::MakeCurrent(glx_display, 0 /* None */, ptr::null_mut());
                glx::DestroyContext(glx_display, glx_context);
                glx::DestroyGLXPixmap(glx_display, glx_pixmap);
                return None;
            }

            Some(GLRasterizationContext {
                display: display,
                glx_context: glx_context as xlib::XID,
                glx_pixmap: glx_pixmap as xlib::XID,
                size: size,
                framebuffer_id: framebuffer_id,
                texture_id: texture_id,
                depth_stencil_renderbuffer_id: depth_stencil_renderbuffer_id,
                gr_context: gr_context,
            })
        }
    }

    pub fn drop_current_context(&self) {
        unsafe {
            glx::MakeCurrent(self.display as *mut glx::types::Display,
                             0 /* None */,
                             ptr::null_mut());
        }
    }

    pub fn make_current(&self) {
        unsafe {
            glx::MakeCurrent(self.display as *mut glx::types::Display,
                             self.glx_pixmap,
                             self.glx_context as glx::types::GLXContext);
        }
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

        gl::flush();
        gl::bind_framebuffer(gl::FRAMEBUFFER, 0);
    }
}
