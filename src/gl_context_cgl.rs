/*
 * Copyright 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use euclid::size::Size2D;
use cgl;
use gleam::gl;
use std::ptr;

pub struct PlatformDisplayData {
    pub pixel_format: cgl::CGLPixelFormatObj,
}

pub struct GLPlatformContext {
    pub cgl_context: cgl::CGLContextObj,
}

impl Drop for GLPlatformContext {
    fn drop(&mut self) {
        unsafe {
            cgl::CGLDestroyContext(self.cgl_context);
        }
    }
}

impl GLPlatformContext {
    pub fn new(platform_display_data: PlatformDisplayData,
               _: Size2D<i32>)
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

            Some(GLPlatformContext {
                cgl_context: cgl_context,
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
