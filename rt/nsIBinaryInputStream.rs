//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBinaryInputStream.idl
//


#[repr(C)]
pub struct nsIBinaryInputStream {
    vtable: *const nsIBinaryInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBinaryInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x899b826b, 0x2eb3, 0x469c,
            [0x8b, 0x31, 0x4c, 0x29, 0xf5, 0xd3, 0x41, 0xa6])
    }
}

unsafe impl RefCounted for nsIBinaryInputStream {
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
pub trait nsIBinaryInputStreamCoerce {
    fn coerce_from(v: &nsIBinaryInputStream) -> &Self;
}

impl nsIBinaryInputStreamCoerce for nsIBinaryInputStream {
    #[inline]
    fn coerce_from(v: &nsIBinaryInputStream) -> &Self {
        v
    }
}

impl nsIBinaryInputStream {
    #[inline]
    pub fn coerce<T: nsIBinaryInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBinaryInputStream {
    type Target = nsIInputStream;
    #[inline]
    fn deref(&self) -> &nsIInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIInputStreamCoerce> nsIBinaryInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBinaryInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBinaryInputStreamVTable {
    pub __base: nsIInputStreamVTable,

    /* void setInputStream (in nsIInputStream aInputStream); */
    pub setInputStream: unsafe extern "C" fn (this: *const nsIBinaryInputStream, aInputStream: *const nsIInputStream) -> nsresult,

    /* boolean readBoolean (); */
    pub readBoolean: unsafe extern "C" fn (this: *const nsIBinaryInputStream, _retval: *mut bool) -> nsresult,

    /* uint8_t read8 (); */
    pub read8: unsafe extern "C" fn (this: *const nsIBinaryInputStream, _retval: *mut uint8_t) -> nsresult,

    /* uint16_t read16 (); */
    pub read16: unsafe extern "C" fn (this: *const nsIBinaryInputStream, _retval: *mut uint16_t) -> nsresult,

    /* uint32_t read32 (); */
    pub read32: unsafe extern "C" fn (this: *const nsIBinaryInputStream, _retval: *mut uint32_t) -> nsresult,

    /* uint64_t read64 (); */
    pub read64: unsafe extern "C" fn (this: *const nsIBinaryInputStream, _retval: *mut uint64_t) -> nsresult,

    /* float readFloat (); */
    pub readFloat: unsafe extern "C" fn (this: *const nsIBinaryInputStream, _retval: *mut libc::c_float) -> nsresult,

    /* double readDouble (); */
    pub readDouble: unsafe extern "C" fn (this: *const nsIBinaryInputStream, _retval: *mut libc::c_double) -> nsresult,

    /* ACString readCString (); */
    pub readCString: unsafe extern "C" fn (this: *const nsIBinaryInputStream, _retval: *mut nsACString) -> nsresult,

    /* AString readString (); */
    pub readString: unsafe extern "C" fn (this: *const nsIBinaryInputStream, _retval: *mut nsAString) -> nsresult,

    /* void readBytes (in uint32_t aLength, [size_is (aLength), retval] out string aString); */
    pub readBytes: unsafe extern "C" fn (this: *const nsIBinaryInputStream, aLength: uint32_t, aString: *mut *const libc::c_char) -> nsresult,

    /* void readByteArray (in uint32_t aLength, [array, size_is (aLength), retval] out uint8_t aBytes); */
    /// Unable to call function as its signature contains a non-rust type
    pub readByteArray: *const ::libc::c_void,

    /* [implicit_jscontext] unsigned long readArrayBuffer (in uint32_t aLength, in jsval aArrayBuffer); */
    /// Unable to call function as its signature contains a non-rust type
    pub readArrayBuffer: *const ::libc::c_void,

}


impl nsIBinaryInputStream {
    /* void setInputStream (in nsIInputStream aInputStream); */
    #[inline]
    pub unsafe fn setInputStream(&self, aInputStream: Option<&nsIInputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).setInputStream)(self as *const _, aInputStream.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean readBoolean (); */
    #[inline]
    pub unsafe fn readBoolean(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).readBoolean)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* uint8_t read8 (); */
    #[inline]
    pub unsafe fn read8(&self, ) -> Result<uint8_t, nsresult> {
        let mut _retval: uint8_t = ::std::mem::zeroed();
        match ((*self.vtable).read8)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* uint16_t read16 (); */
    #[inline]
    pub unsafe fn read16(&self, ) -> Result<uint16_t, nsresult> {
        let mut _retval: uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).read16)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* uint32_t read32 (); */
    #[inline]
    pub unsafe fn read32(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).read32)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* uint64_t read64 (); */
    #[inline]
    pub unsafe fn read64(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).read64)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* float readFloat (); */
    #[inline]
    pub unsafe fn readFloat(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).readFloat)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* double readDouble (); */
    #[inline]
    pub unsafe fn readDouble(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).readDouble)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString readCString (); */
    #[inline]
    pub unsafe fn readCString(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).readCString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString readString (); */
    #[inline]
    pub unsafe fn readString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).readString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void readBytes (in uint32_t aLength, [size_is (aLength), retval] out string aString); */
    #[inline]
    pub unsafe fn readBytes(&self, aLength: uint32_t) -> Result<*const libc::c_char, nsresult> {
        let mut aString: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).readBytes)(self as *const _, aLength, &mut aString as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aString)
    }

    /* void readByteArray (in uint32_t aLength, [array, size_is (aLength), retval] out uint8_t aBytes); */


    /* [implicit_jscontext] unsigned long readArrayBuffer (in uint32_t aLength, in jsval aArrayBuffer); */


}


