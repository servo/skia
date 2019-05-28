/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub use crate::skia::{
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

pub mod skia;

#[cfg(feature = "gl_backend")]
pub mod gl_context;

#[cfg(feature = "gl_backend")]
pub mod gl_rasterization_context;

#[cfg(all(feature = "gl_backend", target_os="linux"))]
pub mod gl_context_glx;
#[cfg(all(feature = "gl_backend", target_os="linux"))]
pub mod gl_rasterization_context_glx;

#[cfg(all(feature = "gl_backend", target_os="macos"))]
pub mod gl_context_cgl;
#[cfg(all(feature = "gl_backend", target_os="macos"))]
pub mod gl_rasterization_context_cgl;

#[cfg(all(feature = "gl_backend", target_os="android"))]
pub mod gl_context_android;
#[cfg(all(feature = "gl_backend", target_os="android"))]
pub mod gl_rasterization_context_android;

#[cfg(all(feature = "gl_backend", target_os="windows"))]
pub mod gl_context_wgl;
#[cfg(all(feature = "gl_backend", target_os="windows"))]
pub mod gl_rasterization_context_wgl;
