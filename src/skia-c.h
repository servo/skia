/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "GrContext.h"

#ifndef SKIA_C_DEFINED
#define SKIA_C_DEFINED

typedef void* SkiaGrContextRef;
typedef const void* SkiaGrGLInterfaceRef;

#ifdef __cplusplus
extern "C" {
#endif

SkiaGrGLInterfaceRef SkiaGrGLCreateNativeInterface();
void SkiaGrGLInterfaceRetain(SkiaGrGLInterfaceRef);
void SkiaGrGLInterfaceRelease(SkiaGrGLInterfaceRef);
bool SkiaGrGLInterfaceHasExtension(SkiaGrGLInterfaceRef, const char extension[]);
bool SkiaGrGLInterfaceGLVersionGreaterThanOrEqualTo(SkiaGrGLInterfaceRef, int32_t major, int32_t minor);

SkiaGrContextRef SkiaGrContextCreate(SkiaGrGLInterfaceRef);
void SkiaGrContextRetain(SkiaGrContextRef);
void SkiaGrContextRelease(SkiaGrContextRef);

#ifdef __cplusplus
}
#endif

#endif /* SKIA_C_DEFINED */
