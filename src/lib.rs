/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate euclid;
extern crate gleam;
extern crate libc;

#[cfg(target_os="macos")]
extern crate cgl;
#[cfg(target_os="macos")]
extern crate io_surface;

#[cfg(target_os="linux")]
extern crate x11;
#[cfg(target_os="linux")]
extern crate glx;

#[cfg(target_os="android")]
extern crate egl;

#[cfg(any(target_os="linux", target_os="android"))]
extern crate freetype_sys;

#[cfg(any(target_os="linux", target_os="android"))]
extern crate fontconfig_sys;

pub use skia::{
    SkiaGrContextRef,
    SkiaGrGLInterfaceRef,
    SkiaGrGLCreateNativeInterface,
    SkiaGrGLInterfaceRetain,
    SkiaGrGLInterfaceRelease,
    SkiaGrGLInterfaceHasExtension,
    SkiaGrGLInterfaceGLVersionGreaterThanOrEqualTo,
    SkiaGrContextCreate,
    SkiaGrContextRetain,
    SkiaGrContextRelease,
};

pub mod gl_context;
pub mod gl_rasterization_context;
pub mod skia;

#[cfg(target_os="linux")]
pub mod gl_context_glx;
#[cfg(target_os="linux")]
pub mod gl_rasterization_context_glx;

#[cfg(target_os="macos")]
pub mod gl_context_cgl;
#[cfg(target_os="macos")]
pub mod gl_rasterization_context_cgl;

#[cfg(target_os="android")]
pub mod gl_context_android;
#[cfg(target_os="android")]
pub mod gl_rasterization_context_android;

#[cfg(target_os="windows")]
pub mod gl_context_wgl;
#[cfg(target_os="windows")]
pub mod gl_rasterization_context_wgl;
