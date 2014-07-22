
/*
 * Copyright 2013 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */
#include "gl/SkNativeSharedGLContext.h"
#include "gl/GrGLUtil.h"

SkNativeSharedGLContext::SkNativeSharedGLContext(GrGLNativeContext& nativeContext)
    : fContext(NULL)
    , fDisplay(nativeContext.fDisplay)
    , fVisualInfo(nativeContext.fVisualInfo)
    , fGrContext(NULL)
    , fGL(NULL)
    , fFBO(0)
    , fTextureID(0)
    , fDepthStencilBufferID(0)
    , fPixmap(0)
    , fGlxPixmap(0) {
}

SkNativeSharedGLContext::~SkNativeSharedGLContext() {
    if (fGlxPixmap) {
        destroyGLResources();
    }
    if (fPixmap) {
        XFreePixmap(fDisplay, fPixmap);
    }
    if (fGrContext) {
        fGrContext->Release();
    }
    SkSafeUnref(fGL);
    this->destroyGLContext();
}

void SkNativeSharedGLContext::destroyGLResources() {
    if (fGL) {
        // We need this thread to grab the GLX context before we can make
        // OpenGL calls.  But glXMakeCurrent() will flush the old context,
        // which might have been uninitialized.  Calling with (None, NULL)
        // first solves this problem (somehow).
        glXMakeCurrent(fDisplay, None, NULL);
        glXMakeCurrent(fDisplay, fGlxPixmap, fContext);

        SK_GL_NOERRCHECK(*this, DeleteFramebuffers(1, &fFBO));
        SK_GL_NOERRCHECK(*this, DeleteTextures(1, &fTextureID));
        SK_GL_NOERRCHECK(*this, DeleteRenderbuffers(1, &fDepthStencilBufferID));
    }

    glXDestroyGLXPixmap(fDisplay, fGlxPixmap);
    fGlxPixmap = 0;
}

void SkNativeSharedGLContext::destroyGLContext() {
    glXMakeCurrent(fDisplay, None, NULL);
    if (NULL != fContext) {
        glXDestroyContext(fDisplay, fContext);
    }
}

const GrGLInterface* SkNativeSharedGLContext::createGLContext(const int width, const int height) {
    SkASSERT(NULL == fContext);

    // Create the base pixmap and GLX pixmap.
    //
    // TODO(pcwalton): This is inefficient. It would be better to have the caller pass in the
    // pixmap.
    int screen = DefaultScreen(fDisplay);
    Drawable root = RootWindow(fDisplay, screen);
    fPixmap = XCreatePixmap(fDisplay, root, width, height, fVisualInfo->depth);
    fGlxPixmap = glXCreateGLXPixmap(fDisplay, fVisualInfo, fPixmap);

    fContext = glXCreateContext(fDisplay, fVisualInfo, NULL, true);
    if (NULL == fContext) {
        SkDebugf("glXCreateContext failed with %d.", fContext);
        return NULL;
    }

    glXMakeCurrent(fDisplay, fGlxPixmap, fContext);

    const GrGLInterface* interface = GrGLCreateNativeInterface();
    if (NULL == interface) {
        SkDebugf("Context could not create GL interface.\n");
        this->destroyGLContext();
        return NULL;
    }

    return interface;
}

bool SkNativeSharedGLContext::init(int width, int height) {
    width = width <= 0 ? 1 : width;
    height = height <= 0 ? 1 : height;

    if (fGL) {
        fGL->unref();
        this->destroyGLContext();
    }

    fGL = this->createGLContext(width, height);
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
        SK_GL(*this, TexParameteri(GR_GL_TEXTURE_2D, GR_GL_TEXTURE_MAG_FILTER, GR_GL_LINEAR));
        SK_GL(*this, TexParameteri(GR_GL_TEXTURE_2D, GR_GL_TEXTURE_MIN_FILTER, GR_GL_LINEAR));
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
        if (GR_GL_FRAMEBUFFER_COMPLETE != status || GR_GL_NO_ERROR != error) {
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

void SkNativeSharedGLContext::makeCurrent() const {
    glXMakeCurrent(fDisplay, fGlxPixmap, fContext);
}

void SkNativeSharedGLContext::flush() const {
    this->makeCurrent();
    SK_GL(*this, Flush());
}

GrGLSharedSurface SkNativeSharedGLContext::stealSurface() {
    // Render the texture to the default framebuffer.
    int viewport[4];
    SK_GL(*this, GetIntegerv(GR_GL_VIEWPORT, viewport));
    int width = viewport[2], height = viewport[3];
    SK_GL(*this, BindFramebuffer(GR_GL_READ_FRAMEBUFFER, fFBO));
    SK_GL(*this, BindFramebuffer(GR_GL_DRAW_FRAMEBUFFER, 0));
    SK_GL(*this, BlitFramebuffer(0,
                                 0,
                                 width,
                                 height,
                                 0,
                                 0,
                                 width,
                                 height,
                                 GR_GL_COLOR_BUFFER_BIT,
                                 GR_GL_NEAREST));
    SK_GL(*this, Flush());
    SK_GL(*this, BindFramebuffer(GR_GL_FRAMEBUFFER, 0));

    Pixmap pixmap = fPixmap;
    destroyGLResources();
    fPixmap = 0;
    return pixmap;
}

