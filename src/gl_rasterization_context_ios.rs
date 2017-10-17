/*
 * Copyright 2013, 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use gl_context::GLContext;

use euclid::Size2D;
use std::sync::Arc;

pub struct GLRasterizationContext {
    pub gl_context: Arc<GLContext>,
}

impl Drop for GLRasterizationContext {
    fn drop(&mut self) {
    }
}

impl GLRasterizationContext {
    pub fn new(_gl_context: Arc<GLContext>,
               _size: Size2D<i32>)
               -> Option<GLRasterizationContext> {
        None
    }

    pub fn make_current(&self) {
        unimplemented!();
    }

    pub fn flush(&self) {
        unimplemented!();
    }

    pub fn flush_to_surface(&self) {
        unimplemented!();
    }
}
