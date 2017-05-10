//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIDNService.idl
//


#[repr(C)]
pub struct nsIIDNService {
    vtable: *const nsIIDNServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIDNService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa592a60e, 0x3621, 0x4f19,
            [0xa3, 0x18, 0x2b, 0xf2, 0x33, 0xcf, 0xad, 0x3e])
    }
}

unsafe impl RefCounted for nsIIDNService {
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
pub trait nsIIDNServiceCoerce {
    fn coerce_from(v: &nsIIDNService) -> &Self;
}

impl nsIIDNServiceCoerce for nsIIDNService {
    #[inline]
    fn coerce_from(v: &nsIIDNService) -> &Self {
        v
    }
}

impl nsIIDNService {
    #[inline]
    pub fn coerce<T: nsIIDNServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIDNService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIIDNServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIDNService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIDNServiceVTable {
    pub __base: nsISupportsVTable,

    /* ACString convertUTF8toACE (in AUTF8String input); */
    pub convertUTF8toACE: unsafe extern "C" fn (this: *const nsIIDNService, input: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* AUTF8String convertACEtoUTF8 (in ACString input); */
    pub convertACEtoUTF8: unsafe extern "C" fn (this: *const nsIIDNService, input: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* boolean isACE (in ACString input); */
    pub isACE: unsafe extern "C" fn (this: *const nsIIDNService, input: *const nsACString, _retval: *mut bool) -> nsresult,

    /* AUTF8String normalize (in AUTF8String input); */
    pub normalize: unsafe extern "C" fn (this: *const nsIIDNService, input: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* AUTF8String convertToDisplayIDN (in AUTF8String input, out boolean isASCII); */
    pub convertToDisplayIDN: unsafe extern "C" fn (this: *const nsIIDNService, input: *const nsACString, isASCII: *mut bool, _retval: *mut nsACString) -> nsresult,

}


impl nsIIDNService {
    /* ACString convertUTF8toACE (in AUTF8String input); */
    #[inline]
    pub unsafe fn convertUTF8toACE(&self, input: &[u8]) -> Result<nsCString, nsresult> {
        let input = nsCString::from(input);
        let mut _retval = nsCString::new();
        match ((*self.vtable).convertUTF8toACE)(self as *const _, &*input, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String convertACEtoUTF8 (in ACString input); */
    #[inline]
    pub unsafe fn convertACEtoUTF8(&self, input: &[u8]) -> Result<nsCString, nsresult> {
        let input = nsCString::from(input);
        let mut _retval = nsCString::new();
        match ((*self.vtable).convertACEtoUTF8)(self as *const _, &*input, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isACE (in ACString input); */
    #[inline]
    pub unsafe fn isACE(&self, input: &[u8]) -> Result<bool, nsresult> {
        let input = nsCString::from(input);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isACE)(self as *const _, &*input, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String normalize (in AUTF8String input); */
    #[inline]
    pub unsafe fn normalize(&self, input: &[u8]) -> Result<nsCString, nsresult> {
        let input = nsCString::from(input);
        let mut _retval = nsCString::new();
        match ((*self.vtable).normalize)(self as *const _, &*input, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String convertToDisplayIDN (in AUTF8String input, out boolean isASCII); */
    #[inline]
    pub unsafe fn convertToDisplayIDN(&self, input: &[u8]) -> Result<(bool, nsCString), nsresult> {
        let input = nsCString::from(input);
        let mut isASCII: bool = ::std::mem::zeroed();
        let mut _retval = nsCString::new();
        match ((*self.vtable).convertToDisplayIDN)(self as *const _, &*input, &mut isASCII as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((isASCII, _retval))
    }

}


