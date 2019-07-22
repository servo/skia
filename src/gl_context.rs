/*
 * Copyright 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use skia;

use euclid::default::Size2D;
use gleam::gl;
use std::ptr;
use std::rc::Rc;
use std::sync::Arc;

#[cfg(target_os="macos")]
pub use gl_context_cgl::GLPlatformContext;
#[cfg(target_os="macos")]
pub use gl_context_cgl::PlatformDisplayData;

#[cfg(target_os="linux")]
pub use gl_context_glx::GLPlatformContext;
#[cfg(target_os="linux")]
pub use gl_context_glx::PlatformDisplayData;
#[cfg(target_os="linux")]
pub use gl_rasterization_context::GLRasterizationContext;

#[cfg(target_os="android")]
pub use gl_context_android::GLPlatformContext;
#[cfg(target_os="android")]
pub use gl_context_android::PlatformDisplayData;

#[cfg(target_os="windows")]
pub use gl_context_wgl::GLPlatformContext;
#[cfg(target_os="windows")]
pub use gl_context_wgl::PlatformDisplayData;

pub struct GLContext {
    gl: Rc<gl::Gl>,
    pub platform_context: GLPlatformContext,
    pub gr_context: skia::SkiaGrContextRef,
    pub gl_interface: skia::SkiaGrGLInterfaceRef,
    pub size: Size2D<i32>,
}

impl Drop for GLContext {
    fn drop(&mut self) {
        self.platform_context.make_current();

        unsafe {
            skia::SkiaGrContextRelease(self.gr_context);
            skia::SkiaGrGLInterfaceRelease(self.gl_interface);
        }
    }
}

impl GLContext {
    pub fn new(gl: Rc<gl::Gl>,
               platform_display_data: PlatformDisplayData,
               size: Size2D<i32>)
               -> Option<Arc<GLContext>> {

        let platform_context = GLPlatformContext::new(gl.clone(), platform_display_data, size);
        let platform_context = match platform_context {
            Some(platform_context) => platform_context,
            None => return None,
        };

        // The Skia GL interface needs to be created while the context is active, so we
        // do that immediately after setting the context as the current one.
        platform_context.make_current();

        unsafe {
            let gl_interface = skia::SkiaGrGLCreateNativeInterface();
            if gl_interface == ptr::null_mut() {
                platform_context.drop_current_context();
                return None;
            }

            let gr_context = skia::SkiaGrContextCreate(gl_interface);
            if gr_context == ptr::null_mut() {
                platform_context.drop_current_context();
                return None;
            }

            Some(Arc::new(GLContext {
                gl: gl,
                platform_context: platform_context,
                gr_context: gr_context,
                gl_interface: gl_interface,
                size: size,
            }))
        }
   }

    pub fn gl(&self) -> &gl::Gl {
        &*self.gl
    }

    pub fn flush(&self) {
        self.make_current();
        self.gl.flush();
    }

    pub fn make_current(&self) {
        self.platform_context.make_current();
    }

    pub fn drop_current_context(&self) {
        self.platform_context.drop_current_context();
    }
}
