//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStringStream.idl
//


#[repr(C)]
pub struct nsIStringInputStream {
    vtable: *const nsIStringInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStringInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x450cd2d4, 0xf0fd, 0x424d,
            [0xb3, 0x65, 0xb1, 0x25, 0x1f, 0x80, 0xfd, 0x53])
    }
}

unsafe impl RefCounted for nsIStringInputStream {
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
pub trait nsIStringInputStreamCoerce {
    fn coerce_from(v: &nsIStringInputStream) -> &Self;
}

impl nsIStringInputStreamCoerce for nsIStringInputStream {
    #[inline]
    fn coerce_from(v: &nsIStringInputStream) -> &Self {
        v
    }
}

impl nsIStringInputStream {
    #[inline]
    pub fn coerce<T: nsIStringInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStringInputStream {
    type Target = nsIInputStream;
    #[inline]
    fn deref(&self) -> &nsIInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIInputStreamCoerce> nsIStringInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStringInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStringInputStreamVTable {
    pub __base: nsIInputStreamVTable,

    /* void setData (in string data, in long dataLen); */
    pub setData: unsafe extern "C" fn (this: *const nsIStringInputStream, data: *const libc::c_char, dataLen: libc::int32_t) -> nsresult,

    /* [noscript] void adoptData (in charPtr data, in long dataLen); */
    pub adoptData: unsafe extern "C" fn (this: *const nsIStringInputStream, data: *const u8, dataLen: libc::int32_t) -> nsresult,

    /* [noscript] void shareData (in string data, in long dataLen); */
    pub shareData: unsafe extern "C" fn (this: *const nsIStringInputStream, data: *const libc::c_char, dataLen: libc::int32_t) -> nsresult,

    /* [noscript,notxpcom] size_t SizeOfIncludingThis (in MallocSizeOf aMallocSizeOf); */
    /// Unable to call function as its signature contains a non-rust type
    pub SizeOfIncludingThis: *const ::libc::c_void,

}


impl nsIStringInputStream {
    /* void setData (in string data, in long dataLen); */
    #[inline]
    pub unsafe fn setData(&self, data: *const libc::c_char, dataLen: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setData)(self as *const _, data, dataLen) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void adoptData (in charPtr data, in long dataLen); */
    #[inline]
    pub unsafe fn adoptData(&self, data: *const u8, dataLen: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).adoptData)(self as *const _, data, dataLen) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void shareData (in string data, in long dataLen); */
    #[inline]
    pub unsafe fn shareData(&self, data: *const libc::c_char, dataLen: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).shareData)(self as *const _, data, dataLen) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,notxpcom] size_t SizeOfIncludingThis (in MallocSizeOf aMallocSizeOf); */


}


