//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIComponentRegistrar.idl
//


#[repr(C)]
pub struct nsIComponentRegistrar {
    vtable: *const nsIComponentRegistrarVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIComponentRegistrar {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2417cbfe, 0x65ad, 0x48a6,
            [0xb4, 0xb6, 0xeb, 0x84, 0xdb, 0x17, 0x43, 0x92])
    }
}

unsafe impl RefCounted for nsIComponentRegistrar {
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
pub trait nsIComponentRegistrarCoerce {
    fn coerce_from(v: &nsIComponentRegistrar) -> &Self;
}

impl nsIComponentRegistrarCoerce for nsIComponentRegistrar {
    #[inline]
    fn coerce_from(v: &nsIComponentRegistrar) -> &Self {
        v
    }
}

impl nsIComponentRegistrar {
    #[inline]
    pub fn coerce<T: nsIComponentRegistrarCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIComponentRegistrar {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIComponentRegistrarCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIComponentRegistrar) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIComponentRegistrarVTable {
    pub __base: nsISupportsVTable,

    /* void autoRegister (in nsIFile aSpec); */
    pub autoRegister: unsafe extern "C" fn (this: *const nsIComponentRegistrar, aSpec: *const nsIFile) -> nsresult,

    /* void autoUnregister (in nsIFile aSpec); */
    pub autoUnregister: unsafe extern "C" fn (this: *const nsIComponentRegistrar, aSpec: *const nsIFile) -> nsresult,

    /* void registerFactory (in nsCIDRef aClass, in string aClassName, in string aContractID, in nsIFactory aFactory); */
    pub registerFactory: unsafe extern "C" fn (this: *const nsIComponentRegistrar, aClass: *const nsCID, aClassName: *const libc::c_char, aContractID: *const libc::c_char, aFactory: *const nsIFactory) -> nsresult,

    /* void unregisterFactory (in nsCIDRef aClass, in nsIFactory aFactory); */
    pub unregisterFactory: unsafe extern "C" fn (this: *const nsIComponentRegistrar, aClass: *const nsCID, aFactory: *const nsIFactory) -> nsresult,

    /* void registerFactoryLocation (in nsCIDRef aClass, in string aClassName, in string aContractID, in nsIFile aFile, in string aLoaderStr, in string aType); */
    pub registerFactoryLocation: unsafe extern "C" fn (this: *const nsIComponentRegistrar, aClass: *const nsCID, aClassName: *const libc::c_char, aContractID: *const libc::c_char, aFile: *const nsIFile, aLoaderStr: *const libc::c_char, aType: *const libc::c_char) -> nsresult,

    /* void unregisterFactoryLocation (in nsCIDRef aClass, in nsIFile aFile); */
    pub unregisterFactoryLocation: unsafe extern "C" fn (this: *const nsIComponentRegistrar, aClass: *const nsCID, aFile: *const nsIFile) -> nsresult,

    /* boolean isCIDRegistered (in nsCIDRef aClass); */
    pub isCIDRegistered: unsafe extern "C" fn (this: *const nsIComponentRegistrar, aClass: *const nsCID, _retval: *mut bool) -> nsresult,

    /* boolean isContractIDRegistered (in string aContractID); */
    pub isContractIDRegistered: unsafe extern "C" fn (this: *const nsIComponentRegistrar, aContractID: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* nsISimpleEnumerator enumerateCIDs (); */
    pub enumerateCIDs: unsafe extern "C" fn (this: *const nsIComponentRegistrar, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* nsISimpleEnumerator enumerateContractIDs (); */
    pub enumerateContractIDs: unsafe extern "C" fn (this: *const nsIComponentRegistrar, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* string CIDToContractID (in nsCIDRef aClass); */
    pub CIDToContractID: unsafe extern "C" fn (this: *const nsIComponentRegistrar, aClass: *const nsCID, _retval: *mut *const libc::c_char) -> nsresult,

    /* nsCIDPtr contractIDToCID (in string aContractID); */
    pub contractIDToCID: unsafe extern "C" fn (this: *const nsIComponentRegistrar, aContractID: *const libc::c_char, _retval: *mut *const nsCID) -> nsresult,

}


impl nsIComponentRegistrar {
    /* void autoRegister (in nsIFile aSpec); */
    #[inline]
    pub unsafe fn autoRegister(&self, aSpec: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).autoRegister)(self as *const _, aSpec.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void autoUnregister (in nsIFile aSpec); */
    #[inline]
    pub unsafe fn autoUnregister(&self, aSpec: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).autoUnregister)(self as *const _, aSpec.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerFactory (in nsCIDRef aClass, in string aClassName, in string aContractID, in nsIFactory aFactory); */
    #[inline]
    pub unsafe fn registerFactory(&self, aClass: *const nsCID, aClassName: *const libc::c_char, aContractID: *const libc::c_char, aFactory: Option<&nsIFactory>) -> Result<(), nsresult> {

        match ((*self.vtable).registerFactory)(self as *const _, aClass, aClassName, aContractID, aFactory.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterFactory (in nsCIDRef aClass, in nsIFactory aFactory); */
    #[inline]
    pub unsafe fn unregisterFactory(&self, aClass: *const nsCID, aFactory: Option<&nsIFactory>) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterFactory)(self as *const _, aClass, aFactory.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerFactoryLocation (in nsCIDRef aClass, in string aClassName, in string aContractID, in nsIFile aFile, in string aLoaderStr, in string aType); */
    #[inline]
    pub unsafe fn registerFactoryLocation(&self, aClass: *const nsCID, aClassName: *const libc::c_char, aContractID: *const libc::c_char, aFile: Option<&nsIFile>, aLoaderStr: *const libc::c_char, aType: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).registerFactoryLocation)(self as *const _, aClass, aClassName, aContractID, aFile.map_or(::std::ptr::null(), |x| x as *const _), aLoaderStr, aType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterFactoryLocation (in nsCIDRef aClass, in nsIFile aFile); */
    #[inline]
    pub unsafe fn unregisterFactoryLocation(&self, aClass: *const nsCID, aFile: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterFactoryLocation)(self as *const _, aClass, aFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isCIDRegistered (in nsCIDRef aClass); */
    #[inline]
    pub unsafe fn isCIDRegistered(&self, aClass: *const nsCID) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isCIDRegistered)(self as *const _, aClass, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isContractIDRegistered (in string aContractID); */
    #[inline]
    pub unsafe fn isContractIDRegistered(&self, aContractID: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isContractIDRegistered)(self as *const _, aContractID, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISimpleEnumerator enumerateCIDs (); */
    #[inline]
    pub unsafe fn enumerateCIDs(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).enumerateCIDs)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISimpleEnumerator enumerateContractIDs (); */
    #[inline]
    pub unsafe fn enumerateContractIDs(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).enumerateContractIDs)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* string CIDToContractID (in nsCIDRef aClass); */
    #[inline]
    pub unsafe fn CIDToContractID(&self, aClass: *const nsCID) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).CIDToContractID)(self as *const _, aClass, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsCIDPtr contractIDToCID (in string aContractID); */
    #[inline]
    pub unsafe fn contractIDToCID(&self, aContractID: *const libc::c_char) -> Result<*const nsCID, nsresult> {
        let mut _retval: *const nsCID = ::std::ptr::null();
        match ((*self.vtable).contractIDToCID)(self as *const _, aContractID, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


