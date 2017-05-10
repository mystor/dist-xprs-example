//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentProcess.idl
//


#[repr(C)]
pub struct nsIContentProcessInfo {
    vtable: *const nsIContentProcessInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentProcessInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x456f58be, 0x29dd, 0x4973,
            [0x88, 0x5b, 0x95, 0xae, 0xce, 0x1c, 0x9a, 0x8a])
    }
}

unsafe impl RefCounted for nsIContentProcessInfo {
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
pub trait nsIContentProcessInfoCoerce {
    fn coerce_from(v: &nsIContentProcessInfo) -> &Self;
}

impl nsIContentProcessInfoCoerce for nsIContentProcessInfo {
    #[inline]
    fn coerce_from(v: &nsIContentProcessInfo) -> &Self {
        v
    }
}

impl nsIContentProcessInfo {
    #[inline]
    pub fn coerce<T: nsIContentProcessInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentProcessInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentProcessInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentProcessInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentProcessInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isAlive; */
    pub get_isAlive: unsafe extern "C" fn (this: *const nsIContentProcessInfo, aIsAlive: *mut bool) -> nsresult,

    /* readonly attribute int32_t processId; */
    pub get_processId: unsafe extern "C" fn (this: *const nsIContentProcessInfo, aProcessId: *mut int32_t) -> nsresult,

    /* readonly attribute nsIContentProcessInfo opener; */
    pub get_opener: unsafe extern "C" fn (this: *const nsIContentProcessInfo, aOpener: *mut *const nsIContentProcessInfo) -> nsresult,

    /* readonly attribute int32_t tabCount; */
    pub get_tabCount: unsafe extern "C" fn (this: *const nsIContentProcessInfo, aTabCount: *mut int32_t) -> nsresult,

    /* readonly attribute nsIMessageSender messageManager; */
    pub get_messageManager: unsafe extern "C" fn (this: *const nsIContentProcessInfo, aMessageManager: *mut *const nsIMessageSender) -> nsresult,

}


impl nsIContentProcessInfo {
    /* readonly attribute boolean isAlive; */
    #[inline]
    pub unsafe fn get_isAlive(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isAlive)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute int32_t processId; */
    #[inline]
    pub unsafe fn get_processId(&self, ) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_processId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIContentProcessInfo opener; */
    #[inline]
    pub unsafe fn get_opener(&self, ) -> Result<Option<RefPtr<nsIContentProcessInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_opener)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute int32_t tabCount; */
    #[inline]
    pub unsafe fn get_tabCount(&self, ) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_tabCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIMessageSender messageManager; */
    #[inline]
    pub unsafe fn get_messageManager(&self, ) -> Result<Option<RefPtr<nsIMessageSender>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_messageManager)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


pub mod nsIContentProcessProvider_consts {
    pub const NEW_PROCESS: i64 = -1;
}


#[repr(C)]
pub struct nsIContentProcessProvider {
    vtable: *const nsIContentProcessProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentProcessProvider {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x83ffb063, 0x5f65, 0x4c45,
            [0xae, 0x07, 0x3f, 0x55, 0x3e, 0x08, 0x09, 0xbb])
    }
}

unsafe impl RefCounted for nsIContentProcessProvider {
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
pub trait nsIContentProcessProviderCoerce {
    fn coerce_from(v: &nsIContentProcessProvider) -> &Self;
}

impl nsIContentProcessProviderCoerce for nsIContentProcessProvider {
    #[inline]
    fn coerce_from(v: &nsIContentProcessProvider) -> &Self {
        v
    }
}

impl nsIContentProcessProvider {
    #[inline]
    pub fn coerce<T: nsIContentProcessProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentProcessProvider {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentProcessProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentProcessProvider) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentProcessProviderVTable {
    pub __base: nsISupportsVTable,

    /* int32_t provideProcess (in AString aType, in nsIContentProcessInfo aOpener, [array, size_is (aCount)] in nsIContentProcessInfo aAliveProcesses, in uint32_t aCount, in uint32_t aMaxCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub provideProcess: *const ::libc::c_void,

}


impl nsIContentProcessProvider {
    /* int32_t provideProcess (in AString aType, in nsIContentProcessInfo aOpener, [array, size_is (aCount)] in nsIContentProcessInfo aAliveProcesses, in uint32_t aCount, in uint32_t aMaxCount); */


}


