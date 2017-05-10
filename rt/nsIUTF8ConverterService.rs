//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUTF8ConverterService.idl
//


#[repr(C)]
pub struct nsIUTF8ConverterService {
    vtable: *const nsIUTF8ConverterServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUTF8ConverterService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x249f52a3, 0x2599, 0x4b00,
            [0xba, 0x40, 0x04, 0x81, 0x36, 0x48, 0x31, 0xa2])
    }
}

unsafe impl RefCounted for nsIUTF8ConverterService {
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
pub trait nsIUTF8ConverterServiceCoerce {
    fn coerce_from(v: &nsIUTF8ConverterService) -> &Self;
}

impl nsIUTF8ConverterServiceCoerce for nsIUTF8ConverterService {
    #[inline]
    fn coerce_from(v: &nsIUTF8ConverterService) -> &Self {
        v
    }
}

impl nsIUTF8ConverterService {
    #[inline]
    pub fn coerce<T: nsIUTF8ConverterServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUTF8ConverterService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUTF8ConverterServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUTF8ConverterService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUTF8ConverterServiceVTable {
    pub __base: nsISupportsVTable,

    /* [optional_argc] AUTF8String convertStringToUTF8 (in ACString aString, in string aCharset, in boolean aSkipCheck, [optional] in boolean aAllowSubstitution); */
    /// Unable to call function as its signature contains a non-rust type
    pub convertStringToUTF8: *const ::libc::c_void,

    /* AUTF8String convertURISpecToUTF8 (in ACString aSpec, in string aCharset); */
    pub convertURISpecToUTF8: unsafe extern "C" fn (this: *const nsIUTF8ConverterService, aSpec: *const nsACString, aCharset: *const libc::c_char, _retval: *mut nsACString) -> nsresult,

}


impl nsIUTF8ConverterService {
    /* [optional_argc] AUTF8String convertStringToUTF8 (in ACString aString, in string aCharset, in boolean aSkipCheck, [optional] in boolean aAllowSubstitution); */


    /* AUTF8String convertURISpecToUTF8 (in ACString aSpec, in string aCharset); */
    #[inline]
    pub unsafe fn convertURISpecToUTF8(&self, aSpec: &[u8], aCharset: *const libc::c_char) -> Result<nsCString, nsresult> {
        let aSpec = nsCString::from(aSpec);
        let mut _retval = nsCString::new();
        match ((*self.vtable).convertURISpecToUTF8)(self as *const _, &*aSpec, aCharset, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


