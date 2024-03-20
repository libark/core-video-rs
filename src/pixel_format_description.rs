use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{kCFAllocatorDefault, Boolean, CFAllocatorRef, CFIndex, CFType, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    number::CFNumber,
    string::{CFString, CFStringRef},
};
use libc::c_void;

use crate::{pixel_buffer::CVPixelBufferRef, OSType};

pub type CVFillExtendedPixelsCallBack = extern "C" fn(pixelBuffer: CVPixelBufferRef, refCon: *mut c_void) -> Boolean;

#[repr(C)]
pub struct CVFillExtendedPixelsCallBackData {
    pub version: CFIndex,
    pub fillCallBack: CVFillExtendedPixelsCallBack,
    pub refCon: *mut c_void,
}

extern "C" {
    pub static kCVPixelFormatName: CFStringRef;
    pub static kCVPixelFormatConstant: CFStringRef;
    pub static kCVPixelFormatCodecType: CFStringRef;
    pub static kCVPixelFormatFourCC: CFStringRef;
    pub static kCVPixelFormatContainsAlpha: CFStringRef;
    pub static kCVPixelFormatContainsYCbCr: CFStringRef;
    pub static kCVPixelFormatContainsRGB: CFStringRef;
    pub static kCVPixelFormatComponentRange: CFStringRef;
    pub static kCVPixelFormatComponentRange_VideoRange: CFStringRef;
    pub static kCVPixelFormatComponentRange_FullRange: CFStringRef;
    pub static kCVPixelFormatComponentRange_WideRange: CFStringRef;
    pub static kCVPixelFormatPlanes: CFStringRef;
    pub static kCVPixelFormatBlockWidth: CFStringRef;
    pub static kCVPixelFormatBlockHeight: CFStringRef;
    pub static kCVPixelFormatBitsPerBlock: CFStringRef;
    pub static kCVPixelFormatBlockHorizontalAlignment: CFStringRef;
    pub static kCVPixelFormatBlockVerticalAlignment: CFStringRef;
    pub static kCVPixelFormatBlackBlock: CFStringRef;
    pub static kCVPixelFormatHorizontalSubsampling: CFStringRef;
    pub static kCVPixelFormatVerticalSubsampling: CFStringRef;
    pub static kCVPixelFormatOpenGLFormat: CFStringRef;
    pub static kCVPixelFormatOpenGLType: CFStringRef;
    pub static kCVPixelFormatOpenGLInternalFormat: CFStringRef;
    pub static kCVPixelFormatCGBitmapInfo: CFStringRef;
    pub static kCVPixelFormatQDCompatibility: CFStringRef;
    pub static kCVPixelFormatCGBitmapContextCompatibility: CFStringRef;
    pub static kCVPixelFormatCGImageCompatibility: CFStringRef;
    pub static kCVPixelFormatOpenGLCompatibility: CFStringRef;
    pub static kCVPixelFormatOpenGLESCompatibility: CFStringRef;
    pub static kCVPixelFormatFillExtendedPixelsCallback: CFStringRef;

    pub fn CVPixelFormatDescriptionCreateWithPixelFormatType(allocator: CFAllocatorRef, pixelFormat: OSType) -> CFDictionaryRef;
    pub fn CVPixelFormatDescriptionArrayCreateWithAllPixelFormatTypes(allocator: CFAllocatorRef) -> CFArrayRef;
    pub fn CVPixelFormatDescriptionRegisterDescriptionWithPixelFormatType(description: CFDictionaryRef, pixelFormat: OSType);
    pub fn CVIsCompressedPixelFormatAvailable(pixelFormat: OSType) -> Boolean;
}

pub fn pixel_format_description_create_with_pixel_format_type(pixel_format: OSType) -> Result<CFDictionary<CFString, CFType>, ()> {
    unsafe {
        let description = CVPixelFormatDescriptionCreateWithPixelFormatType(kCFAllocatorDefault, pixel_format);
        if description.is_null() {
            Err(())
        } else {
            Ok(TCFType::wrap_under_create_rule(description))
        }
    }
}

pub fn pixel_format_description_array_create_with_all_pixel_format_types() -> Result<CFArray<CFNumber>, ()> {
    unsafe {
        let array = CVPixelFormatDescriptionArrayCreateWithAllPixelFormatTypes(kCFAllocatorDefault);
        if array.is_null() {
            Err(())
        } else {
            Ok(TCFType::wrap_under_create_rule(array))
        }
    }
}

pub fn is_compressed_pixel_format_available(pixel_format: OSType) -> bool {
    unsafe { CVIsCompressedPixelFormatAvailable(pixel_format) != 0 }
}
