use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, CFType, CFTypeID, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    string::{CFString, CFStringRef},
};
use libc::c_void;

use crate::{
    opengl_buffer::{CVOpenGLBuffer, CVOpenGLBufferRef},
    return_::{kCVReturnSuccess, CVReturn},
};

#[repr(C)]
pub struct __CVOpenGLBufferPool(c_void);

pub type CVOpenGLBufferPoolRef = *const __CVOpenGLBufferPool;

extern "C" {
    pub static kCVOpenGLBufferPoolMinimumBufferCountKey: CFStringRef;
    pub static kCVOpenGLBufferPoolMaximumBufferAgeKey: CFStringRef;

    pub fn CVOpenGLBufferPoolGetTypeID() -> CFTypeID;
    pub fn CVOpenGLBufferPoolRetain(openGLBufferPool: CVOpenGLBufferPoolRef) -> CVOpenGLBufferPoolRef;
    pub fn CVOpenGLBufferPoolRelease(openGLBufferPool: CVOpenGLBufferPoolRef);
    pub fn CVOpenGLBufferPoolCreate(
        allocator: CFAllocatorRef,
        poolAttributes: CFDictionaryRef,
        openGLBufferAttributes: CFDictionaryRef,
        poolOut: *mut CVOpenGLBufferPoolRef,
    ) -> CVReturn;
    pub fn CVOpenGLBufferPoolGetAttributes(pool: CVOpenGLBufferPoolRef) -> CFDictionaryRef;
    pub fn CVOpenGLBufferPoolGetOpenGLBufferAttributes(pool: CVOpenGLBufferPoolRef) -> CFDictionaryRef;
    pub fn CVOpenGLBufferPoolCreateOpenGLBuffer(
        allocator: CFAllocatorRef,
        openGLBufferPool: CVOpenGLBufferPoolRef,
        openGLBufferOut: *mut CVOpenGLBufferRef,
    ) -> CVReturn;
}

pub struct CVOpenGLBufferPool(CVOpenGLBufferPoolRef);

impl Drop for CVOpenGLBufferPool {
    fn drop(&mut self) {
        unsafe { CVOpenGLBufferPoolRelease(self.0) }
    }
}

impl_TCFType!(CVOpenGLBufferPool, CVOpenGLBufferPoolRef, CVOpenGLBufferPoolGetTypeID);
impl_CFTypeDescription!(CVOpenGLBufferPool);

impl CVOpenGLBufferPool {
    pub fn new(
        pool_attributes: &CFDictionary<CFString, CFType>,
        opengl_buffer_attributes: &CFDictionary<CFString, CFType>,
    ) -> Result<CVOpenGLBufferPool, CVReturn> {
        let mut pool: CVOpenGLBufferPoolRef = std::ptr::null_mut();
        let status = unsafe {
            CVOpenGLBufferPoolCreate(
                kCFAllocatorDefault,
                pool_attributes.as_concrete_TypeRef(),
                opengl_buffer_attributes.as_concrete_TypeRef(),
                &mut pool,
            )
        };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(pool) })
        } else {
            Err(status)
        }
    }

    pub fn get_attributes(&self) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let attributes = CVOpenGLBufferPoolGetAttributes(self.0);
            if attributes.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(attributes))
            }
        }
    }

    pub fn get_opengl_buffer_attributes(&self) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let attributes = CVOpenGLBufferPoolGetOpenGLBufferAttributes(self.0);
            if attributes.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(attributes))
            }
        }
    }

    pub fn create_open_gl_buffer(&self) -> Result<CVOpenGLBuffer, CVReturn> {
        let mut buffer: CVOpenGLBufferRef = std::ptr::null_mut();
        let status = unsafe { CVOpenGLBufferPoolCreateOpenGLBuffer(kCFAllocatorDefault, self.0, &mut buffer) };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(buffer) })
        } else {
            Err(status)
        }
    }
}
