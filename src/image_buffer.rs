use std::mem;

use core_foundation::{
    base::{Boolean, CFGetTypeID, CFType, CFTypeID, CFTypeRef, TCFType, TCFTypeRef},
    dictionary::{CFDictionary, CFDictionaryRef},
    string::{CFString, CFStringRef},
};
use core_graphics::{
    color_space::CGColorSpace,
    geometry::{CGRect, CGSize},
    sys::CGColorSpaceRef,
};
use foreign_types::ForeignType;

use crate::buffer::{CVBuffer, CVBufferRef, CVBufferRelease, CVBufferRetain, TCVBuffer};

pub type CVImageBufferRef = CVBufferRef;

extern "C" {
    pub static kCVImageBufferCGColorSpaceKey: CFStringRef;
    pub static kCVImageBufferCleanApertureKey: CFStringRef;
    pub static kCVImageBufferCleanApertureWidthKey: CFStringRef;
    pub static kCVImageBufferCleanApertureHeightKey: CFStringRef;
    pub static kCVImageBufferCleanApertureHorizontalOffsetKey: CFStringRef;
    pub static kCVImageBufferCleanApertureVerticalOffsetKey: CFStringRef;
    pub static kCVImageBufferPreferredCleanApertureKey: CFStringRef;
    pub static kCVImageBufferFieldCountKey: CFStringRef;
    pub static kCVImageBufferFieldDetailKey: CFStringRef;
    pub static kCVImageBufferFieldDetailTemporalTopFirst: CFStringRef;
    pub static kCVImageBufferFieldDetailTemporalBottomFirst: CFStringRef;
    pub static kCVImageBufferFieldDetailSpatialFirstLineEarly: CFStringRef;
    pub static kCVImageBufferFieldDetailSpatialFirstLineLate: CFStringRef;
    pub static kCVImageBufferPixelAspectRatioKey: CFStringRef;
    pub static kCVImageBufferPixelAspectRatioHorizontalSpacingKey: CFStringRef;
    pub static kCVImageBufferPixelAspectRatioVerticalSpacingKey: CFStringRef;
    pub static kCVImageBufferDisplayDimensionsKey: CFStringRef;
    pub static kCVImageBufferDisplayWidthKey: CFStringRef;
    pub static kCVImageBufferDisplayHeightKey: CFStringRef;
    pub static kCVImageBufferGammaLevelKey: CFStringRef;
    pub static kCVImageBufferICCProfileKey: CFStringRef;
    pub static kCVImageBufferYCbCrMatrixKey: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_ITU_R_709_2: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_ITU_R_601_4: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_SMPTE_240M_1995: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_DCI_P3: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_P3_D65: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_ITU_R_2020: CFStringRef;
    pub static kCVImageBufferColorPrimariesKey: CFStringRef;
    pub static kCVImageBufferColorPrimaries_ITU_R_709_2: CFStringRef;
    pub static kCVImageBufferColorPrimaries_EBU_3213: CFStringRef;
    pub static kCVImageBufferColorPrimaries_SMPTE_C: CFStringRef;
    pub static kCVImageBufferColorPrimaries_P22: CFStringRef;
    pub static kCVImageBufferColorPrimaries_DCI_P3: CFStringRef;
    pub static kCVImageBufferColorPrimaries_P3_D65: CFStringRef;
    pub static kCVImageBufferColorPrimaries_ITU_R_2020: CFStringRef;
    pub static kCVImageBufferTransferFunctionKey: CFStringRef;
    pub static kCVImageBufferTransferFunction_ITU_R_709_2: CFStringRef;
    pub static kCVImageBufferTransferFunction_SMPTE_240M_1995: CFStringRef;
    pub static kCVImageBufferTransferFunction_UseGamma: CFStringRef;
    pub static kCVImageBufferTransferFunction_sRGB: CFStringRef;
    pub static kCVImageBufferTransferFunction_ITU_R_2020: CFStringRef;
    pub static kCVImageBufferTransferFunction_SMPTE_ST_428_1: CFStringRef;
    pub static kCVImageBufferTransferFunction_SMPTE_ST_2084_PQ: CFStringRef;
    pub static kCVImageBufferTransferFunction_ITU_R_2100_HLG: CFStringRef;
    pub static kCVImageBufferTransferFunction_Linear: CFStringRef;
    pub static kCVImageBufferChromaLocationTopFieldKey: CFStringRef;
    pub static kCVImageBufferChromaLocationBottomFieldKey: CFStringRef;
    pub static kCVImageBufferChromaLocation_Left: CFStringRef;
    pub static kCVImageBufferChromaLocation_Center: CFStringRef;
    pub static kCVImageBufferChromaLocation_TopLeft: CFStringRef;
    pub static kCVImageBufferChromaLocation_Top: CFStringRef;
    pub static kCVImageBufferChromaLocation_BottomLeft: CFStringRef;
    pub static kCVImageBufferChromaLocation_Bottom: CFStringRef;
    pub static kCVImageBufferChromaLocation_DV420: CFStringRef;
    pub static kCVImageBufferChromaSubsamplingKey: CFStringRef;
    pub static kCVImageBufferChromaSubsampling_420: CFStringRef;
    pub static kCVImageBufferChromaSubsampling_422: CFStringRef;
    pub static kCVImageBufferChromaSubsampling_411: CFStringRef;
    pub static kCVImageBufferAlphaChannelIsOpaque: CFStringRef;
    pub static kCVImageBufferAlphaChannelModeKey: CFStringRef;
    pub static kCVImageBufferAlphaChannelMode_StraightAlpha: CFStringRef;
    pub static kCVImageBufferAlphaChannelMode_PremultipliedAlpha: CFStringRef;

    pub fn CVYCbCrMatrixGetIntegerCodePointForString(yCbCrMatrixString: CFStringRef) -> i32;
    pub fn CVColorPrimariesGetIntegerCodePointForString(colorPrimariesString: CFStringRef) -> i32;
    pub fn CVTransferFunctionGetIntegerCodePointForString(transferFunctionString: CFStringRef) -> i32;
    pub fn CVYCbCrMatrixGetStringForIntegerCodePoint(yCbCrMatrixCodePoint: i32) -> CFStringRef;
    pub fn CVColorPrimariesGetStringForIntegerCodePoint(colorPrimariesCodePoint: i32) -> CFStringRef;
    pub fn CVTransferFunctionGetStringForIntegerCodePoint(transferFunctionCodePoint: i32) -> CFStringRef;

    pub fn CVImageBufferGetEncodedSize(imageBuffer: CVImageBufferRef) -> CGSize;
    pub fn CVImageBufferGetDisplaySize(imageBuffer: CVImageBufferRef) -> CGSize;
    pub fn CVImageBufferGetCleanRect(imageBuffer: CVImageBufferRef) -> CGRect;
    pub fn CVImageBufferIsFlipped(imageBuffer: CVImageBufferRef) -> Boolean;
    pub fn CVImageBufferGetColorSpace(imageBuffer: CVImageBufferRef) -> CGColorSpaceRef;
    pub fn CVImageBufferCreateColorSpaceFromAttachments(attachments: CFDictionaryRef) -> CGColorSpaceRef;

    pub static kCVImageBufferMasteringDisplayColorVolumeKey: CFStringRef;
    pub static kCVImageBufferContentLightLevelInfoKey: CFStringRef;
    pub static kCVImageBufferAmbientViewingEnvironmentKey: CFStringRef;
    pub static kCVImageBufferRegionOfInterestKey: CFStringRef;
}

pub struct CVImageBuffer(CVImageBufferRef);

impl Drop for CVImageBuffer {
    fn drop(&mut self) {
        unsafe { CVBufferRelease(self.0) }
    }
}

impl CVImageBuffer {
    #[inline]
    pub fn as_concrete_TypeRef(&self) -> CVImageBufferRef {
        self.0
    }

    #[inline]
    pub fn as_CFType(&self) -> CFType {
        unsafe { CFType::wrap_under_get_rule(self.as_CFTypeRef()) }
    }

    #[inline]
    pub fn as_CFTypeRef(&self) -> CFTypeRef {
        self.as_concrete_TypeRef() as CFTypeRef
    }

    #[inline]
    pub fn into_CFType(self) -> CFType {
        let reference = self.as_CFTypeRef();
        mem::forget(self);
        unsafe { CFType::wrap_under_create_rule(reference) }
    }

    #[inline]
    pub unsafe fn wrap_under_create_rule(reference: CVImageBufferRef) -> CVImageBuffer {
        CVImageBuffer(reference)
    }

    #[inline]
    pub unsafe fn wrap_under_get_rule(reference: CVImageBufferRef) -> CVImageBuffer {
        CVImageBuffer(CVBufferRetain(reference))
    }

    #[inline]
    pub fn type_of(&self) -> CFTypeID {
        unsafe { CFGetTypeID(self.as_CFTypeRef()) }
    }

    #[inline]
    pub fn instance_of<T: TCFType>(&self) -> bool {
        self.type_of() == T::type_id()
    }
}

impl Clone for CVImageBuffer {
    fn clone(&self) -> CVImageBuffer {
        unsafe { CVImageBuffer::wrap_under_get_rule(self.0) }
    }
}

impl PartialEq for CVImageBuffer {
    fn eq(&self, other: &CVImageBuffer) -> bool {
        self.as_CFType().eq(&other.as_CFType())
    }
}

impl Eq for CVImageBuffer {}

impl_CFTypeDescription!(CVImageBuffer);

pub trait TCVImageBuffer: TCVBuffer {
    #[inline]
    fn as_image_buffer(&self) -> CVImageBuffer {
        unsafe { CVImageBuffer::wrap_under_get_rule(self.as_concrete_TypeRef().as_void_ptr() as CVImageBufferRef) }
    }

    #[inline]
    fn into_image_buffer(self) -> CVImageBuffer
    where
        Self: Sized,
    {
        let reference = self.as_concrete_TypeRef().as_void_ptr() as CVImageBufferRef;
        mem::forget(self);
        unsafe { CVImageBuffer::wrap_under_create_rule(reference) }
    }
}

impl CVImageBuffer {
    pub fn as_buffer(&self) -> CVBuffer {
        unsafe { CVBuffer::wrap_under_get_rule(self.as_concrete_TypeRef() as CVBufferRef) }
    }

    pub fn into_buffer(self) -> CVBuffer
    where
        Self: Sized,
    {
        let reference = self.as_concrete_TypeRef() as CVBufferRef;
        mem::forget(self);
        unsafe { CVBuffer::wrap_under_create_rule(reference) }
    }

    pub fn get_encoded_size(&self) -> CGSize {
        unsafe { CVImageBufferGetEncodedSize(self.as_concrete_TypeRef()) }
    }

    pub fn get_display_size(&self) -> CGSize {
        unsafe { CVImageBufferGetDisplaySize(self.as_concrete_TypeRef()) }
    }

    pub fn get_clean_rect(&self) -> CGRect {
        unsafe { CVImageBufferGetCleanRect(self.as_concrete_TypeRef()) }
    }

    pub fn is_flipped(&self) -> bool {
        unsafe { CVImageBufferIsFlipped(self.as_concrete_TypeRef()) != 0 }
    }

    pub fn get_color_space(&self) -> Option<CGColorSpace> {
        unsafe {
            let color_space = CVImageBufferGetColorSpace(self.as_concrete_TypeRef());
            if color_space.is_null() {
                None
            } else {
                Some(CGColorSpace::from_ptr(color_space))
            }
        }
    }
}

pub fn create_color_space_from_attachments(attachments: &CFDictionary<CFString, CFType>) -> Option<CGColorSpace> {
    unsafe {
        let color_space = CVImageBufferCreateColorSpaceFromAttachments(attachments.as_concrete_TypeRef());
        if color_space.is_null() {
            None
        } else {
            Some(CGColorSpace::from_ptr(color_space))
        }
    }
}
