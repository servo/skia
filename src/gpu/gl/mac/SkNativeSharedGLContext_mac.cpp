
/*
 * Copyright 2013 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */
#include "gl/SkNativeSharedGLContext.h"
#include "gl/GrGLUtil.h"
#include <CoreFoundation/CoreFoundation.h>
#include <OpenGL/CGLIOSurface.h>

// This is defined here instead of in GrGLDefines.h, to make it easier
// to rebase our Skia changes onto later versions.
#define GR_GL_TEXTURE_RECTANGLE_ARB                         0x84F5

SkNativeSharedGLContext::SkNativeSharedGLContext(GrGLNativeContext& nativeContext)
    : fContext(NULL)
    , fPixelFormat(nativeContext.fPixelFormat)
    , fIOSurface(NULL)
    , fTextureID(0)
    , fGrContext(NULL)
    , fGL(NULL)
    , fFBO(0)
    , fDepthStencilBufferID(0) {
}

SkNativeSharedGLContext::~SkNativeSharedGLContext() {
    if (fGL) {
        SK_GL_NOERRCHECK(*this, DeleteFramebuffers(1, &fFBO));
        if (fTextureID)
            SK_GL_NOERRCHECK(*this, DeleteTextures(1, &fTextureID));
        SK_GL_NOERRCHECK(*this, DeleteRenderbuffers(1, &fDepthStencilBufferID));
    }
    SkSafeUnref(fGL);
    this->destroyGLContext();
    if (fGrContext) {
        fGrContext->unref();
    }
}

void SkNativeSharedGLContext::destroyGLContext() {
    if (NULL != fContext) {
        CGLReleaseContext(fContext);
    }
}

const GrGLInterface* SkNativeSharedGLContext::createGLContext(const int width, const int height) {
    SkASSERT(NULL == fContext);

    if (NULL == fPixelFormat) {
        SkDebugf("CGLGetPixelFormat failed.");
        return NULL;
    }

    CGLError err = CGLCreateContext(fPixelFormat, NULL, &fContext);

    if (NULL == fContext) {
        SkDebugf("CGLCreateContext failed with %s.", CGLErrorString(err));
        return NULL;
    }

    CGLSetCurrentContext(fContext);

    const GrGLInterface* interface = GrGLCreateNativeInterface();
    if (NULL == interface) {
        SkDebugf("Context could not create GL interface.\n");
        this->destroyGLContext();
        return NULL;
    }

    return interface;
}

bool SkNativeSharedGLContext::init(const int width, const int height) {
    if (fGL) {
        fGL->unref();
        this->destroyGLContext();
    }

    fGL = this->createGLContext(width, height);
    if (fGL) {
        const GrGLubyte* temp;

        SK_GL_RET(*this, temp, GetString(GR_GL_VERSION));
        const char* versionStr = reinterpret_cast<const char*>(temp);
        GrGLStandard standard = GrGLGetStandardInUseFromString(versionStr);

        if (!fGL->validate() || !fExtensions.init(standard, this->gl()->fFunctions.fGetString, this->gl()->fFunctions.fGetStringi, this->gl()->fFunctions.fGetIntegerv)) {
            fGL = NULL;
            this->destroyGLContext();
            return false;
        }

        // clear any existing GL erorrs
        GrGLenum error;
        do {
            SK_GL_RET(*this, error, GetError());
        } while (GR_GL_NO_ERROR != error);

        SK_GL(*this, Enable(GR_GL_TEXTURE_RECTANGLE_ARB));

        SK_GL(*this, GenFramebuffers(1, &fFBO));
        SK_GL(*this, BindFramebuffer(GR_GL_FRAMEBUFFER, fFBO));
        SK_GL(*this, GenTextures(1, &fTextureID));
        SK_GL(*this, BindTexture(GR_GL_TEXTURE_RECTANGLE_ARB, fTextureID));

        // Create the IOSurface and bind it to this texture.
        const void *surfacePropertyKeys[5] = {
            kIOSurfaceWidth,
            kIOSurfaceHeight,
            kIOSurfaceBytesPerRow,
            kIOSurfaceBytesPerElement,
            kIOSurfaceIsGlobal
        };
        int stride = width * 4, bpp = 4;
        const void *surfacePropertyValues[5] = {
            CFNumberCreate(kCFAllocatorDefault, kCFNumberIntType, &width),
            CFNumberCreate(kCFAllocatorDefault, kCFNumberIntType, &height),
            CFNumberCreate(kCFAllocatorDefault, kCFNumberIntType, &stride),
            CFNumberCreate(kCFAllocatorDefault, kCFNumberIntType, &bpp),
            kCFBooleanTrue
        };
        CFDictionaryRef surfaceProperties =
            CFDictionaryCreate(kCFAllocatorDefault,
                               surfacePropertyKeys,
                               surfacePropertyValues,
                               5,
                               &kCFTypeDictionaryKeyCallBacks,
                               &kCFTypeDictionaryValueCallBacks);
        fIOSurface = IOSurfaceCreate(surfaceProperties);
        CFRelease(surfaceProperties);

        CGLTexImageIOSurface2D(fContext,
                               GR_GL_TEXTURE_RECTANGLE_ARB,
                               GR_GL_RGBA,
                               width,
                               height,
                               GR_GL_BGRA,
                               0x8367, // UNSIGNED_INT_8_8_8_8_REV
                               fIOSurface,
                               0);

        SK_GL(*this, TexParameteri(GR_GL_TEXTURE_RECTANGLE_ARB, GR_GL_TEXTURE_WRAP_S, GR_GL_CLAMP_TO_EDGE));
        SK_GL(*this, TexParameteri(GR_GL_TEXTURE_RECTANGLE_ARB, GR_GL_TEXTURE_WRAP_T, GR_GL_CLAMP_TO_EDGE));
        SK_GL(*this, TexParameteri(GR_GL_TEXTURE_RECTANGLE_ARB, GR_GL_TEXTURE_MAG_FILTER, GR_GL_NEAREST));
        SK_GL(*this, TexParameteri(GR_GL_TEXTURE_RECTANGLE_ARB, GR_GL_TEXTURE_MIN_FILTER, GR_GL_NEAREST));

        SK_GL(*this, FramebufferTexture2D(GR_GL_FRAMEBUFFER,
                                          GR_GL_COLOR_ATTACHMENT0,
                                          GR_GL_TEXTURE_RECTANGLE_ARB,
                                          fTextureID, 0));
        SK_GL(*this, GenRenderbuffers(1, &fDepthStencilBufferID));
        SK_GL(*this, BindRenderbuffer(GR_GL_RENDERBUFFER, fDepthStencilBufferID));

        // Some drivers that support packed depth stencil will only succeed
        // in binding a packed format an FBO. However, we can't rely on packed
        // depth stencil being available.
        bool supportsPackedDepthStencil;
        if (kGLES_GrGLStandard == standard) {
            supportsPackedDepthStencil = this->hasExtension("GL_OES_packed_depth_stencil");
        } else {
            GrGLVersion version = GrGLGetVersionFromString(versionStr);
            supportsPackedDepthStencil = version >= GR_GL_VER(3,0) ||
                                         this->hasExtension("GL_EXT_packed_depth_stencil") ||
                                         this->hasExtension("GL_ARB_framebuffer_object");
        }

        if (supportsPackedDepthStencil) {
            // ES2 requires sized internal formats for RenderbufferStorage
            // On Desktop we let the driver decide.
            GrGLenum format = kGLES_GrGLStandard == standard ?
                                    GR_GL_DEPTH24_STENCIL8 :
                                    GR_GL_DEPTH_STENCIL;
            SK_GL(*this, RenderbufferStorage(GR_GL_RENDERBUFFER,
                                             format,
                                             width, height));
            SK_GL(*this, FramebufferRenderbuffer(GR_GL_FRAMEBUFFER,
                                                 GR_GL_DEPTH_ATTACHMENT,
                                                 GR_GL_RENDERBUFFER,
                                                 fDepthStencilBufferID));
        } else {
            GrGLenum format = kGLES_GrGLStandard == standard ?
                                    GR_GL_STENCIL_INDEX8 :
                                    GR_GL_STENCIL_INDEX;
            SK_GL(*this, RenderbufferStorage(GR_GL_RENDERBUFFER,
                                             format,
                                             width, height));
        }
        SK_GL(*this, FramebufferRenderbuffer(GR_GL_FRAMEBUFFER,
                                             GR_GL_STENCIL_ATTACHMENT,
                                             GR_GL_RENDERBUFFER,
                                             fDepthStencilBufferID));
        SK_GL(*this, Viewport(0, 0, width, height));
        SK_GL(*this, ClearStencil(0));
        SK_GL(*this, Clear(GR_GL_STENCIL_BUFFER_BIT));

        SK_GL_RET(*this, error, GetError());
        GrGLenum status;
        SK_GL_RET(*this, status, CheckFramebufferStatus(GR_GL_FRAMEBUFFER));

        if (GR_GL_FRAMEBUFFER_COMPLETE != status ||
            GR_GL_NO_ERROR != error) {
            fFBO = 0;
            fTextureID = 0;
            fDepthStencilBufferID = 0;
            fGL->unref();
            fGL = NULL;
            this->destroyGLContext();
            return false;
        } else {
            return true;
        }
    }
    return false;
}

GrContext *SkNativeSharedGLContext::getGrContext() {
    if (fGrContext) {
        return fGrContext;
    } else {
        GrBackendContext p3dctx = reinterpret_cast<GrBackendContext>(this->gl());
        fGrContext = GrContext::Create(kOpenGL_GrBackend, p3dctx);
        if (fGrContext == NULL) {
            return NULL;
        }
        // No need to AddRef; the GrContext is created with refcount = 1.
        return fGrContext;
    }
}

GrGLSharedSurface SkNativeSharedGLContext::stealSurface() {
    // Unbind the texture from the framebuffer.
    if (fGL && fFBO) {
        SK_GL(*this, BindFramebuffer(GR_GL_FRAMEBUFFER, fFBO));
        SK_GL(*this, FramebufferTexture2D(GR_GL_FRAMEBUFFER,
                                          GR_GL_COLOR_ATTACHMENT0,
                                          GR_GL_TEXTURE_RECTANGLE_ARB,
                                          0,
                                          0));
    }

    IOSurfaceRef surface = fIOSurface;
    fTextureID = 0;
    fIOSurface = NULL;
    return surface;
}

void SkNativeSharedGLContext::makeCurrent() const {
    CGLSetCurrentContext(fContext);
}

void SkNativeSharedGLContext::flush() const {
    this->makeCurrent();
    SK_GL(*this, Flush());
}

