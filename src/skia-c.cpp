/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "skia-c.h"

#include "gl/GrGLUtil.h"

extern "C" SkiaGrGLInterfaceRef
SkiaGrGLCreateNativeInterface() {
    return GrGLCreateNativeInterface();
}

extern "C" void
SkiaGrGLInterfaceRetain(SkiaGrGLInterfaceRef aGrGLInterface) {
    SkSafeRef(static_cast<const GrGLInterface*>(aGrGLInterface));
}

extern "C" void
SkiaGrGLInterfaceRelease(SkiaGrGLInterfaceRef aGrGLInterface) {
    SkSafeUnref(static_cast<const GrGLInterface*>(aGrGLInterface));
}

extern "C" bool
SkiaGrGLInterfaceHasExtension(SkiaGrGLInterfaceRef aGrGLInterface, const char extension[]) {
    return static_cast<const GrGLInterface*>(aGrGLInterface)->hasExtension(extension);
}

extern "C" bool
SkiaGrGLInterfaceGLVersionGreaterThanOrEqualTo(SkiaGrGLInterfaceRef aGrGLInterface, int32_t major, int32_t minor) {
    const GrGLubyte* versionUByte;
    GR_GL_CALL_RET(static_cast<const GrGLInterface*>(aGrGLInterface), versionUByte, GetString(GR_GL_VERSION));
    const char* version = reinterpret_cast<const char*>(versionUByte);

    GrGLVersion glVersion = GrGLGetVersionFromString(version);
    return GR_GL_INVALID_VER != glVersion && glVersion >= GR_GL_VER(major, minor);
}

extern "C" SkiaGrContextRef
SkiaGrContextCreate(SkiaGrGLInterfaceRef anInterface) {
    return GrContext::Create(kOpenGL_GrBackend, reinterpret_cast<GrBackendContext>(anInterface));
}

extern "C" void
SkiaGrContextRetain(SkiaGrContextRef aContext) {
    SkSafeRef(static_cast<GrContext*>(aContext));
}

extern "C" void
SkiaGrContextRelease(SkiaGrContextRef aContext) {
    SkSafeUnref(static_cast<GrContext*>(aContext));
}
