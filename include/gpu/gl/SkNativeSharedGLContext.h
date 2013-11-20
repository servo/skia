
/*
 * Copyright 2013 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */
#ifndef SkNativeSharedGLContext_DEFINED
#define SkNativeSharedGLContext_DEFINED

#include "SkGLContextHelper.h"
#include "GrContext.h"

#if defined(SK_BUILD_FOR_MAC)
    #include <OpenGL/OpenGL.h>
    #include <IOSurface/IOSurface.h>
#elif defined(SK_BUILD_FOR_ANDROID) || defined(SK_BUILD_FOR_NACL)
    #include <GLES2/gl2.h>
    #include <EGL/egl.h>
    #include <EGL/eglext.h>
#elif defined(SK_BUILD_FOR_UNIX)
    #include <X11/Xlib.h>
    #include <GL/glx.h>
#elif defined(SK_BUILD_FOR_WIN32)
    #include <Windows.h>
    #include <GL/GL.h>
#endif

#if defined(SK_BUILD_FOR_MAC)
struct GrGLNativeContext {
    CGLPixelFormatObj fPixelFormat;
};
typedef CGLContextObj GrGLSharedContext;
typedef IOSurfaceRef GrGLSharedSurface;
#elif defined(SK_BUILD_FOR_ANDROID)
struct GrGLNativeContext {
    EGLDisplay *fDisplay;
};
typedef EGLContext GrGLSharedContext;
typedef EGLImageKHR GrGLSharedSurface;
#elif defined(SK_BUILD_FOR_UNIX)
struct GrGLNativeContext {
    Display *fDisplay;
    XVisualInfo *fVisualInfo;
};
typedef GLXContext GrGLSharedContext;
typedef Pixmap GrGLSharedSurface;
#else
#error "No shared contexts on this platform."
#endif

class SkNativeSharedGLContext : public SkRefCnt {
public:
    explicit SkNativeSharedGLContext(GrGLNativeContext& nativeContext);
    virtual ~SkNativeSharedGLContext();

    virtual bool init(int width, int height);
    virtual unsigned int getFBOID() const { return fFBO; }
    virtual const GrGLInterface *gl() const { return fGL; }
    virtual GrContext *getGrContext();
    virtual void makeCurrent() const;
    virtual void flush() const;

    virtual bool hasExtension(const char* extensionName) const {
        GrAssert(NULL != fGL);
        return fExtensions.has(extensionName);
    }

    static GrGLSharedContext GetCurrent() {
        #if defined(SK_BUILD_FOR_MAC)
        return CGLGetCurrentContext();
        #elif defined(SK_BUILD_FOR_ANDROID) || defined(SK_BUILD_FOR_NACL)
        return eglGetCurrentContext();
        #elif defined(SK_BUILD_FOR_UNIX)
        return glXGetCurrentContext();
        #endif
    }

    // Releases and returns the native surface. After this call, the caller is
    // responsible for destroying it.
    //
    // If this call is called more than once, invocations after the first will
    // return zero and do nothing.
    //
    // Any rendering that takes place after this call will result in rendering
    // to a framebuffer bound to no attachment at all (i.e. an incomplete
    // framebuffer), which will result in OpenGL errors.
    GrGLSharedSurface stealSurface();

protected:
    virtual const GrGLInterface *createGLContext(const int width, const int height);
    virtual void destroyGLContext();

private:
#if defined(SK_BUILD_FOR_MAC)
    CGLContextObj fContext;
    CGLPixelFormatObj fPixelFormat;
    IOSurfaceRef fIOSurface;
#elif defined(SK_BUILD_FOR_ANDROID)
    EGLContext fContext;
    EGLDisplay fDisplay;
    EGLSurface fSurface;
    EGLImageKHR fEGLImage;
#elif defined(SK_BUILD_FOR_UNIX)
    GLXContext fContext;
    Display* fDisplay;
    XVisualInfo *fVisualInfo;
    Pixmap fPixmap;
    GLXPixmap fGlxPixmap;

    void destroyGLResources();
#else
#error "No shared contexts on this platform."
#endif

    GrGLExtensions fExtensions;
    GrGLuint fTextureID;
    GrGLuint fFBO;
    GrGLuint fDepthStencilBufferID;

    const GrGLInterface* fGL;
    GrContext* fGrContext;

    typedef SkRefCnt INHERITED;
};

#endif
