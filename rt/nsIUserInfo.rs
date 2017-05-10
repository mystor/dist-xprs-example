//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUserInfo.idl
//


#[repr(C)]
pub struct nsIUserInfo {
    vtable: *const nsIUserInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUserInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6c1034f0, 0x1dd2, 0x11b2,
            [0xaa, 0x14, 0xe6, 0x65, 0x7e, 0xd7, 0xbb, 0x0b])
    }
}

unsafe impl RefCounted for nsIUserInfo {
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
pub trait nsIUserInfoCoerce {
    fn coerce_from(v: &nsIUserInfo) -> &Self;
}

impl nsIUserInfoCoerce for nsIUserInfo {
    #[inline]
    fn coerce_from(v: &nsIUserInfo) -> &Self {
        v
    }
}

impl nsIUserInfo {
    #[inline]
    pub fn coerce<T: nsIUserInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUserInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUserInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUserInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUserInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute wstring fullname; */
    pub get_fullname: unsafe extern "C" fn (this: *const nsIUserInfo, aFullname: *mut *const libc::int16_t) -> nsresult,

    /* readonly attribute string emailAddress; */
    pub get_emailAddress: unsafe extern "C" fn (this: *const nsIUserInfo, aEmailAddress: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute string username; */
    pub get_username: unsafe extern "C" fn (this: *const nsIUserInfo, aUsername: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute string domain; */
    pub get_domain: unsafe extern "C" fn (this: *const nsIUserInfo, aDomain: *mut *const libc::c_char) -> nsresult,

}


impl nsIUserInfo {
    /* readonly attribute wstring fullname; */
    #[inline]
    pub unsafe fn get_fullname(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_fullname)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute string emailAddress; */
    #[inline]
    pub unsafe fn get_emailAddress(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_emailAddress)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute string username; */
    #[inline]
    pub unsafe fn get_username(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_username)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute string domain; */
    #[inline]
    pub unsafe fn get_domain(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_domain)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


