/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// Some crumminess to make sure we link correctly

#[cfg(target_os = "linux")]
#[link(name = "skia", kind = "static")]
#[link(name = "stdc++")]
#[link(name = "freetype")]
#[link(name = "fontconfig")]
#[link(name = "bz2")]
#[link(name = "GL")]
extern { }

#[cfg(target_os = "android")]
#[link(name = "skia", kind = "static")]
#[link(name = "stdc++")]
#[link(name = "fontconfig")]
#[link(name = "EGL")]
#[link(name = "GLESv2")]
extern { }

#[cfg(target_os = "macos")]
#[link(name = "skia", kind = "static")]
#[link(name = "stdc++")]
#[link(name = "IOSurface", kind = "framework")]
#[link(name = "OpenGL", kind = "framework")]
#[link(name = "ApplicationServices", kind = "framework")]
extern { }
