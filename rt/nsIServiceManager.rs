//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIServiceManager.idl
//


#[repr(C)]
pub struct nsIServiceManager {
    vtable: *const nsIServiceManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIServiceManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8bb35ed9, 0xe332, 0x462d,
            [0x91, 0x55, 0x4a, 0x00, 0x2a, 0xb5, 0xc9, 0x58])
    }
}

unsafe impl RefCounted for nsIServiceManager {
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
pub trait nsIServiceManagerCoerce {
    fn coerce_from(v: &nsIServiceManager) -> &Self;
}

impl nsIServiceManagerCoerce for nsIServiceManager {
    #[inline]
    fn coerce_from(v: &nsIServiceManager) -> &Self {
        v
    }
}

impl nsIServiceManager {
    #[inline]
    pub fn coerce<T: nsIServiceManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIServiceManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIServiceManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServiceManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIServiceManagerVTable {
    pub __base: nsISupportsVTable,

    /* void getService (in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    pub getService: unsafe extern "C" fn (this: *const nsIServiceManager, aClass: *const nsCID, aIID: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

    /* void getServiceByContractID (in string aContractID, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    pub getServiceByContractID: unsafe extern "C" fn (this: *const nsIServiceManager, aContractID: *const libc::c_char, aIID: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

    /* boolean isServiceInstantiated (in nsCIDRef aClass, in nsIIDRef aIID); */
    pub isServiceInstantiated: unsafe extern "C" fn (this: *const nsIServiceManager, aClass: *const nsCID, aIID: *const nsIID, _retval: *mut bool) -> nsresult,

    /* boolean isServiceInstantiatedByContractID (in string aContractID, in nsIIDRef aIID); */
    pub isServiceInstantiatedByContractID: unsafe extern "C" fn (this: *const nsIServiceManager, aContractID: *const libc::c_char, aIID: *const nsIID, _retval: *mut bool) -> nsresult,

}


impl nsIServiceManager {
    /* void getService (in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn getService<T: XpCom>(&self, aClass: *const nsCID) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).getService)(self as *const _, aClass, &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

    /* void getServiceByContractID (in string aContractID, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn getServiceByContractID<T: XpCom>(&self, aContractID: *const libc::c_char) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).getServiceByContractID)(self as *const _, aContractID, &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

    /* boolean isServiceInstantiated (in nsCIDRef aClass, in nsIIDRef aIID); */
    #[inline]
    pub unsafe fn isServiceInstantiated(&self, aClass: *const nsCID, aIID: *const nsIID) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isServiceInstantiated)(self as *const _, aClass, aIID, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isServiceInstantiatedByContractID (in string aContractID, in nsIIDRef aIID); */
    #[inline]
    pub unsafe fn isServiceInstantiatedByContractID(&self, aContractID: *const libc::c_char, aIID: *const nsIID) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isServiceInstantiatedByContractID)(self as *const _, aContractID, aIID, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


