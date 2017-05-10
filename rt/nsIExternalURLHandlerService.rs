//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIExternalURLHandlerService.idl
//


#[repr(C)]
pub struct nsIExternalURLHandlerService {
    vtable: *const nsIExternalURLHandlerServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIExternalURLHandlerService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x56c5c7d3, 0x6fd3, 0x43f8,
            [0x94, 0x29, 0x43, 0x97, 0xe1, 0x11, 0x45, 0x3a])
    }
}

unsafe impl RefCounted for nsIExternalURLHandlerService {
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
pub trait nsIExternalURLHandlerServiceCoerce {
    fn coerce_from(v: &nsIExternalURLHandlerService) -> &Self;
}

impl nsIExternalURLHandlerServiceCoerce for nsIExternalURLHandlerService {
    #[inline]
    fn coerce_from(v: &nsIExternalURLHandlerService) -> &Self {
        v
    }
}

impl nsIExternalURLHandlerService {
    #[inline]
    pub fn coerce<T: nsIExternalURLHandlerServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIExternalURLHandlerService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIExternalURLHandlerServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIExternalURLHandlerService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIExternalURLHandlerServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIHandlerInfo getURLHandlerInfoFromOS (in nsIURI aURL, out boolean aFound); */
    pub getURLHandlerInfoFromOS: unsafe extern "C" fn (this: *const nsIExternalURLHandlerService, aURL: *const nsIURI, aFound: *mut bool, _retval: *mut *const nsIHandlerInfo) -> nsresult,

}


impl nsIExternalURLHandlerService {
    /* nsIHandlerInfo getURLHandlerInfoFromOS (in nsIURI aURL, out boolean aFound); */
    #[inline]
    pub unsafe fn getURLHandlerInfoFromOS(&self, aURL: Option<&nsIURI>) -> Result<(bool, Option<RefPtr<nsIHandlerInfo>>), nsresult> {
        let mut aFound: bool = ::std::mem::zeroed();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getURLHandlerInfoFromOS)(self as *const _, aURL.map_or(::std::ptr::null(), |x| x as *const _), &mut aFound as *mut _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aFound, _retval.refptr()))
    }

}


