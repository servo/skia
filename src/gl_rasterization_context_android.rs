/*
 * Copyright 2013, 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use gl_context::GLContext;
use gl_rasterization_context;

use euclid::size::Size2D;
use egl::egl;
use egl::eglext;
use gleam::gl;
use std::sync::Arc;

pub struct GLRasterizationContext {
    pub gl_context: Arc<GLContext>,
    pub egl_image: eglext::EGLImageKHR,
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
            let egl_image_attributes = [
                eglext::EGL_IMAGE_PRESERVED_KHR as i32, egl::EGL_TRUE as i32,
                egl::EGL_NONE as i32, egl::EGL_NONE as i32,
            ];
            let egl_image = eglext::CreateImageKHR(gl_context.platform_context.display,
                                                   gl_context.platform_context.egl_context,
                                                   eglext::EGL_GL_TEXTURE_2D_KHR,
                                                   texture_id as egl::EGLClientBuffer,
                                                   egl_image_attributes.as_ptr());

            return Some(GLRasterizationContext {
                gl_context: gl_context,
                egl_image: egl_image,
                size: size,
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
        gl::bind_framebuffer(0x8CA8 as gl::GLenum, self.framebuffer_id);
        gl::framebuffer_texture_2d(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, 0, 0);
    }
}
