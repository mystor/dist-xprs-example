//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleApplication.idl
//


#[repr(C)]
pub struct nsIAccessibleApplication {
    vtable: *const nsIAccessibleApplicationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleApplication {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x79251626, 0x387c, 0x4531,
            [0x89, 0xf3, 0x68, 0x0d, 0x31, 0xd6, 0xcf, 0x05])
    }
}

unsafe impl RefCounted for nsIAccessibleApplication {
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
pub trait nsIAccessibleApplicationCoerce {
    fn coerce_from(v: &nsIAccessibleApplication) -> &Self;
}

impl nsIAccessibleApplicationCoerce for nsIAccessibleApplication {
    #[inline]
    fn coerce_from(v: &nsIAccessibleApplication) -> &Self {
        v
    }
}

impl nsIAccessibleApplication {
    #[inline]
    pub fn coerce<T: nsIAccessibleApplicationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleApplication {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleApplicationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleApplication) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleApplicationVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString appName; */
    pub get_appName: unsafe extern "C" fn (this: *const nsIAccessibleApplication, aAppName: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString appVersion; */
    pub get_appVersion: unsafe extern "C" fn (this: *const nsIAccessibleApplication, aAppVersion: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString platformName; */
    pub get_platformName: unsafe extern "C" fn (this: *const nsIAccessibleApplication, aPlatformName: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString platformVersion; */
    pub get_platformVersion: unsafe extern "C" fn (this: *const nsIAccessibleApplication, aPlatformVersion: *mut nsAString) -> nsresult,

}


impl nsIAccessibleApplication {
    /* readonly attribute DOMString appName; */
    #[inline]
    pub unsafe fn get_appName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_appName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString appVersion; */
    #[inline]
    pub unsafe fn get_appVersion(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_appVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString platformName; */
    #[inline]
    pub unsafe fn get_platformName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_platformName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString platformVersion; */
    #[inline]
    pub unsafe fn get_platformVersion(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_platformVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


