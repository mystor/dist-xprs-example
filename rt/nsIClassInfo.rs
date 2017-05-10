//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClassInfo.idl
//


pub mod nsIClassInfo_consts {
    pub const SINGLETON: i64 = 1;
    pub const THREADSAFE: i64 = 2;
    pub const MAIN_THREAD_ONLY: i64 = 4;
    pub const DOM_OBJECT: i64 = 8;
    pub const PLUGIN_OBJECT: i64 = 16;
    pub const SINGLETON_CLASSINFO: i64 = 32;
    pub const CONTENT_NODE: i64 = 64;
    pub const RESERVED: i64 = 2147483648;
}


#[repr(C)]
pub struct nsIClassInfo {
    vtable: *const nsIClassInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIClassInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa60569d7, 0xd401, 0x4677,
            [0xba, 0x63, 0x2a, 0xa5, 0x97, 0x1a, 0xf2, 0x5d])
    }
}

unsafe impl RefCounted for nsIClassInfo {
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
pub trait nsIClassInfoCoerce {
    fn coerce_from(v: &nsIClassInfo) -> &Self;
}

impl nsIClassInfoCoerce for nsIClassInfo {
    #[inline]
    fn coerce_from(v: &nsIClassInfo) -> &Self {
        v
    }
}

impl nsIClassInfo {
    #[inline]
    pub fn coerce<T: nsIClassInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIClassInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIClassInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClassInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIClassInfoVTable {
    pub __base: nsISupportsVTable,

    /* void getInterfaces (out uint32_t count, [array, size_is (count), retval] out nsIIDPtr array); */
    /// Unable to call function as its signature contains a non-rust type
    pub getInterfaces: *const ::libc::c_void,

    /* nsIXPCScriptable getScriptableHelper (); */
    pub getScriptableHelper: unsafe extern "C" fn (this: *const nsIClassInfo, _retval: *mut *const nsIXPCScriptable) -> nsresult,

    /* readonly attribute string contractID; */
    pub get_contractID: unsafe extern "C" fn (this: *const nsIClassInfo, aContractID: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute string classDescription; */
    pub get_classDescription: unsafe extern "C" fn (this: *const nsIClassInfo, aClassDescription: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute nsCIDPtr classID; */
    pub get_classID: unsafe extern "C" fn (this: *const nsIClassInfo, aClassID: *mut *const nsCID) -> nsresult,

    /* readonly attribute uint32_t flags; */
    pub get_flags: unsafe extern "C" fn (this: *const nsIClassInfo, aFlags: *mut uint32_t) -> nsresult,

    /* [noscript] readonly attribute nsCID classIDNoAlloc; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_classIDNoAlloc: *const ::libc::c_void,

}


impl nsIClassInfo {
    /* void getInterfaces (out uint32_t count, [array, size_is (count), retval] out nsIIDPtr array); */


    /* nsIXPCScriptable getScriptableHelper (); */
    #[inline]
    pub unsafe fn getScriptableHelper(&self, ) -> Result<Option<RefPtr<nsIXPCScriptable>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getScriptableHelper)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute string contractID; */
    #[inline]
    pub unsafe fn get_contractID(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_contractID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute string classDescription; */
    #[inline]
    pub unsafe fn get_classDescription(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_classDescription)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsCIDPtr classID; */
    #[inline]
    pub unsafe fn get_classID(&self, ) -> Result<*const nsCID, nsresult> {
        let mut _retval: *const nsCID = ::std::ptr::null();
        match ((*self.vtable).get_classID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t flags; */
    #[inline]
    pub unsafe fn get_flags(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_flags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] readonly attribute nsCID classIDNoAlloc; */


}


