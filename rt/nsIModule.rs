//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIModule.idl
//


#[repr(C)]
pub struct nsIModule {
    vtable: *const nsIModuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIModule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7392d032, 0x5371, 0x11d3,
            [0x99, 0x4e, 0x00, 0x80, 0x5f, 0xd2, 0x6f, 0xee])
    }
}

unsafe impl RefCounted for nsIModule {
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
pub trait nsIModuleCoerce {
    fn coerce_from(v: &nsIModule) -> &Self;
}

impl nsIModuleCoerce for nsIModule {
    #[inline]
    fn coerce_from(v: &nsIModule) -> &Self {
        v
    }
}

impl nsIModule {
    #[inline]
    pub fn coerce<T: nsIModuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIModule {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIModuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIModule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIModuleVTable {
    pub __base: nsISupportsVTable,

    /* void getClassObject (in nsIComponentManager aCompMgr, in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult aResult); */
    pub getClassObject: unsafe extern "C" fn (this: *const nsIModule, aCompMgr: *const nsIComponentManager, aClass: *const nsCID, aIID: *const nsIID, aResult: *mut *const libc::c_void) -> nsresult,

    /* void registerSelf (in nsIComponentManager aCompMgr, in nsIFile aLocation, in string aLoaderStr, in string aType); */
    pub registerSelf: unsafe extern "C" fn (this: *const nsIModule, aCompMgr: *const nsIComponentManager, aLocation: *const nsIFile, aLoaderStr: *const libc::c_char, aType: *const libc::c_char) -> nsresult,

    /* void unregisterSelf (in nsIComponentManager aCompMgr, in nsIFile aLocation, in string aLoaderStr); */
    pub unregisterSelf: unsafe extern "C" fn (this: *const nsIModule, aCompMgr: *const nsIComponentManager, aLocation: *const nsIFile, aLoaderStr: *const libc::c_char) -> nsresult,

    /* boolean canUnload (in nsIComponentManager aCompMgr); */
    pub canUnload: unsafe extern "C" fn (this: *const nsIModule, aCompMgr: *const nsIComponentManager, _retval: *mut bool) -> nsresult,

}


impl nsIModule {
    /* void getClassObject (in nsIComponentManager aCompMgr, in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult aResult); */
    #[inline]
    pub unsafe fn getClassObject<T: XpCom>(&self, aCompMgr: Option<&nsIComponentManager>, aClass: *const nsCID) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut aResult : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).getClassObject)(self as *const _, aCompMgr.map_or(::std::ptr::null(), |x| x as *const _), aClass, &T::iid(), aResult.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aResult.refptr())
    }

    /* void registerSelf (in nsIComponentManager aCompMgr, in nsIFile aLocation, in string aLoaderStr, in string aType); */
    #[inline]
    pub unsafe fn registerSelf(&self, aCompMgr: Option<&nsIComponentManager>, aLocation: Option<&nsIFile>, aLoaderStr: *const libc::c_char, aType: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).registerSelf)(self as *const _, aCompMgr.map_or(::std::ptr::null(), |x| x as *const _), aLocation.map_or(::std::ptr::null(), |x| x as *const _), aLoaderStr, aType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterSelf (in nsIComponentManager aCompMgr, in nsIFile aLocation, in string aLoaderStr); */
    #[inline]
    pub unsafe fn unregisterSelf(&self, aCompMgr: Option<&nsIComponentManager>, aLocation: Option<&nsIFile>, aLoaderStr: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterSelf)(self as *const _, aCompMgr.map_or(::std::ptr::null(), |x| x as *const _), aLocation.map_or(::std::ptr::null(), |x| x as *const _), aLoaderStr) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean canUnload (in nsIComponentManager aCompMgr); */
    #[inline]
    pub unsafe fn canUnload(&self, aCompMgr: Option<&nsIComponentManager>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canUnload)(self as *const _, aCompMgr.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


