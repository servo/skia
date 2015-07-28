/*
 * Copyright 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use euclid::size::Size2D;
use egl::egl;
use std::ptr;

pub struct PlatformDisplayData {
    pub display: egl::EGLDisplay,
}

pub struct GLPlatformContext {
    pub display: egl::EGLDisplay,
    pub egl_context: egl::EGLContext,
    egl_surface: egl::EGLSurface,
}

impl Drop for GLPlatformContext {
    fn drop(&mut self) {
        self.drop_current_context();
        egl::DestroyContext(self.display, self.egl_context);
        egl::DestroySurface(self.display, self.egl_surface);
    }
}

impl GLPlatformContext {
    pub fn new(platform_display_data: PlatformDisplayData,
               size: Size2D<i32>)
               -> Option<GLPlatformContext> {
        let config_attributes = [
            egl::EGL_SURFACE_TYPE as i32, egl::EGL_PBUFFER_BIT as i32,
            egl::EGL_RENDERABLE_TYPE as i32, egl::EGL_OPENGL_ES2_BIT as i32,
            egl::EGL_RED_SIZE as i32, 8,
            egl::EGL_BLUE_SIZE as i32, 8,
            egl::EGL_ALPHA_SIZE as i32, 8,
            egl::EGL_NONE as i32,
        ];

        let display = platform_display_data.display;
        let mut surface_config = ptr::null_mut();
        let mut number_of_configs = 0;
        egl::ChooseConfig(display,
                          config_attributes.as_ptr(),
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


        Some(GLPlatformContext {
            display: display,
            egl_context: egl_context,
            egl_surface: egl_surface,
        })
    }

    pub fn drop_current_context(&self) {
        egl::MakeCurrent(self.display, ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    }

    pub fn make_current(&self) {
        egl::MakeCurrent(self.display, self.egl_surface, self.egl_surface, self.egl_context);
    }
}
