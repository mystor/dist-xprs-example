//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptableBase64Encoder.idl
//


#[repr(C)]
pub struct nsIScriptableBase64Encoder {
    vtable: *const nsIScriptableBase64EncoderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIScriptableBase64Encoder {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9479c864, 0xd1f9, 0x45ab,
            [0xb7, 0xb9, 0x28, 0xb9, 0x07, 0xbd, 0x2b, 0xa9])
    }
}

unsafe impl RefCounted for nsIScriptableBase64Encoder {
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
pub trait nsIScriptableBase64EncoderCoerce {
    fn coerce_from(v: &nsIScriptableBase64Encoder) -> &Self;
}

impl nsIScriptableBase64EncoderCoerce for nsIScriptableBase64Encoder {
    #[inline]
    fn coerce_from(v: &nsIScriptableBase64Encoder) -> &Self {
        v
    }
}

impl nsIScriptableBase64Encoder {
    #[inline]
    pub fn coerce<T: nsIScriptableBase64EncoderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIScriptableBase64Encoder {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIScriptableBase64EncoderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptableBase64Encoder) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIScriptableBase64EncoderVTable {
    pub __base: nsISupportsVTable,

    /* ACString encodeToCString (in nsIInputStream stream, in unsigned long length); */
    pub encodeToCString: unsafe extern "C" fn (this: *const nsIScriptableBase64Encoder, stream: *const nsIInputStream, length: libc::uint32_t, _retval: *mut nsACString) -> nsresult,

    /* AString encodeToString (in nsIInputStream stream, in unsigned long length); */
    pub encodeToString: unsafe extern "C" fn (this: *const nsIScriptableBase64Encoder, stream: *const nsIInputStream, length: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

}


impl nsIScriptableBase64Encoder {
    /* ACString encodeToCString (in nsIInputStream stream, in unsigned long length); */
    #[inline]
    pub unsafe fn encodeToCString(&self, stream: Option<&nsIInputStream>, length: libc::uint32_t) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).encodeToCString)(self as *const _, stream.map_or(::std::ptr::null(), |x| x as *const _), length, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString encodeToString (in nsIInputStream stream, in unsigned long length); */
    #[inline]
    pub unsafe fn encodeToString(&self, stream: Option<&nsIInputStream>, length: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).encodeToString)(self as *const _, stream.map_or(::std::ptr::null(), |x| x as *const _), length, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


