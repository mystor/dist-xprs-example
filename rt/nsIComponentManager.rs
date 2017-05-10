//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIComponentManager.idl
//


#[repr(C)]
pub struct nsIComponentManager {
    vtable: *const nsIComponentManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIComponentManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd604ffc3, 0x1ba3, 0x4f6c,
            [0xb6, 0x5f, 0x1e, 0xd4, 0x19, 0x93, 0x64, 0xc3])
    }
}

unsafe impl RefCounted for nsIComponentManager {
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
pub trait nsIComponentManagerCoerce {
    fn coerce_from(v: &nsIComponentManager) -> &Self;
}

impl nsIComponentManagerCoerce for nsIComponentManager {
    #[inline]
    fn coerce_from(v: &nsIComponentManager) -> &Self {
        v
    }
}

impl nsIComponentManager {
    #[inline]
    pub fn coerce<T: nsIComponentManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIComponentManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIComponentManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIComponentManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIComponentManagerVTable {
    pub __base: nsISupportsVTable,

    /* void getClassObject (in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    pub getClassObject: unsafe extern "C" fn (this: *const nsIComponentManager, aClass: *const nsCID, aIID: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

    /* void getClassObjectByContractID (in string aContractID, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    pub getClassObjectByContractID: unsafe extern "C" fn (this: *const nsIComponentManager, aContractID: *const libc::c_char, aIID: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

    /* void createInstance (in nsCIDRef aClass, in nsISupports aDelegate, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    pub createInstance: unsafe extern "C" fn (this: *const nsIComponentManager, aClass: *const nsCID, aDelegate: *const nsISupports, aIID: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

    /* void createInstanceByContractID (in string aContractID, in nsISupports aDelegate, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    pub createInstanceByContractID: unsafe extern "C" fn (this: *const nsIComponentManager, aContractID: *const libc::c_char, aDelegate: *const nsISupports, aIID: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

    /* void addBootstrappedManifestLocation (in nsIFile aLocation); */
    pub addBootstrappedManifestLocation: unsafe extern "C" fn (this: *const nsIComponentManager, aLocation: *const nsIFile) -> nsresult,

    /* void removeBootstrappedManifestLocation (in nsIFile aLocation); */
    pub removeBootstrappedManifestLocation: unsafe extern "C" fn (this: *const nsIComponentManager, aLocation: *const nsIFile) -> nsresult,

    /* nsIArray getManifestLocations (); */
    pub getManifestLocations: unsafe extern "C" fn (this: *const nsIComponentManager, _retval: *mut *const nsIArray) -> nsresult,

}


impl nsIComponentManager {
    /* void getClassObject (in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn getClassObject<T: XpCom>(&self, aClass: *const nsCID) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).getClassObject)(self as *const _, aClass, &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

    /* void getClassObjectByContractID (in string aContractID, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn getClassObjectByContractID<T: XpCom>(&self, aContractID: *const libc::c_char) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).getClassObjectByContractID)(self as *const _, aContractID, &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

    /* void createInstance (in nsCIDRef aClass, in nsISupports aDelegate, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn createInstance<T: XpCom>(&self, aClass: *const nsCID, aDelegate: Option<&nsISupports>) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).createInstance)(self as *const _, aClass, aDelegate.map_or(::std::ptr::null(), |x| x as *const _), &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

    /* void createInstanceByContractID (in string aContractID, in nsISupports aDelegate, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn createInstanceByContractID<T: XpCom>(&self, aContractID: *const libc::c_char, aDelegate: Option<&nsISupports>) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).createInstanceByContractID)(self as *const _, aContractID, aDelegate.map_or(::std::ptr::null(), |x| x as *const _), &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

    /* void addBootstrappedManifestLocation (in nsIFile aLocation); */
    #[inline]
    pub unsafe fn addBootstrappedManifestLocation(&self, aLocation: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).addBootstrappedManifestLocation)(self as *const _, aLocation.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeBootstrappedManifestLocation (in nsIFile aLocation); */
    #[inline]
    pub unsafe fn removeBootstrappedManifestLocation(&self, aLocation: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).removeBootstrappedManifestLocation)(self as *const _, aLocation.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIArray getManifestLocations (); */
    #[inline]
    pub unsafe fn getManifestLocations(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getManifestLocations)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


