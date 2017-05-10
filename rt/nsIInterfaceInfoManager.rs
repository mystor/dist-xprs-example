//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIInterfaceInfoManager.idl
//


#[repr(C)]
pub struct nsIInterfaceInfoManager {
    vtable: *const nsIInterfaceInfoManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIInterfaceInfoManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1d53d8d9, 0x1d92, 0x428f,
            [0xb5, 0xcc, 0x19, 0x8b, 0x55, 0xe8, 0x97, 0xd7])
    }
}

unsafe impl RefCounted for nsIInterfaceInfoManager {
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
pub trait nsIInterfaceInfoManagerCoerce {
    fn coerce_from(v: &nsIInterfaceInfoManager) -> &Self;
}

impl nsIInterfaceInfoManagerCoerce for nsIInterfaceInfoManager {
    #[inline]
    fn coerce_from(v: &nsIInterfaceInfoManager) -> &Self {
        v
    }
}

impl nsIInterfaceInfoManager {
    #[inline]
    pub fn coerce<T: nsIInterfaceInfoManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIInterfaceInfoManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIInterfaceInfoManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInterfaceInfoManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIInterfaceInfoManagerVTable {
    pub __base: nsISupportsVTable,

    /* nsIInterfaceInfo getInfoForIID (in nsIIDPtr iid); */
    pub getInfoForIID: unsafe extern "C" fn (this: *const nsIInterfaceInfoManager, iid: *const nsIID, _retval: *mut *const nsIInterfaceInfo) -> nsresult,

    /* nsIInterfaceInfo getInfoForName (in string name); */
    pub getInfoForName: unsafe extern "C" fn (this: *const nsIInterfaceInfoManager, name: *const libc::c_char, _retval: *mut *const nsIInterfaceInfo) -> nsresult,

}


impl nsIInterfaceInfoManager {
    /* nsIInterfaceInfo getInfoForIID (in nsIIDPtr iid); */
    #[inline]
    pub unsafe fn getInfoForIID(&self, iid: *const nsIID) -> Result<Option<RefPtr<nsIInterfaceInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getInfoForIID)(self as *const _, iid, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIInterfaceInfo getInfoForName (in string name); */
    #[inline]
    pub unsafe fn getInfoForName(&self, name: *const libc::c_char) -> Result<Option<RefPtr<nsIInterfaceInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getInfoForName)(self as *const _, name, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


