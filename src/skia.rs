// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::*;

pub type SkiaSkNativeSharedGLContextRef = *mut c_void;
pub type SkiaGrContextRef = *mut c_void;
pub type SkiaGrGLSharedSurfaceRef = *mut c_void;
pub type SkiaGrGLNativeContextRef = *mut c_void;

#[link(name = "skia")]
extern {

pub fn SkiaSkNativeSharedGLContextCreate(aNativeContext: SkiaGrGLNativeContextRef, aWidth: i32, aHeight: i32) -> SkiaSkNativeSharedGLContextRef;
pub fn SkiaSkNativeSharedGLContextRetain(aGLContext: SkiaSkNativeSharedGLContextRef);
pub fn SkiaSkNativeSharedGLContextRelease(aGLContext: SkiaSkNativeSharedGLContextRef);
pub fn SkiaSkNativeSharedGLContextGetFBOID(aGLContext: SkiaSkNativeSharedGLContextRef) -> c_uint;
pub fn SkiaSkNativeSharedGLContextStealSurface(aGLContext: SkiaSkNativeSharedGLContextRef) -> SkiaGrGLSharedSurfaceRef;
pub fn SkiaSkNativeSharedGLContextGetGrContext(aGLContext: SkiaSkNativeSharedGLContextRef) -> SkiaGrContextRef;
pub fn SkiaSkNativeSharedGLContextMakeCurrent(aGLContext: SkiaSkNativeSharedGLContextRef);
pub fn SkiaSkNativeSharedGLContextFlush(aGLContext: SkiaSkNativeSharedGLContextRef);

}
