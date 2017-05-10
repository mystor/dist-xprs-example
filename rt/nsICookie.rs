//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICookie.idl
//


pub type nsCookieStatus = libc::int32_t;


pub type nsCookiePolicy = libc::int32_t;


pub mod nsICookie_consts {
    pub const STATUS_UNKNOWN: i64 = 0;
    pub const STATUS_ACCEPTED: i64 = 1;
    pub const STATUS_DOWNGRADED: i64 = 2;
    pub const STATUS_FLAGGED: i64 = 3;
    pub const STATUS_REJECTED: i64 = 4;
    pub const POLICY_UNKNOWN: i64 = 0;
    pub const POLICY_NONE: i64 = 1;
    pub const POLICY_NO_CONSENT: i64 = 2;
    pub const POLICY_IMPLICIT_CONSENT: i64 = 3;
    pub const POLICY_EXPLICIT_CONSENT: i64 = 4;
    pub const POLICY_NO_II: i64 = 5;
}


#[repr(C)]
pub struct nsICookie {
    vtable: *const nsICookieVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICookie {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xadf0db5e, 0x211e, 0x45a3,
            [0xbe, 0x14, 0x44, 0x86, 0xac, 0x43, 0x0a, 0x58])
    }
}

unsafe impl RefCounted for nsICookie {
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
pub trait nsICookieCoerce {
    fn coerce_from(v: &nsICookie) -> &Self;
}

impl nsICookieCoerce for nsICookie {
    #[inline]
    fn coerce_from(v: &nsICookie) -> &Self {
        v
    }
}

impl nsICookie {
    #[inline]
    pub fn coerce<T: nsICookieCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICookie {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICookieCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICookie) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICookieVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsICookie, aName: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String value; */
    pub get_value: unsafe extern "C" fn (this: *const nsICookie, aValue: *mut nsACString) -> nsresult,

    /* readonly attribute boolean isDomain; */
    pub get_isDomain: unsafe extern "C" fn (this: *const nsICookie, aIsDomain: *mut bool) -> nsresult,

    /* readonly attribute AUTF8String host; */
    pub get_host: unsafe extern "C" fn (this: *const nsICookie, aHost: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String path; */
    pub get_path: unsafe extern "C" fn (this: *const nsICookie, aPath: *mut nsACString) -> nsresult,

    /* readonly attribute boolean isSecure; */
    pub get_isSecure: unsafe extern "C" fn (this: *const nsICookie, aIsSecure: *mut bool) -> nsresult,

    /* readonly attribute uint64_t expires; */
    pub get_expires: unsafe extern "C" fn (this: *const nsICookie, aExpires: *mut uint64_t) -> nsresult,

    /* readonly attribute nsCookieStatus status; */
    pub get_status: unsafe extern "C" fn (this: *const nsICookie, aStatus: *mut nsCookieStatus) -> nsresult,

    /* readonly attribute nsCookiePolicy policy; */
    pub get_policy: unsafe extern "C" fn (this: *const nsICookie, aPolicy: *mut nsCookiePolicy) -> nsresult,

    /* [implicit_jscontext] readonly attribute jsval originAttributes; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_originAttributes: *const ::libc::c_void,

}


impl nsICookie {
    /* readonly attribute ACString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String value; */
    #[inline]
    pub unsafe fn get_value(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_value)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isDomain; */
    #[inline]
    pub unsafe fn get_isDomain(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isDomain)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String host; */
    #[inline]
    pub unsafe fn get_host(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_host)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String path; */
    #[inline]
    pub unsafe fn get_path(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_path)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isSecure; */
    #[inline]
    pub unsafe fn get_isSecure(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSecure)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint64_t expires; */
    #[inline]
    pub unsafe fn get_expires(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_expires)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsCookieStatus status; */
    #[inline]
    pub unsafe fn get_status(&self, ) -> Result<nsCookieStatus, nsresult> {
        let mut _retval: nsCookieStatus = ::std::mem::zeroed();
        match ((*self.vtable).get_status)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsCookiePolicy policy; */
    #[inline]
    pub unsafe fn get_policy(&self, ) -> Result<nsCookiePolicy, nsresult> {
        let mut _retval: nsCookiePolicy = ::std::mem::zeroed();
        match ((*self.vtable).get_policy)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] readonly attribute jsval originAttributes; */


}


