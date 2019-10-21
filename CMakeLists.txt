project(skia)
cmake_minimum_required(VERSION 2.8)
enable_language(CXX ASM)

function(set_prefix var prefix)
  string(REGEX REPLACE "(^|;)([^;]+)" "\\1${prefix}\\2" tmp "${ARGN}")
  set(${var} "${tmp}" PARENT_SCOPE)
endfunction()

# Use ccache if we're told to.
if(DEFINED ENV{CCACHE})
  set_property(GLOBAL PROPERTY RULE_LAUNCH_COMPILE $ENV{CCACHE})
endif()

if("$ENV{CFG_ENABLE_DEBUG_SKIA}" STREQUAL "1")
  add_definitions(
    -DDSK_DEBUG
    -DGR_DEBUG=1
    -DGR_GL_LOG_CALLS=1
    -DGR_GL_LOG_CALLS_START=1
  )
  if(MSVC)
    add_definitions(-Zi)
  else()
    add_definitions(-g)
  endif()
else()
  add_definitions(
    -DSK_RELEASE
    -DGR_RELEASE=1
  )
  if(MSVC)
    add_definitions(-Os)
  else()
    add_definitions(-O3)
  endif()
endif()

add_definitions(
  -DSK_A32_SHIFT=24
  -DSK_R32_SHIFT=16
  -DSK_G32_SHIFT=8
  -DSK_B32_SHIFT=0
  -DGR_GL_USE_NEW_SHADER_SOURCE_SIGNATURE=1
)

if(CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_definitions(-Wno-c++11-extensions)
endif()

if(NOT WIN32)
  add_definitions(-fPIC)
endif()

if(NOT APPLE AND NOT WIN32)
  if(NOT "$ENV{DEP_FREETYPE_OUTDIR}" STREQUAL "")
    include_directories($ENV{DEP_FREETYPE_OUTDIR}/include/freetype2)
    message(STATUS "Using DEP_FREETYPE_OUTDIR/include/freetype2: " $ENV{DEP_FREETYPE_OUTDIR})
  else()
    # We need to work with older cmake -- pkg_check_modules was added in
    # a version of cmake newer than what Ubuntu 14.04 LTS ships with.
    #pkg_check_modules(FREETYPE REQUIRED freetype2)
    EXEC_PROGRAM(pkg-config ARGS --cflags freetype2 OUTPUT_VARIABLE FREETYPE_CFLAGS RESULT_VARIABLE FREETYPE_NOT_FOUND)
    if(FREETYPE_NOT_FOUND)
      message(FATAL_ERROR "Freetype is required by Skia for this build, but it was not built by the freetype crate nor found by pkg-config")
    endif()
    EXEC_PROGRAM(pkg-config ARGS --libs freetype2 OUTPUT_VARIABLE FREETYPE_CLDFLAGS)
    message(STATUS "Using FREETYPE_CFLAGS: " ${FREETYPE_CFLAGS})
    message(STATUS "Using FREETYPE_CLDFLAGS: " ${FREETYPE_CLDFLAGS})
    add_definitions(${FREETYPE_CFLAGS})
    link_libraries(${FREETYPE_CLDFLAGS})
  endif()
endif()

if(NOT "$ENV{DEP_EXPAT_OUTDIR}" STREQUAL "")
  include_directories($ENV{DEP_EXPAT_OUTDIR}/include)
endif()

include_directories(
  include/config
  include/core
  include/effects
  include/gpu
  include/images
  include/pathops
  include/pipe
  include/ports
  include/utils
  src/core
  src/image
  src/opts
  src/ports
  src/sfnt
  src/utils
  third_party/etc1
  third_party/ktx
  src/gpu
  )

set_prefix(SKIA_CORE_SRC src/core/
  SkAAClip.cpp
  SkAdvancedTypefaceMetrics.cpp
  SkAlphaRuns.cpp
  SkAnnotation.cpp
  SkBBHFactory.cpp
  SkBBoxHierarchyRecord.cpp
  SkBBoxRecord.cpp
  SkBitmap.cpp
  SkBitmapDevice.cpp
  SkBitmapFilter.cpp
  SkBitmapHeap.cpp
  SkBitmapProcShader.cpp
  SkBitmapProcState.cpp
  SkBitmapProcState_matrixProcs.cpp
  SkBitmapScaler.cpp
  SkBitmap_scroll.cpp
  SkBlitMask_D32.cpp
  SkBlitRow_D16.cpp
  SkBlitRow_D32.cpp
  SkBlitter_A8.cpp
  SkBlitter_ARGB32.cpp
  SkBlitter.cpp
  SkBlitter_RGB16.cpp
  SkBlitter_Sprite.cpp
  SkBuffer.cpp
  SkCanvas.cpp
  SkChunkAlloc.cpp
  SkClipStack.cpp
  SkColor.cpp
  SkColorFilter.cpp
  SkColorTable.cpp
  SkComposeShader.cpp
  SkConfig8888.cpp
  SkConvolver.cpp
  SkCubicClipper.cpp
  SkData.cpp
  SkDataTable.cpp
  SkDebug.cpp
  SkDeque.cpp
  SkDevice.cpp
  SkDeviceLooper.cpp
  SkDeviceProfile.cpp
  SkDistanceFieldGen.cpp
  SkDither.cpp
  SkDraw.cpp
  SkDrawLooper.cpp
  SkEdgeBuilder.cpp
  SkEdgeClipper.cpp
  SkEdge.cpp
  SkError.cpp
  SkFilterProc.cpp
  SkFilterShader.cpp
  SkFlate.cpp
  SkFlattenable.cpp
  SkFlattenableSerialization.cpp
  SkFloatBits.cpp
  SkFloat.cpp
  SkFont.cpp
  SkFontDescriptor.cpp
  SkFontHost.cpp
  SkFontStream.cpp
  SkGeometry.cpp
  SkGlyphCache.cpp
  SkGraphics.cpp
  SkImageFilter.cpp
  SkImageGenerator.cpp
  SkImageInfo.cpp
  SkInstCnt.cpp
  SkLineClipper.cpp
  SkLocalMatrixShader.cpp
  SkMallocPixelRef.cpp
  SkMask.cpp
  SkMaskFilter.cpp
  SkMaskGamma.cpp
  SkMath.cpp
  SkMatrixClipStateMgr.cpp
  SkMatrix.cpp
  SkMetaData.cpp
  SkMipMap.cpp
  SkPackBits.cpp
  SkPaint.cpp
  SkPaintOptionsAndroid.cpp
  SkPaintPriv.cpp
  SkPath.cpp
  SkPathEffect.cpp
  SkPathHeap.cpp
  SkPathMeasure.cpp
  SkPathRef.cpp
  SkPicture.cpp
  SkPictureFlat.cpp
  SkPicturePlayback.cpp
  SkPictureRecord.cpp
  SkPictureRecorder.cpp
  SkPictureShader.cpp
  SkPictureStateTree.cpp
  SkPixelRef.cpp
  SkPoint.cpp
  SkProcSpriteBlitter.cpp
  SkPtrRecorder.cpp
  SkQuadClipper.cpp
  SkQuadTree.cpp
  SkRasterClip.cpp
  SkRasterizer.cpp
  SkReadBuffer.cpp
  SkRect.cpp
  SkRefDict.cpp
  SkRegion.cpp
  SkRegion_path.cpp
  SkRRect.cpp
  SkRTree.cpp
  SkScalar.cpp
  SkScaledImageCache.cpp
  SkScalerContext.cpp
  SkScan_Antihair.cpp
  SkScan_AntiPath.cpp
  SkScan.cpp
  SkScan_Hairline.cpp
  SkScan_Path.cpp
  SkShader.cpp
  SkSpriteBlitter_ARGB32.cpp
  SkSpriteBlitter_RGB16.cpp
  SkStream.cpp
  SkString.cpp
  SkStringUtils.cpp
  SkStroke.cpp
  SkStrokeRec.cpp
  SkStrokerPriv.cpp
  SkTileGrid.cpp
  SkTLS.cpp
  SkTSearch.cpp
  SkTypefaceCache.cpp
  SkTypeface.cpp
  SkUnPreMultiply.cpp
  SkUtilsArm.cpp
  SkUtils.cpp
  SkValidatingReadBuffer.cpp
  SkVertState.cpp
  SkWriteBuffer.cpp
  SkWriter32.cpp
  SkXfermode.cpp
  )
set_prefix(SKIA_EFFECTS_SRC src/effects/
  gradients/SkBitmapCache.cpp
  gradients/SkClampRange.cpp
  gradients/SkGradientShader.cpp
  gradients/SkLinearGradient.cpp
  gradients/SkRadialGradient.cpp
  gradients/SkSweepGradient.cpp
  gradients/SkTwoPointConicalGradient.cpp
  gradients/SkTwoPointConicalGradient_gpu.cpp
  gradients/SkTwoPointRadialGradient.cpp
  Sk1DPathEffect.cpp
  Sk2DPathEffect.cpp
  SkAlphaThresholdFilter.cpp
  SkArithmeticMode.cpp
  SkAvoidXfermode.cpp
  SkBicubicImageFilter.cpp
  SkBitmapSource.cpp
  SkBlurDrawLooper.cpp
  SkBlurImageFilter.cpp
  SkBlurMask.cpp
  SkBlurMaskFilter.cpp
  SkColorFilterImageFilter.cpp
  SkColorFilters.cpp
  SkColorMatrix.cpp
  SkColorMatrixFilter.cpp
  SkComposeImageFilter.cpp
  SkCornerPathEffect.cpp
  SkDashPathEffect.cpp
  SkDiscretePathEffect.cpp
  SkDisplacementMapEffect.cpp
  SkDropShadowImageFilter.cpp
  SkEmbossMask.cpp
  SkEmbossMaskFilter.cpp
  SkGpuBlurUtils.cpp
  SkLayerDrawLooper.cpp
  SkLayerRasterizer.cpp
  SkLerpXfermode.cpp
  SkLightingImageFilter.cpp
  SkLumaColorFilter.cpp
  SkMagnifierImageFilter.cpp
  SkMatrixConvolutionImageFilter.cpp
  SkMatrixImageFilter.cpp
  SkMergeImageFilter.cpp
  SkMorphologyImageFilter.cpp
  SkOffsetImageFilter.cpp
  SkPaintFlagsDrawFilter.cpp
  SkPerlinNoiseShader.cpp
  SkPictureImageFilter.cpp
  SkPixelXorXfermode.cpp
  SkPorterDuff.cpp
  SkRectShaderImageFilter.cpp
  SkStippleMaskFilter.cpp
  SkTableColorFilter.cpp
  SkTableMaskFilter.cpp
  SkTestImageFilters.cpp
  SkTileImageFilter.cpp
  SkTransparentShader.cpp
  SkXfermodeImageFilter.cpp
  )
set_prefix(SKIA_GL_SRC src/gpu/
  effects/GrBezierEffect.cpp
  effects/GrBicubicEffect.cpp
  effects/GrConfigConversionEffect.cpp
  effects/GrConvexPolyEffect.cpp
  effects/GrConvolutionEffect.cpp
  effects/GrCustomCoordsTextureEffect.cpp
  effects/GrDashingEffect.cpp
  effects/GrDistanceFieldTextureEffect.cpp
  effects/GrOvalEffect.cpp
  effects/GrRRectEffect.cpp
  effects/GrSimpleTextureEffect.cpp
  effects/GrSingleTextureEffect.cpp
  effects/GrTextureDomain.cpp
  effects/GrTextureStripAtlas.cpp
  gl/debug/GrBufferObj.cpp
  gl/debug/GrDebugGL.cpp
  gl/debug/GrFrameBufferObj.cpp
  gl/debug/GrGLCreateDebugInterface.cpp
  gl/debug/GrProgramObj.cpp
  gl/debug/GrShaderObj.cpp
  gl/debug/GrTextureObj.cpp
  gl/debug/GrTextureUnitObj.cpp
  gl/debug/SkDebugGLContext.cpp
  gl/GrGLAssembleInterface.cpp
  gl/GrGLBufferImpl.cpp
  gl/GrGLCaps.cpp
  gl/GrGLContext.cpp
  gl/GrGLCreateNullInterface.cpp
  gl/GrGLDefaultInterface_native.cpp
  gl/GrGLExtensions.cpp
  gl/GrGLIndexBuffer.cpp
  gl/GrGLInterface.cpp
  gl/GrGLNoOpInterface.cpp
  gl/GrGLPath.cpp
  gl/GrGLProgram.cpp
  gl/GrGLProgramDesc.cpp
  gl/GrGLProgramEffects.cpp
  gl/GrGLRenderTarget.cpp
  gl/GrGLShaderBuilder.cpp
  gl/GrGLSL.cpp
  gl/GrGLStencilBuffer.cpp
  gl/GrGLTexture.cpp
  gl/GrGLUniformManager.cpp
  gl/GrGLUtil.cpp
  gl/GrGLVertexArray.cpp
  gl/GrGLVertexBuffer.cpp
  gl/GrGpuGL.cpp
  gl/GrGpuGL_program.cpp
  gl/GrGLNameAllocator.cpp
  gl/SkGLContextHelper.cpp
  gl/SkNullGLContext.cpp
  GrAAConvexPathRenderer.cpp
  GrAAHairLinePathRenderer.cpp
  GrAARectRenderer.cpp
  GrAddPathRenderers_default.cpp
  GrAllocPool.cpp
  GrAtlas.cpp
  GrBitmapTextContext.cpp
  GrBlend.cpp
  GrBufferAllocPool.cpp
  GrCacheID.cpp
  GrClipData.cpp
  GrClipMaskCache.cpp
  GrClipMaskManager.cpp
  GrContext.cpp
  GrDefaultPathRenderer.cpp
  GrDistanceFieldTextContext.cpp
  GrDrawState.cpp
  GrDrawTarget.cpp
  GrEffect.cpp
  GrGpu.cpp
  GrGpuFactory.cpp
  GrGpuObject.cpp
  GrInOrderDrawBuffer.cpp
  GrLayerCache.cpp
  GrMemoryPool.cpp
  GrOvalRenderer.cpp
  GrPaint.cpp
  GrPath.cpp
  GrPathRendererChain.cpp
  GrPathRenderer.cpp
  GrPathUtils.cpp
  GrPictureUtils.cpp
  GrRectanizer_pow2.cpp
  GrRectanizer_skyline.cpp
  GrReducedClip.cpp
  GrRenderTarget.cpp
  GrResourceCache.cpp
  GrSoftwarePathRenderer.cpp
  GrStencilAndCoverPathRenderer.cpp
  GrStencilBuffer.cpp
  GrStencil.cpp
  GrSurface.cpp
  GrSWMaskHelper.cpp
  GrTest.cpp
  GrTextContext.cpp
  GrTextStrike.cpp
  GrTextureAccess.cpp
  GrTexture.cpp
  GrTraceMarker.cpp
  SkGpuDevice.cpp
  SkGr.cpp
  SkGrFontScaler.cpp
  SkGrPixelRef.cpp
  SkGrTexturePixelRef.cpp
  )
set_prefix(SKIA_IMAGE_SRC src/image/
  SkImage_Codec.cpp
  SkImage.cpp
  SkImage_Gpu.cpp
  SkImagePriv.cpp
  SkImage_Raster.cpp
  SkSurface.cpp
  SkSurface_Gpu.cpp
  SkSurface_Raster.cpp
  )
set_prefix(SKIA_OPTS_SSE2_SRC src/opts/
  SkBitmapFilter_opts_SSE2.cpp
  SkBitmapProcState_opts_SSE2.cpp
  SkBlitRect_opts_SSE2.cpp
  SkBlitRow_opts_SSE2.cpp
  SkBlurImage_opts_SSE2.cpp
  SkMorphology_opts_SSE2.cpp
  SkUtils_opts_SSE2.cpp
  SkXfermode_opts_SSE2.cpp
  opts_check_x86.cpp
  )
set_prefix(SKIA_OPTS_SSSE3_SRC src/opts/
  SkBitmapProcState_opts_SSSE3.cpp
  )
set_prefix(SKIA_OPTS_ARM_NEON_SRC src/opts/
  memset.arm.S
  SkBitmapProcState_opts_arm.cpp
  SkBlitMask_opts_arm.cpp
  SkBlitRow_opts_arm.cpp
  SkBlurImage_opts_arm.cpp
  SkMorphology_opts_arm.cpp
  SkUtils_opts_arm.cpp
  SkXfermode_opts_arm.cpp
  memset16_neon.S
  memset32_neon.S
  SkBitmapProcState_arm_neon.cpp
  SkBitmapProcState_matrixProcs_neon.cpp
  SkBlitMask_opts_arm_neon.cpp
  SkBlitRow_opts_arm_neon.cpp
  SkBlurImage_opts_neon.cpp
  SkMorphology_opts_neon.cpp
  SkXfermode_opts_arm_neon.cpp
  )
set_prefix(SKIA_OPTS_AARCH64_SRC src/opts/
  SkBitmapProcState_opts_arm.cpp
  SkBlitMask_opts_arm.cpp
  SkBlitRow_opts_arm.cpp
  SkBlurImage_opts_arm.cpp
  SkMorphology_opts_arm.cpp
  SkUtils_opts_none.cpp
  SkXfermode_opts_arm.cpp
  SkBitmapProcState_arm_neon.cpp
  SkBitmapProcState_matrixProcs_neon.cpp
  SkBlitMask_opts_arm_neon.cpp
  SkBlitRow_opts_arm_neon.cpp
  SkBlurImage_opts_neon.cpp
  SkMorphology_opts_neon.cpp
  SkXfermode_opts_arm_neon.cpp
  )
set_prefix(SKIA_PATHOPS_SRC src/pathops/
  SkAddIntersections.cpp
  SkDCubicIntersection.cpp
  SkDCubicLineIntersection.cpp
  SkDCubicToQuads.cpp
  SkDLineIntersection.cpp
  SkDQuadImplicit.cpp
  SkDQuadIntersection.cpp
  SkDQuadLineIntersection.cpp
  SkIntersections.cpp
  SkOpAngle.cpp
  SkOpContour.cpp
  SkOpEdgeBuilder.cpp
  SkOpSegment.cpp
  SkPathOpsBounds.cpp
  SkPathOpsCommon.cpp
  SkPathOpsCubic.cpp
  SkPathOpsDebug.cpp
  SkPathOpsLine.cpp
  SkPathOpsOp.cpp
  SkPathOpsPoint.cpp
  SkPathOpsQuad.cpp
  SkPathOpsRect.cpp
  SkPathOpsSimplify.cpp
  SkPathOpsTriangle.cpp
  SkPathOpsTypes.cpp
  SkPathWriter.cpp
  SkQuarticRoot.cpp
  SkReduceOrder.cpp
  )
set_prefix(SKIA_SFNT_SRC src/sfnt/
  SkOTTable_name.cpp
  SkOTUtils.cpp
  )
set_prefix(SKIA_UTILS_SRC src/utils/
  SkBase64.cpp
  SkBitSet.cpp
  SkBoundaryPatch.cpp
  SkCamera.cpp
  SkCanvasStack.cpp
  SkCanvasStateUtils.cpp
  SkCondVar.cpp
  SkCountdown.cpp
  SkCubicInterval.cpp
  SkCullPoints.cpp
  SkDashPath.cpp
  SkDumpCanvas.cpp
  SkEventTracer.cpp
  SkFrontBufferedStream.cpp
  SkGatherPixelRefsAndRects.cpp
  SkInterpolator.cpp
  SkLayer.cpp
  SkMatrix22.cpp
  SkMatrix44.cpp
  SkMD5.cpp
  SkMeshUtils.cpp
  SkNinePatch.cpp
  SkNullCanvas.cpp
  SkNWayCanvas.cpp
  SkOSFile.cpp
  SkParseColor.cpp
  SkParse.cpp
  SkParsePath.cpp
  SkPathUtils.cpp
  SkPDFRasterizer.cpp
  SkPictureUtils.cpp
  SkProxyCanvas.cpp
  SkRTConf.cpp
  SkTextureCompressor.cpp
  SkSHA1.cpp
  )
set_prefix(SKIA_THIRDPARTY_SRC third_party/
  etc1/etc1.cpp
  ktx/ktx.cpp
  )

if(APPLE)
  add_definitions(-DSK_USE_POSIX_THREADS)
  include_directories(include/utils/mac)
  include_directories(src/utils/mac)

  set_prefix(SKIA_GL_PLATFORM_SRC src/gpu/gl/mac/
    GrGLCreateNativeInterface_mac.cpp
    SkNativeGLContext_mac.cpp
    )

  set_prefix(SKIA_PORTS_SRC src/ports/
    SkDebug_stdio.cpp
    SkFontHost_mac.cpp
    SkGlobalInitialization_default.cpp
    SkImageDecoder_empty.cpp
    SkMemory_malloc.cpp
    SkOSFile_posix.cpp
    SkOSFile_stdio.cpp
    SkTLS_pthread.cpp
    )

  set(SKIA_FONTS_SRC "")

  set_prefix(SKIA_UTILS_PLATFORM_SRC src/utils/mac/
    SkCreateCGImageRef.cpp
    SkStream_mac.cpp
    )
endif() # MacOS X

if(WIN32)
  add_definitions(-D_WIN32_WINNT=0x0600 -DSK_BUILD_FOR_WIN32)
  include_directories(include/utils/win)
  include_directories(src/utils/win)

  if (CMAKE_COMPILER_IS_GNUCC)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=gnu++0x -Wno-deprecated-declarations")
    include_directories(/mingw64/include/freetype2)
  endif()

  set_prefix(SKIA_GL_PLATFORM_SRC src/gpu/gl/win/
    GrGLCreateNativeInterface_win.cpp
    SkNativeGLContext_win.cpp
    )

  set_prefix(SKIA_PORTS_SRC src/ports/
    SkDebug_win.cpp
    SkFontHost_win.cpp
    SkGlobalInitialization_default.cpp
    SkImageDecoder_empty.cpp
    SkMemory_malloc.cpp
    SkOSFile_win.cpp
    SkOSFile_stdio.cpp
    SkTLS_win.cpp
    )

  set_prefix(SKIA_UTILS_PLATFORM_SRC src/utils/win/
    SkAutoCoInitialize.cpp
    SkHRESULT.cpp
    SkIStream.cpp
    SkWGL_win.cpp
    )

  if(WIN32)
    # If we're building with DirectWrite and DW only
    set_prefix(SKIA_PORTS_2_SRC src/ports/
      SkFontMgr_default_dw.cpp
      SkFontMgr_win_dw.cpp
      SkTypeface_win_dw.cpp
      SkScalerContext_win_dw.cpp
      )

    set(SKIA_FONTS_SRC
      src/fonts/SkFontMgr_indirect.cpp
      src/utils/win/SkDWrite.cpp
      src/utils/win/SkDWriteFontFileStream.cpp
      src/utils/win/SkDWriteGeometrySink.cpp
      )
  else()
    add_definitions(-DSK_INCLUDE_FREETYPE)
    set_prefix(SKIA_PORTS_2_SRC src/ports/
      SkFontConfigInterface_direct.cpp
      SkFontHost_FreeType.cpp
      SkFontHost_FreeType_common.cpp
      SkFontHost_fontconfig.cpp
      )

    set_prefix(SKIA_FONTS_SRC src/fonts/
      SkFontMgr_fontconfig.cpp
      )
  endif()
endif() # Windows

message(STATUS "Building for target: $ENV{TARGET}")

if($ENV{TARGET} MATCHES ".*android.*")
  add_definitions(-DSK_BUILD_FOR_ANDROID -DEGL_EGLEXT_PROTOTYPES)
  include_directories(platform_tools/android/third_party/cpufeatures)

  # FIXME: The skia version we updated to doesn't have android FontMgr class.
  # However, this was added to skia upstream recently, so next time we update
  # skia this should be removed and have the android FontMgr source file added
  # to the makefile.
  add_definitions(-DSK_FONTHOST_DOES_NOT_USE_FONTMGR=1)

  set_prefix(SKIA_GL_PLATFORM_SRC src/gpu/gl/android/
    GrGLCreateNativeInterface_android.cpp
    SkNativeGLContext_android.cpp
    )

  set_prefix(SKIA_PORTS_SRC src/ports/
    SkDebug_android.cpp
    SkFontConfigInterface_android.cpp
    SkFontConfigParser_android.cpp
    SkFontHost_FreeType.cpp
    SkFontHost_FreeType_common.cpp
    SkFontHost_fontconfig.cpp
    SkGlobalInitialization_default.cpp
    SkImageDecoder_empty.cpp
    SkMemory_malloc.cpp
    SkOSFile_posix.cpp
    SkOSFile_stdio.cpp
    SkTLS_pthread.cpp
    )

  set_prefix(SKIA_UTILS_PLATFORM_SRC platform_tools/android/third_party/cpufeatures/
    cpu-features.c
    )
elseif($ENV{TARGET} MATCHES ".*linux.*")
  set_prefix(SKIA_GL_PLATFORM_SRC src/gpu/gl/unix/
    GrGLCreateNativeInterface_unix.cpp
    SkNativeGLContext_unix.cpp
    )

  set_prefix(SKIA_PORTS_SRC src/ports/
    SkDebug_stdio.cpp
    SkFontConfigInterface_direct.cpp
    SkFontHost_FreeType.cpp
    SkFontHost_FreeType_common.cpp
    SkFontHost_fontconfig.cpp
    SkGlobalInitialization_default.cpp
    SkImageDecoder_empty.cpp
    SkMemory_malloc.cpp
    SkOSFile_posix.cpp
    SkOSFile_stdio.cpp
    SkTLS_pthread.cpp
    )

  set_prefix(SKIA_FONTS_SRC src/fonts/
    SkFontMgr_fontconfig.cpp
    )
endif()

set(SKIA_SRC
  src/skia-c.cpp
  ${SKIA_CORE_SRC}
  ${SKIA_EFFECTS_SRC}
  ${SKIA_FONTS_SRC}
  ${SKIA_GL_SRC}
  ${SKIA_GL_PLATFORM_SRC}
  ${SKIA_IMAGE_SRC}
  ${SKIA_PATHOPS_SRC}
  ${SKIA_PORTS_SRC}
  ${SKIA_PORTS_2_SRC}
  ${SKIA_SFNT_SRC}
  ${SKIA_THIRDPARTY_SRC}
  ${SKIA_UTILS_SRC}
  ${SKIA_UTILS_PLATFORM_SRC}
  )

if($ENV{TARGET} MATCHES "(i686|x86_64)-.*")
  set(SKIA_SRC ${SKIA_SRC} ${SKIA_OPTS_SSE2_SRC})
  set(SKIA_SRC ${SKIA_SRC} ${SKIA_OPTS_SSSE3_SRC})
  if(NOT MSVC)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -msse2 -mfpmath=sse")
    set_source_files_properties(${SKIA_OPTS_SSSE3_SRC} PROPERTIES COMPILE_FLAGS -mssse3)
  else()
    # /arch:SSE2 is invalid on x86-64 (SSE2 always present)
    if(CMAKE_SYSTEM_PROCESSOR STREQUAL "x86")
      set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /arch:SSE2")
    endif()
  endif()
elseif($ENV{TARGET} MATCHES "(arm|armv7)-.*")
  set(SKIA_SRC ${SKIA_SRC} ${SKIA_OPTS_ARM_NEON_SRC})
  add_definitions(-mfpu=neon -D__ARM_HAVE_OPTIONAL_NEON_SUPPORT)
  # FIXME: Need more advanced detection of FP support
  if($ENV{TARGET} MATCHES ".*eabihf.*")
    add_definitions(-mfloat-abi=hard)
  else()
    add_definitions(-mfloat-abi=softfp)
  endif()
  if(NOT "$ENV{TARGET}" STREQUAL "$ENV{HOST}")
    add_definitions(--target=$ENV{TARGET})
  endif()
elseif($ENV{TARGET} MATCHES "aarch64-.*")
  set(SKIA_SRC ${SKIA_SRC} ${SKIA_OPTS_AARCH64_SRC})
  add_definitions(-D__ARM_HAVE_NEON)
endif()

add_library(skia STATIC ${SKIA_SRC})
install(TARGETS skia ARCHIVE DESTINATION lib)
install(DIRECTORY include/ DESTINATION include)
install(DIRECTORY src/ DESTINATION src)
