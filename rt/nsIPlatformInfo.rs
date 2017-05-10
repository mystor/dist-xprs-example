//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPlatformInfo.idl
//


#[repr(C)]
pub struct nsIPlatformInfo {
    vtable: *const nsIPlatformInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPlatformInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xab6650cf, 0x0806, 0x4aea,
            [0xb8, 0xf2, 0x40, 0xfd, 0xae, 0x74, 0xf1, 0xcc])
    }
}

unsafe impl RefCounted for nsIPlatformInfo {
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
pub trait nsIPlatformInfoCoerce {
    fn coerce_from(v: &nsIPlatformInfo) -> &Self;
}

impl nsIPlatformInfoCoerce for nsIPlatformInfo {
    #[inline]
    fn coerce_from(v: &nsIPlatformInfo) -> &Self {
        v
    }
}

impl nsIPlatformInfo {
    #[inline]
    pub fn coerce<T: nsIPlatformInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPlatformInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPlatformInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPlatformInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPlatformInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString platformVersion; */
    pub get_platformVersion: unsafe extern "C" fn (this: *const nsIPlatformInfo, aPlatformVersion: *mut nsACString) -> nsresult,

    /* readonly attribute ACString platformBuildID; */
    pub get_platformBuildID: unsafe extern "C" fn (this: *const nsIPlatformInfo, aPlatformBuildID: *mut nsACString) -> nsresult,

}


impl nsIPlatformInfo {
    /* readonly attribute ACString platformVersion; */
    #[inline]
    pub unsafe fn get_platformVersion(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_platformVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString platformBuildID; */
    #[inline]
    pub unsafe fn get_platformBuildID(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_platformBuildID)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


