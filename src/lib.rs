/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![crate_name = "skia"]
#![crate_type = "rlib"]

#![feature(libc)]

extern crate libc;

pub use skia::{
    SkiaSkNativeSharedGLContextRef,
    SkiaGrContextRef,
    SkiaGrGLSharedSurfaceRef,
    SkiaGrGLNativeContextRef,
    SkiaSkNativeSharedGLContextCreate,
    SkiaSkNativeSharedGLContextRetain,
    SkiaSkNativeSharedGLContextRelease,
    SkiaSkNativeSharedGLContextGetFBOID,
    SkiaSkNativeSharedGLContextStealSurface,
    SkiaSkNativeSharedGLContextGetGrContext,
    SkiaSkNativeSharedGLContextMakeCurrent,
    SkiaSkNativeSharedGLContextFlush,
};

pub mod skia;
