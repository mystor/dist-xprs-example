//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgIEncoder.idl
//


pub mod imgIEncoder_consts {
    pub const INPUT_FORMAT_RGB: i64 = 0;
    pub const INPUT_FORMAT_RGBA: i64 = 1;
    pub const INPUT_FORMAT_HOSTARGB: i64 = 2;
}


#[repr(C)]
pub struct imgIEncoder {
    vtable: *const imgIEncoderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for imgIEncoder {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4baa2d6e, 0xfee7, 0x42df,
            [0xae, 0x3f, 0x5f, 0xbe, 0xbc, 0x0c, 0x26, 0x7c])
    }
}

unsafe impl RefCounted for imgIEncoder {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// Enable coercing to ourselves
pub trait imgIEncoderCoerce {
    fn coerce_from(v: &imgIEncoder) -> &Self;
}

impl imgIEncoderCoerce for imgIEncoder {
    #[inline]
    fn coerce_from(v: &imgIEncoder) -> &Self {
        v
    }
}

impl imgIEncoder {
    #[inline]
    pub fn coerce<T: imgIEncoderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for imgIEncoder {
    type Target = nsIAsyncInputStream;
    #[inline]
    fn deref(&self) -> &nsIAsyncInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIAsyncInputStreamCoerce> imgIEncoderCoerce for T {
    #[inline]
    fn coerce_from(v: &imgIEncoder) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct imgIEncoderVTable {
    pub __base: nsIAsyncInputStreamVTable,

    /* void initFromData ([array, size_is (length), const] in uint8_t data, in unsigned long length, in uint32_t width, in uint32_t height, in uint32_t stride, in uint32_t inputFormat, in AString outputOptions); */
    /// Unable to call function as its signature contains a non-rust type
    pub initFromData: *const ::libc::c_void,

    /* void startImageEncode (in uint32_t width, in uint32_t height, in uint32_t inputFormat, in AString outputOptions); */
    pub startImageEncode: unsafe extern "C" fn (this: *const imgIEncoder, width: uint32_t, height: uint32_t, inputFormat: uint32_t, outputOptions: *const nsAString) -> nsresult,

    /* void addImageFrame ([array, size_is (length), const] in uint8_t data, in unsigned long length, in uint32_t width, in uint32_t height, in uint32_t stride, in uint32_t frameFormat, in AString frameOptions); */
    /// Unable to call function as its signature contains a non-rust type
    pub addImageFrame: *const ::libc::c_void,

    /* void endImageEncode (); */
    pub endImageEncode: unsafe extern "C" fn (this: *const imgIEncoder) -> nsresult,

    /* [noscript] unsigned long getImageBufferUsed (); */
    pub getImageBufferUsed: unsafe extern "C" fn (this: *const imgIEncoder, _retval: *mut libc::uint32_t) -> nsresult,

    /* [noscript] charPtr getImageBuffer (); */
    pub getImageBuffer: unsafe extern "C" fn (this: *const imgIEncoder, _retval: *mut *const u8) -> nsresult,

}


impl imgIEncoder {
    /* void initFromData ([array, size_is (length), const] in uint8_t data, in unsigned long length, in uint32_t width, in uint32_t height, in uint32_t stride, in uint32_t inputFormat, in AString outputOptions); */


    /* void startImageEncode (in uint32_t width, in uint32_t height, in uint32_t inputFormat, in AString outputOptions); */
    #[inline]
    pub unsafe fn startImageEncode(&self, width: uint32_t, height: uint32_t, inputFormat: uint32_t, outputOptions: &[u16]) -> Result<(), nsresult> {
        let outputOptions = nsString::from(outputOptions);
        match ((*self.vtable).startImageEncode)(self as *const _, width, height, inputFormat, &*outputOptions) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addImageFrame ([array, size_is (length), const] in uint8_t data, in unsigned long length, in uint32_t width, in uint32_t height, in uint32_t stride, in uint32_t frameFormat, in AString frameOptions); */


    /* void endImageEncode (); */
    #[inline]
    pub unsafe fn endImageEncode(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).endImageEncode)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] unsigned long getImageBufferUsed (); */
    #[inline]
    pub unsafe fn getImageBufferUsed(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getImageBufferUsed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] charPtr getImageBuffer (); */
    #[inline]
    pub unsafe fn getImageBuffer(&self, ) -> Result<*const u8, nsresult> {
        let mut _retval: *const u8 = ::std::ptr::null();
        match ((*self.vtable).getImageBuffer)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


