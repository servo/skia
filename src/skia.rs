// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::*;

pub type SkiaGrContextRef = *mut c_void;
pub type SkiaGrGLInterfaceRef = *const c_void;

extern {

pub fn SkiaGrGLCreateNativeInterface() -> SkiaGrGLInterfaceRef;
pub fn SkiaGrGLInterfaceRetain(anInterface: SkiaGrGLInterfaceRef);
pub fn SkiaGrGLInterfaceRelease(anInterface: SkiaGrGLInterfaceRef);
pub fn SkiaGrGLInterfaceHasExtension(anInterface: SkiaGrGLInterfaceRef, extension: *const c_char) -> bool;
pub fn SkiaGrGLInterfaceGLVersionGreaterThanOrEqualTo(anInterface: SkiaGrGLInterfaceRef, major: i32, minor: i32) -> bool;

pub fn SkiaGrContextCreate(anInterface: SkiaGrGLInterfaceRef) -> SkiaGrContextRef;
pub fn SkiaGrContextRetain(aContext: SkiaGrContextRef);
pub fn SkiaGrContextRelease(aContext: SkiaGrContextRef);

}
