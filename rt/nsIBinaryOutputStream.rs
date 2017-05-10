//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBinaryOutputStream.idl
//


#[repr(C)]
pub struct nsIBinaryOutputStream {
    vtable: *const nsIBinaryOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBinaryOutputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x204ee610, 0x8765, 0x11d3,
            [0x90, 0xcf, 0x00, 0x40, 0x05, 0x6a, 0x90, 0x6e])
    }
}

unsafe impl RefCounted for nsIBinaryOutputStream {
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
pub trait nsIBinaryOutputStreamCoerce {
    fn coerce_from(v: &nsIBinaryOutputStream) -> &Self;
}

impl nsIBinaryOutputStreamCoerce for nsIBinaryOutputStream {
    #[inline]
    fn coerce_from(v: &nsIBinaryOutputStream) -> &Self {
        v
    }
}

impl nsIBinaryOutputStream {
    #[inline]
    pub fn coerce<T: nsIBinaryOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBinaryOutputStream {
    type Target = nsIOutputStream;
    #[inline]
    fn deref(&self) -> &nsIOutputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIOutputStreamCoerce> nsIBinaryOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBinaryOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBinaryOutputStreamVTable {
    pub __base: nsIOutputStreamVTable,

    /* void setOutputStream (in nsIOutputStream aOutputStream); */
    pub setOutputStream: unsafe extern "C" fn (this: *const nsIBinaryOutputStream, aOutputStream: *const nsIOutputStream) -> nsresult,

    /* void writeBoolean (in boolean aBoolean); */
    pub writeBoolean: unsafe extern "C" fn (this: *const nsIBinaryOutputStream, aBoolean: bool) -> nsresult,

    /* void write8 (in uint8_t aByte); */
    pub write8: unsafe extern "C" fn (this: *const nsIBinaryOutputStream, aByte: uint8_t) -> nsresult,

    /* void write16 (in uint16_t a16); */
    pub write16: unsafe extern "C" fn (this: *const nsIBinaryOutputStream, a16: uint16_t) -> nsresult,

    /* void write32 (in uint32_t a32); */
    pub write32: unsafe extern "C" fn (this: *const nsIBinaryOutputStream, a32: uint32_t) -> nsresult,

    /* void write64 (in uint64_t a64); */
    pub write64: unsafe extern "C" fn (this: *const nsIBinaryOutputStream, a64: uint64_t) -> nsresult,

    /* void writeFloat (in float aFloat); */
    pub writeFloat: unsafe extern "C" fn (this: *const nsIBinaryOutputStream, aFloat: libc::c_float) -> nsresult,

    /* void writeDouble (in double aDouble); */
    pub writeDouble: unsafe extern "C" fn (this: *const nsIBinaryOutputStream, aDouble: libc::c_double) -> nsresult,

    /* void writeStringZ (in string aString); */
    pub writeStringZ: unsafe extern "C" fn (this: *const nsIBinaryOutputStream, aString: *const libc::c_char) -> nsresult,

    /* void writeWStringZ (in wstring aString); */
    pub writeWStringZ: unsafe extern "C" fn (this: *const nsIBinaryOutputStream, aString: *const libc::int16_t) -> nsresult,

    /* void writeUtf8Z (in wstring aString); */
    pub writeUtf8Z: unsafe extern "C" fn (this: *const nsIBinaryOutputStream, aString: *const libc::int16_t) -> nsresult,

    /* void writeBytes ([size_is (aLength)] in string aString, in uint32_t aLength); */
    pub writeBytes: unsafe extern "C" fn (this: *const nsIBinaryOutputStream, aString: *const libc::c_char, aLength: uint32_t) -> nsresult,

    /* void writeByteArray ([array, size_is (aLength)] in uint8_t aBytes, in uint32_t aLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub writeByteArray: *const ::libc::c_void,

}


impl nsIBinaryOutputStream {
    /* void setOutputStream (in nsIOutputStream aOutputStream); */
    #[inline]
    pub unsafe fn setOutputStream(&self, aOutputStream: Option<&nsIOutputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).setOutputStream)(self as *const _, aOutputStream.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void writeBoolean (in boolean aBoolean); */
    #[inline]
    pub unsafe fn writeBoolean(&self, aBoolean: bool) -> Result<(), nsresult> {

        match ((*self.vtable).writeBoolean)(self as *const _, aBoolean) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void write8 (in uint8_t aByte); */
    #[inline]
    pub unsafe fn write8(&self, aByte: uint8_t) -> Result<(), nsresult> {

        match ((*self.vtable).write8)(self as *const _, aByte) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void write16 (in uint16_t a16); */
    #[inline]
    pub unsafe fn write16(&self, a16: uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).write16)(self as *const _, a16) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void write32 (in uint32_t a32); */
    #[inline]
    pub unsafe fn write32(&self, a32: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).write32)(self as *const _, a32) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void write64 (in uint64_t a64); */
    #[inline]
    pub unsafe fn write64(&self, a64: uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).write64)(self as *const _, a64) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void writeFloat (in float aFloat); */
    #[inline]
    pub unsafe fn writeFloat(&self, aFloat: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).writeFloat)(self as *const _, aFloat) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void writeDouble (in double aDouble); */
    #[inline]
    pub unsafe fn writeDouble(&self, aDouble: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).writeDouble)(self as *const _, aDouble) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void writeStringZ (in string aString); */
    #[inline]
    pub unsafe fn writeStringZ(&self, aString: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).writeStringZ)(self as *const _, aString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void writeWStringZ (in wstring aString); */
    #[inline]
    pub unsafe fn writeWStringZ(&self, aString: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).writeWStringZ)(self as *const _, aString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void writeUtf8Z (in wstring aString); */
    #[inline]
    pub unsafe fn writeUtf8Z(&self, aString: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).writeUtf8Z)(self as *const _, aString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void writeBytes ([size_is (aLength)] in string aString, in uint32_t aLength); */
    #[inline]
    pub unsafe fn writeBytes(&self, aString: *const libc::c_char, aLength: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).writeBytes)(self as *const _, aString, aLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void writeByteArray ([array, size_is (aLength)] in uint8_t aBytes, in uint32_t aLength); */


}


