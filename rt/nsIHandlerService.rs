//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHandlerService.idl
//


#[repr(C)]
pub struct nsIHandlerService {
    vtable: *const nsIHandlerServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHandlerService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x53f0ad17, 0xec62, 0x46a1,
            [0xad, 0xbc, 0xef, 0xcc, 0xc0, 0x6b, 0xab, 0xcd])
    }
}

unsafe impl RefCounted for nsIHandlerService {
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
pub trait nsIHandlerServiceCoerce {
    fn coerce_from(v: &nsIHandlerService) -> &Self;
}

impl nsIHandlerServiceCoerce for nsIHandlerService {
    #[inline]
    fn coerce_from(v: &nsIHandlerService) -> &Self {
        v
    }
}

impl nsIHandlerService {
    #[inline]
    pub fn coerce<T: nsIHandlerServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHandlerService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHandlerServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHandlerService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHandlerServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsISimpleEnumerator enumerate (); */
    pub enumerate: unsafe extern "C" fn (this: *const nsIHandlerService, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* void fillHandlerInfo (in nsIHandlerInfo aHandlerInfo, in ACString aOverrideType); */
    pub fillHandlerInfo: unsafe extern "C" fn (this: *const nsIHandlerService, aHandlerInfo: *const nsIHandlerInfo, aOverrideType: *const nsACString) -> nsresult,

    /* void store (in nsIHandlerInfo aHandlerInfo); */
    pub store: unsafe extern "C" fn (this: *const nsIHandlerService, aHandlerInfo: *const nsIHandlerInfo) -> nsresult,

    /* boolean exists (in nsIHandlerInfo aHandlerInfo); */
    pub exists: unsafe extern "C" fn (this: *const nsIHandlerService, aHandlerInfo: *const nsIHandlerInfo, _retval: *mut bool) -> nsresult,

    /* void remove (in nsIHandlerInfo aHandlerInfo); */
    pub remove: unsafe extern "C" fn (this: *const nsIHandlerService, aHandlerInfo: *const nsIHandlerInfo) -> nsresult,

    /* ACString getTypeFromExtension (in ACString aFileExtension); */
    pub getTypeFromExtension: unsafe extern "C" fn (this: *const nsIHandlerService, aFileExtension: *const nsACString, _retval: *mut nsACString) -> nsresult,

}


impl nsIHandlerService {
    /* nsISimpleEnumerator enumerate (); */
    #[inline]
    pub unsafe fn enumerate(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).enumerate)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void fillHandlerInfo (in nsIHandlerInfo aHandlerInfo, in ACString aOverrideType); */
    #[inline]
    pub unsafe fn fillHandlerInfo(&self, aHandlerInfo: Option<&nsIHandlerInfo>, aOverrideType: &[u8]) -> Result<(), nsresult> {
        let aOverrideType = nsCString::from(aOverrideType);
        match ((*self.vtable).fillHandlerInfo)(self as *const _, aHandlerInfo.map_or(::std::ptr::null(), |x| x as *const _), &*aOverrideType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void store (in nsIHandlerInfo aHandlerInfo); */
    #[inline]
    pub unsafe fn store(&self, aHandlerInfo: Option<&nsIHandlerInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).store)(self as *const _, aHandlerInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean exists (in nsIHandlerInfo aHandlerInfo); */
    #[inline]
    pub unsafe fn exists(&self, aHandlerInfo: Option<&nsIHandlerInfo>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).exists)(self as *const _, aHandlerInfo.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void remove (in nsIHandlerInfo aHandlerInfo); */
    #[inline]
    pub unsafe fn remove(&self, aHandlerInfo: Option<&nsIHandlerInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).remove)(self as *const _, aHandlerInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* ACString getTypeFromExtension (in ACString aFileExtension); */
    #[inline]
    pub unsafe fn getTypeFromExtension(&self, aFileExtension: &[u8]) -> Result<nsCString, nsresult> {
        let aFileExtension = nsCString::from(aFileExtension);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getTypeFromExtension)(self as *const _, &*aFileExtension, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


