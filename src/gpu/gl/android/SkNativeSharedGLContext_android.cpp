
/*
 * Copyright 2013 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */
#include "gl/SkNativeSharedGLContext.h"
#include "gl/GrGLUtil.h"

SkNativeSharedGLContext::SkNativeSharedGLContext(GrGLSharedContext sharedContext, void* extra)
    : fContext(EGL_NO_CONTEXT)
    , fDisplay(EGL_NO_DISPLAY)
    , fSurface(EGL_NO_SURFACE)
    , fGrContext(NULL)
    , fGL(NULL)
    , fSharedContext(sharedContext)
    , fFBO(0)
    , fTextureID(0)
    , fDepthStencilBufferID(0) {
}

SkNativeSharedGLContext::~SkNativeSharedGLContext() {
    if (fGL) {
        SK_GL_NOERRCHECK(*this, DeleteFramebuffers(1, &fFBO));
        SK_GL_NOERRCHECK(*this, DeleteTextures(1, &fTextureID));
        SK_GL_NOERRCHECK(*this, DeleteRenderbuffers(1, &fDepthStencilBufferID));
    }
    SkSafeUnref(fGL);
    this->destroyGLContext();
    if (fGrContext) {
        fGrContext->Release();
    }
}

void SkNativeSharedGLContext::destroyGLContext() {
    if (fDisplay) {
        eglMakeCurrent(fDisplay, 0, 0, 0);

        if (fContext) {
            eglDestroyContext(fDisplay, fContext);
            fContext = EGL_NO_CONTEXT;
        }

        if (fSurface) {
            eglDestroySurface(fDisplay, fSurface);
            fSurface = EGL_NO_SURFACE;
        }

        //TODO should we close the display?
        fDisplay = EGL_NO_DISPLAY;
    }
}

const GrGLInterface* SkNativeSharedGLContext::createGLContext() {
    fDisplay = eglGetDisplay(EGL_DEFAULT_DISPLAY);

    EGLint majorVersion;
    EGLint minorVersion;
    eglInitialize(fDisplay, &majorVersion, &minorVersion);

    EGLint numConfigs;
    static const EGLint configAttribs[] = {
        EGL_SURFACE_TYPE, EGL_PBUFFER_BIT,
        EGL_RENDERABLE_TYPE, EGL_OPENGL_ES2_BIT,
        EGL_RED_SIZE, 8,
        EGL_GREEN_SIZE, 8,
        EGL_BLUE_SIZE, 8,
        EGL_ALPHA_SIZE, 8,
        EGL_NONE
    };

    EGLConfig surfaceConfig;
    eglChooseConfig(fDisplay, configAttribs, &surfaceConfig, 1, &numConfigs);

    static const EGLint contextAttribs[] = {
        EGL_CONTEXT_CLIENT_VERSION, 2,
        EGL_NONE
    };
    fContext = eglCreateContext(fDisplay, surfaceConfig, fSharedContext, contextAttribs);


    static const EGLint surfaceAttribs[] = {
        EGL_WIDTH, 1,
        EGL_HEIGHT, 1,
        EGL_NONE
    };
    fSurface = eglCreatePbufferSurface(fDisplay, surfaceConfig, surfaceAttribs);

    eglMakeCurrent(fDisplay, fSurface, fSurface, fContext);

    const GrGLInterface* interface = GrGLCreateNativeInterface();
    if (!interface) {
        SkDebugf("Failed to create gl interface");
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

    fGL = this->createGLContext();
    if (fGL) {
        const GrGLubyte* temp;

        GrGLBinding bindingInUse = GrGLGetBindingInUse(this->gl());

        if (!fGL->validate(bindingInUse) || !fExtensions.init(bindingInUse, fGL)) {
            fGL = NULL;
            this->destroyGLContext();
            return false;
        }

        SK_GL_RET(*this, temp, GetString(GR_GL_VERSION));
        const char* versionStr = reinterpret_cast<const char*>(temp);
        GrGLVersion version = GrGLGetVersionFromString(versionStr);

        // clear any existing GL erorrs
        GrGLenum error;
        do {
            SK_GL_RET(*this, error, GetError());
        } while (GR_GL_NO_ERROR != error);

        SK_GL(*this, GenFramebuffers(1, &fFBO));
        SK_GL(*this, BindFramebuffer(GR_GL_FRAMEBUFFER, fFBO));
        SK_GL(*this, GenTextures(1, &fTextureID));
        SK_GL(*this, BindTexture(GR_GL_TEXTURE_2D, fTextureID));
        SK_GL(*this, TexImage2D(GR_GL_TEXTURE_2D, 0,
                                GR_GL_RGBA,
                                width, height, 0,
                                GR_GL_RGBA, GR_GL_UNSIGNED_BYTE, 
                                NULL));
        SK_GL(*this, TexParameteri(GR_GL_TEXTURE_2D, GR_GL_TEXTURE_WRAP_S, GR_GL_CLAMP_TO_EDGE));
        SK_GL(*this, TexParameteri(GR_GL_TEXTURE_2D, GR_GL_TEXTURE_WRAP_T, GR_GL_CLAMP_TO_EDGE));
        SK_GL(*this, TexParameteri(GR_GL_TEXTURE_2D, GR_GL_TEXTURE_MAG_FILTER, GR_GL_NEAREST));
        SK_GL(*this, TexParameteri(GR_GL_TEXTURE_2D, GR_GL_TEXTURE_MIN_FILTER, GR_GL_NEAREST));
        SK_GL(*this, FramebufferTexture2D(GR_GL_FRAMEBUFFER,
                                          GR_GL_COLOR_ATTACHMENT0,
                                          GR_GL_TEXTURE_2D,
                                          fTextureID, 0));
        SK_GL(*this, GenRenderbuffers(1, &fDepthStencilBufferID));
        SK_GL(*this, BindRenderbuffer(GR_GL_RENDERBUFFER, fDepthStencilBufferID));

        // Some drivers that support packed depth stencil will only succeed
        // in binding a packed format an FBO. However, we can't rely on packed
        // depth stencil being available.
        bool supportsPackedDepthStencil;
        if (kES2_GrGLBinding == bindingInUse) {
            supportsPackedDepthStencil = this->hasExtension("GL_OES_packed_depth_stencil");
        } else {
            supportsPackedDepthStencil = version >= GR_GL_VER(3,0) ||
                                         this->hasExtension("GL_EXT_packed_depth_stencil") ||
                                         this->hasExtension("GL_ARB_framebuffer_object");
        }

        if (supportsPackedDepthStencil) {
            // ES2 requires sized internal formats for RenderbufferStorage
            // On Desktop we let the driver decide.
            GrGLenum format = kES2_GrGLBinding == bindingInUse ?
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
            GrGLenum format = kES2_GrGLBinding == bindingInUse ?
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

GrGLuint SkNativeSharedGLContext::stealTextureID() {
    // Unbind the texture from the framebuffer.
    if (fGL && fFBO) {
        SK_GL(*this, BindFramebuffer(GR_GL_FRAMEBUFFER, fFBO));
        SK_GL(*this, FramebufferTexture2D(GR_GL_FRAMEBUFFER,
                    GR_GL_COLOR_ATTACHMENT0,
                    GR_GL_TEXTURE_2D,
                    0,
                    0));
    }

    GrGLuint textureID = fTextureID;
    fTextureID = 0;
    return textureID;
}

void SkNativeSharedGLContext::makeCurrent() const {
    if (!eglMakeCurrent(fDisplay, fSurface, fSurface, fContext)) {
        SkDebugf("Could not set the context.\n");
    }
}

void SkNativeSharedGLContext::flush() const {
    this->makeCurrent();
    SK_GL(*this, Flush());
}
