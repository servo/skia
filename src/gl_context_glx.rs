/*
 * Copyright 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use gl_rasterization_context;
use skia;

use euclid::size::Size2D;
use glx;
use std::ptr;
use x11::xlib;

use gleam::gl;

pub struct PlatformDisplayData {
    pub display: *mut xlib::Display,
    pub visual_info: *mut xlib::XVisualInfo,
}

pub struct GLPlatformContext {
    pub display: *mut xlib::Display,
    glx_context: xlib::XID,
    pub glx_pixmap: xlib::XID,
    pub pixmap: xlib::XID,

    pub framebuffer_id: gl::GLuint,
    pub texture_id: gl::GLuint,
    pub depth_stencil_renderbuffer_id: gl::GLuint,
}

impl Drop for GLPlatformContext {
    fn drop(&mut self) {
        // We need this thread to grab the GLX context before we can make
        // OpenGL calls. But glXMakeCurrent() will flush the old context,
        // which might have been uninitialized. Dropping the current context
        // first solves this problem somehow.
        self.drop_current_context();
        self.make_current();

        gl_rasterization_context::destroy_framebuffer(self.framebuffer_id,
                                                      self.texture_id,
                                                      self.depth_stencil_renderbuffer_id);

        unsafe {
            let glx_display = self.display as *mut glx::types::Display;
            glx::MakeCurrent(glx_display, 0 /* None */, ptr::null_mut());
            glx::DestroyContext(glx_display, self.glx_context as glx::types::GLXContext);
            glx::DestroyGLXPixmap(glx_display, self.glx_pixmap);
            xlib::XFreePixmap(self.display, self.pixmap);
        }
    }
}

impl GLPlatformContext {
    pub fn new(platform_display_data: PlatformDisplayData,
               size: Size2D<i32>)
               -> Option<GLPlatformContext> {
        unsafe {
            let display = platform_display_data.display;
            let visual_info = platform_display_data.visual_info;
            let glx_display = display as *mut glx::types::Display;
            let glx_visual_info = visual_info as *mut glx::types::XVisualInfo;

            let root_window = xlib::XRootWindow(display, xlib::XDefaultScreen(display));
            let pixmap = xlib::XCreatePixmap(display,
                                             root_window,
                                             size.width as u32,
                                             size.height as u32,
                                             (*visual_info).depth as u32);
            let glx_pixmap = glx::CreateGLXPixmap(glx_display,
                                                  glx_visual_info,
                                                  pixmap);

            let glx_context = glx::CreateContext(glx_display,
                                                 glx_visual_info,
                                                 ptr::null_mut(),
                                                 1);

            if glx_context == ptr::null() {
                glx::DestroyGLXPixmap(glx_display, glx_pixmap);
            }

            glx::MakeCurrent(display as *mut glx::types::Display,
                             glx_pixmap,
                             glx_context as glx::types::GLXContext);

            let gl_interface = skia::SkiaGrGLCreateNativeInterface();
            if gl_interface == ptr::null_mut() {
                glx::MakeCurrent(display as *mut glx::types::Display,
                                 0 /* None */,
                                 ptr::null_mut());
                return None;
            }

            if let Some((framebuffer_id, texture_id, depth_stencil_renderbuffer_id)) =
                gl_rasterization_context::setup_framebuffer(gl::TEXTURE_2D,
                                                            size,
                                                            gl_interface,
                                                            || {
                gl::tex_image_2d(gl::TEXTURE_2D, 0,
                                 gl::RGBA as gl::GLint,
                                 size.width, size.height, 0,
                                 gl::RGBA, gl::UNSIGNED_BYTE, None);
            }) {
                skia::SkiaGrGLInterfaceRelease(gl_interface);
                return Some(GLPlatformContext {
                    display: display,
                    glx_context: glx_context as xlib::XID,
                    pixmap: pixmap,
                    glx_pixmap: glx_pixmap as xlib::XID,
                    framebuffer_id: framebuffer_id,
                    texture_id: texture_id,
                    depth_stencil_renderbuffer_id: depth_stencil_renderbuffer_id,
                })
            }
            skia::SkiaGrGLInterfaceRelease(gl_interface);
            None
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
}
