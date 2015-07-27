/*
 * Copyright 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use gl_rasterization_context;
use skia;

use euclid::size::Size2D;
use cgl;
use gleam::gl;
use std::ptr;

pub struct PlatformDisplayData {
    pub pixel_format: cgl::CGLPixelFormatObj,
}

pub struct GLPlatformContext {
    pub cgl_context: cgl::CGLContextObj,

    pub framebuffer_id: gl::GLuint,
    texture_id: gl::GLuint,
    depth_stencil_renderbuffer_id: gl::GLuint,
}

impl Drop for GLPlatformContext {
    fn drop(&mut self) {
        self.make_current();
        gl_rasterization_context::destroy_framebuffer(self.framebuffer_id,
                                                      self.texture_id,
                                                      self.depth_stencil_renderbuffer_id);

        unsafe {
            cgl::CGLDestroyContext(self.cgl_context);
        }
    }
}

impl GLPlatformContext {
    pub fn new(platform_display_data: PlatformDisplayData,
               size: Size2D<i32>)
               -> Option<GLPlatformContext> {
        unsafe {
            let mut cgl_context = ptr::null_mut();
            let _ = cgl::CGLCreateContext(platform_display_data.pixel_format,
                                          ptr::null_mut(),
                                          &mut cgl_context);
            if ptr::null_mut() == cgl_context {
                return None;
            }

            cgl::CGLSetCurrentContext(cgl_context);
            gl::enable(gl::TEXTURE_RECTANGLE_ARB);

            let gl_interface = skia::SkiaGrGLCreateNativeInterface();
            if gl_interface == ptr::null_mut() {
                cgl::CGLDestroyContext(cgl_context);
                return None
            }

            // We only start the framebuffer setup here, since we cannot complete it until
            // we have a texture image. That will be provided by the IOSurface in the
            // GLRasterizationContext.
            let (framebuffer_id, texture_id, depth_stencil_renderbuffer_id) =
                gl_rasterization_context::start_framebuffer_setup(gl::TEXTURE_RECTANGLE_ARB,
                                                                  size,
                                                                  gl_interface);
            skia::SkiaGrGLInterfaceRelease(gl_interface);
            Some(GLPlatformContext {
                cgl_context: cgl_context,
                framebuffer_id: framebuffer_id,
                texture_id: texture_id,
                depth_stencil_renderbuffer_id: depth_stencil_renderbuffer_id,
            })
        }
    }

    pub fn drop_current_context(&self) {
        unsafe {
            cgl::CGLSetCurrentContext(ptr::null_mut());
        }
    }

    pub fn make_current(&self) {
        unsafe {
            cgl::CGLSetCurrentContext(self.cgl_context);
        }
    }
}
