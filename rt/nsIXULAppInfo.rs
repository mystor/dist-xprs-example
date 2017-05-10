//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULAppInfo.idl
//


#[repr(C)]
pub struct nsIXULAppInfo {
    vtable: *const nsIXULAppInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXULAppInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xddea4f31, 0x3c5e, 0x4769,
            [0xac, 0x68, 0x21, 0xab, 0x4b, 0x3d, 0x78, 0x45])
    }
}

unsafe impl RefCounted for nsIXULAppInfo {
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
pub trait nsIXULAppInfoCoerce {
    fn coerce_from(v: &nsIXULAppInfo) -> &Self;
}

impl nsIXULAppInfoCoerce for nsIXULAppInfo {
    #[inline]
    fn coerce_from(v: &nsIXULAppInfo) -> &Self {
        v
    }
}

impl nsIXULAppInfo {
    #[inline]
    pub fn coerce<T: nsIXULAppInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXULAppInfo {
    type Target = nsIPlatformInfo;
    #[inline]
    fn deref(&self) -> &nsIPlatformInfo {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIPlatformInfoCoerce> nsIXULAppInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULAppInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXULAppInfoVTable {
    pub __base: nsIPlatformInfoVTable,

    /* readonly attribute ACString vendor; */
    pub get_vendor: unsafe extern "C" fn (this: *const nsIXULAppInfo, aVendor: *mut nsACString) -> nsresult,

    /* readonly attribute ACString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIXULAppInfo, aName: *mut nsACString) -> nsresult,

    /* readonly attribute ACString ID; */
    pub get_ID: unsafe extern "C" fn (this: *const nsIXULAppInfo, aID: *mut nsACString) -> nsresult,

    /* readonly attribute ACString version; */
    pub get_version: unsafe extern "C" fn (this: *const nsIXULAppInfo, aVersion: *mut nsACString) -> nsresult,

    /* readonly attribute ACString appBuildID; */
    pub get_appBuildID: unsafe extern "C" fn (this: *const nsIXULAppInfo, aAppBuildID: *mut nsACString) -> nsresult,

    /* readonly attribute ACString UAName; */
    pub get_UAName: unsafe extern "C" fn (this: *const nsIXULAppInfo, aUAName: *mut nsACString) -> nsresult,

}


impl nsIXULAppInfo {
    /* readonly attribute ACString vendor; */
    #[inline]
    pub unsafe fn get_vendor(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_vendor)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

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

    /* readonly attribute ACString ID; */
    #[inline]
    pub unsafe fn get_ID(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_ID)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString version; */
    #[inline]
    pub unsafe fn get_version(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_version)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString appBuildID; */
    #[inline]
    pub unsafe fn get_appBuildID(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_appBuildID)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString UAName; */
    #[inline]
    pub unsafe fn get_UAName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_UAName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


