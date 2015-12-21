// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! TODO: use bindgen and modify skia-c so we can avoid allocations.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

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

pub type Surface   = *mut c_void;
pub type Image     = *mut c_void;
pub type Data      = *mut c_void;
pub type Path      = *mut c_void;
pub type Paint     = *mut c_void;
pub type GrContext = *mut c_void;

pub type Color = libc::uint32_t;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum ColorType {
    Unknown = 0,
    Alpha8 = 1,
    Rgb565 = 2,
    Argb4444 = 3,
    Rgba8888 = 4,
    Bgra8888 = 5,
    Index8 = 6,
    Gray8 = 7,
}

#[cfg(target_endian = "little")]
pub const NATIVE_COLOR_TYPE: ColorType = ColorType::Rgba8888;
#[cfg(target_endian = "big")]
pub const NATIVE_COLOR_TYPE: ColorType = ColorType::Bgra8888;

impl Default for ColorType {
    fn default() -> ColorType {
        NATIVE_COLOR_TYPE
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum AlphaType {
    Unknown = 0,
    Opaque = 1,
    Premul = 2,
    Unpremul = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum ColorProfile {
    Linear = 0,
    Srgb = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Error {
    Ok = 0,
    Other = 1,
    BadArg = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum PointMode {
    Points = 0,
    Lines = 1,
    Polygon = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum PathFillType {
    Winding = 0,
    EvenOdd = 1,
    InverseWinding = 2,
    InverseEvenOdd = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum SrcRectConstraint {
    Strict = 0,
    Fast = 1,
}
impl Default for SrcRectConstraint {
    fn default() -> SrcRectConstraint {
        SrcRectConstraint::Strict
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum CacheManagement {
    Unbudgeted = 0,
    Budgeted = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ImageInfo {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub color_type: ColorType,
    pub alpha_type: AlphaType,
    pub color_profile: ColorProfile,
}
impl Default for ImageInfo {
    fn default() -> ImageInfo {
        unsafe { sk_default_image_info() }
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rect {
    pub left: libc::c_float,
    pub top: libc::c_float,
    pub right: libc::c_float,
    pub bottom: libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct IRect {
    pub left: libc::int32_t,
    pub top: libc::int32_t,
    pub right: libc::int32_t,
    pub bottom: libc::int32_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Size {
    pub width: libc::c_float,
    pub height: libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ISize {
    pub width: libc::int32_t,
    pub height: libc::int32_t
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
    pub x: libc::c_float,
    pub y: libc::c_float,
}

extern {
    pub fn sk_default_image_info() -> ImageInfo;
    pub fn sk_color_from_argb(a: libc::uint8_t, r: libc::uint8_t,
                              g: libc::uint8_t, b: libc::uint8_t) -> Color;
    pub fn sk_color_get_a(c: Color) -> libc::uint8_t;
    pub fn sk_color_get_r(c: Color) -> libc::uint8_t;
    pub fn sk_color_get_g(c: Color) -> libc::uint8_t;
    pub fn sk_color_get_b(c: Color) -> libc::uint8_t;

    pub fn sk_surface_ref(s: Surface) -> Error;
    pub fn sk_surface_unref(s: Surface) -> Error;
    pub fn sk_image_ref(i: Image) -> Error;
    pub fn sk_image_unref(i: Image) -> Error;
    pub fn sk_data_ref(d: Data) -> Error;
    pub fn sk_data_unref(d: Data) -> Error;
    pub fn sk_paint_ref(p: Paint) -> Error;
    pub fn sk_paint_unref(p: Paint) -> Error;

    pub fn sk_image_get_size(i: Image, size: *mut ISize) -> Error;

    pub fn sk_new_paint() -> Paint;
    pub fn sk_new_paint_copy(p: Paint) -> Paint;
    pub fn sk_paint_reset(p: Paint) -> Error;
    pub fn sk_paint_get_color(p: Paint) -> Color;
    pub fn sk_paint_set_color(p: Paint, c: Color);

    pub fn sk_new_render_target_surface(gr: GrContext,
                                        budget: CacheManagement,
                                        info: ImageInfo) -> Surface;

    pub fn sk_new_image_snapshot(s: Surface) -> Image;
    pub fn sk_surface_get_size(s: Surface, size: *mut ISize) -> Error;

    pub fn sk_flush(s: Surface) -> Error;
    pub fn sk_save(s: Surface, save_count: *mut libc::c_int) -> Error;
    pub fn sk_restore(s: Surface) -> Error;
    pub fn sk_restore_to_count(s: Surface, count: libc::c_int) -> Error;

    pub fn sk_translate(s: Surface, dx: libc::c_float, dy: libc::c_float) -> Error;
    pub fn sk_scale(s: Surface, sx: libc::c_float, sy: libc::c_float) -> Error;
    pub fn sk_rotate(s: Surface, degrees: libc::c_float) -> Error;

    pub fn sk_clip_rect(s: Surface, rect: Rect) -> Error;
    pub fn sk_draw_paint(s: Surface, paint: Paint) -> Error;
    pub fn sk_draw_points(s: Surface, paint: Paint, mode: PointMode,
                          points: *const Point, count: libc::size_t) -> Error;
    pub fn sk_draw_path(s: Surface, paint: Paint, path: Path) -> Error;
    //pub fn sk_draw_image_rect(s: Surface, paint: Paint, src: *const Rect,
    //                          dest: Rect, img: Image, constraint: SrcRectConstraint) -> Error;
    pub fn sk_draw_text(s: Surface, paint: Paint, pos: Point, text: *const libc::c_void,
                        len: libc::size_t) -> Error;

    pub fn sk_new_path() -> Path;
    pub fn sk_clone_path(p: Path) -> Path;
    pub fn sk_del_path(p: Path);
    pub fn sk_path_reset(p: Path) -> Error;
    pub fn sk_path_set_fill_type(p: Path, ft: PathFillType) -> Error;
    pub fn sk_path_get_fill_type(p: Path) -> PathFillType;
    pub fn sk_path_line_to(p: Path, point: Point, relative: bool) -> Error;
    pub fn sk_path_quad_to(p: Path, p1: Point, p2: Point, relative: bool) -> Error;
    pub fn sk_path_cubic_to(p: Path, p1: Point, p2: Point, p3: Point, relative: bool) -> Error;
    pub fn sk_path_close(p: Path);
    pub fn sk_path_count_points(p: Path) -> libc::c_int;
    pub fn sk_path_get_point(p: Path, index: libc::c_int) -> Point;
}
