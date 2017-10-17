/*
 * Copyright 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */
use euclid::Size2D;
use gleam::gl;
use std::rc::Rc;

pub struct PlatformDisplayData {
}

pub struct GLPlatformContext {
}

impl Drop for GLPlatformContext {
    fn drop(&mut self) {
        self.drop_current_context();
    }
}

impl GLPlatformContext {
    pub fn new(_: Rc<gl::Gl>,
               _platform_display_data: PlatformDisplayData,
               _size: Size2D<i32>)
               -> Option<GLPlatformContext> {
        None
    }

    pub fn drop_current_context(&self) {
        unimplemented!();
    }

    pub fn make_current(&self) {
        unimplemented!();
    }
}
