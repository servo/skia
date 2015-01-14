/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "skia-c.h"

extern "C" SkiaSkNativeSharedGLContextRef
SkiaSkNativeSharedGLContextCreate(SkiaGrGLNativeContextRef aNativeContext, int32_t aWidth, int32_t aHeight) {
    GrGLNativeContext* nativeContext = reinterpret_cast<GrGLNativeContext*>(aNativeContext);
    SkNativeSharedGLContext *sharedGLContext = new SkNativeSharedGLContext(*nativeContext);
    if (sharedGLContext == NULL) {
        return NULL;
    }
    if (!sharedGLContext->init(aWidth, aHeight)) {
        return NULL;
    }
    return sharedGLContext;
}

extern "C" void
SkiaSkNativeSharedGLContextRetain(SkiaSkNativeSharedGLContextRef aGLContext) {
    SkNativeSharedGLContext *sharedGLContext = static_cast<SkNativeSharedGLContext*>(aGLContext);
    sharedGLContext->ref();
}

extern "C" void
SkiaSkNativeSharedGLContextRelease(SkiaSkNativeSharedGLContextRef aGLContext) {
    SkNativeSharedGLContext *sharedGLContext = static_cast<SkNativeSharedGLContext*>(aGLContext);
    sharedGLContext->unref();
}

extern "C" unsigned int
SkiaSkNativeSharedGLContextGetFBOID(SkiaSkNativeSharedGLContextRef aGLContext) {
   SkNativeSharedGLContext *sharedGLContext = static_cast<SkNativeSharedGLContext*>(aGLContext);
   return sharedGLContext->getFBOID();
}

extern "C" SkiaGrGLSharedSurfaceRef
SkiaSkNativeSharedGLContextStealSurface(SkiaSkNativeSharedGLContextRef aGLContext) {
    SkNativeSharedGLContext *sharedGLContext = static_cast<SkNativeSharedGLContext*>(aGLContext);
    return reinterpret_cast<void*>(sharedGLContext->stealSurface());
}

extern "C" SkiaGrContextRef
SkiaSkNativeSharedGLContextGetGrContext(SkiaSkNativeSharedGLContextRef aGLContext) {
    SkNativeSharedGLContext *sharedGLContext = static_cast<SkNativeSharedGLContext*>(aGLContext);
    return sharedGLContext->getGrContext();
}

extern "C" void
SkiaSkNativeSharedGLContextMakeCurrent(SkiaSkNativeSharedGLContextRef aGLContext) {
    SkNativeSharedGLContext *sharedGLContext = static_cast<SkNativeSharedGLContext*>(aGLContext);
    sharedGLContext->makeCurrent();
}

extern "C" void
SkiaSkNativeSharedGLContextFlush(SkiaSkNativeSharedGLContextRef aGLContext) {
    SkNativeSharedGLContext *sharedGLContext = static_cast<SkNativeSharedGLContext*>(aGLContext);
    sharedGLContext->flush();
}

