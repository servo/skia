/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "GrContext.h"
#include "gl/SkNativeSharedGLContext.h"

#ifndef SKIA_C_DEFINED
#define SKIA_C_DEFINED

typedef void* SkiaSkNativeSharedGLContextRef;
typedef void* SkiaGrContextRef;
typedef void* SkiaGrGLSharedSurfaceRef;
typedef GrGLNativeContext* SkiaGrGLNativeContextRef;

#ifdef __cplusplus
extern "C" {
#endif

SkiaSkNativeSharedGLContextRef SkiaSkNativeSharedGLContextCreate(SkiaGrGLNativeContextRef aNativeContext, int32_t aWidth, int32_t aHeight);
void SkiaSkNativeSharedGLContextRetain(SkiaSkNativeSharedGLContextRef aGLContext);
void SkiaSkNativeSharedGLContextRelease(SkiaSkNativeSharedGLContextRef aGLContext);
unsigned int SkiaSkNativeSharedGLContextGetFBOID(SkiaSkNativeSharedGLContextRef aGLContext);
SkiaGrGLSharedSurfaceRef SkiaSkNativeSharedGLContextStealSurface(SkiaSkNativeSharedGLContextRef aGLContext);
SkiaGrContextRef SkiaSkNativeSharedGLContextGetGrContext(SkiaSkNativeSharedGLContextRef aGLContext);
void SkiaSkNativeSharedGLContextMakeCurrent(SkiaSkNativeSharedGLContextRef aGLContext);
void SkiaSkNativeSharedGLContextFlush(SkiaSkNativeSharedGLContextRef aGLContext);

#ifdef __cplusplus
}
#endif

#endif /* SKIA_C_DEFINED */
