//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMacWebAppUtils.idl
//


#[repr(C)]
pub struct nsITrashAppCallback {
    vtable: *const nsITrashAppCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITrashAppCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8c899c4f, 0x58c1, 0x4b74,
            [0x90, 0x34, 0x3b, 0xb6, 0x4e, 0x48, 0x4b, 0x68])
    }
}

unsafe impl RefCounted for nsITrashAppCallback {
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
pub trait nsITrashAppCallbackCoerce {
    fn coerce_from(v: &nsITrashAppCallback) -> &Self;
}

impl nsITrashAppCallbackCoerce for nsITrashAppCallback {
    #[inline]
    fn coerce_from(v: &nsITrashAppCallback) -> &Self {
        v
    }
}

impl nsITrashAppCallback {
    #[inline]
    pub fn coerce<T: nsITrashAppCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITrashAppCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITrashAppCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITrashAppCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITrashAppCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void trashAppFinished (in nsresult rv); */
    pub trashAppFinished: unsafe extern "C" fn (this: *const nsITrashAppCallback, rv: nsresult) -> nsresult,

}


impl nsITrashAppCallback {
    /* void trashAppFinished (in nsresult rv); */
    #[inline]
    pub unsafe fn trashAppFinished(&self, rv: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).trashAppFinished)(self as *const _, rv) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIMacWebAppUtils {
    vtable: *const nsIMacWebAppUtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMacWebAppUtils {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc69cf343, 0xea41, 0x428b,
            [0xb1, 0x61, 0x46, 0x55, 0xfd, 0x54, 0xd8, 0xe7])
    }
}

unsafe impl RefCounted for nsIMacWebAppUtils {
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
pub trait nsIMacWebAppUtilsCoerce {
    fn coerce_from(v: &nsIMacWebAppUtils) -> &Self;
}

impl nsIMacWebAppUtilsCoerce for nsIMacWebAppUtils {
    #[inline]
    fn coerce_from(v: &nsIMacWebAppUtils) -> &Self {
        v
    }
}

impl nsIMacWebAppUtils {
    #[inline]
    pub fn coerce<T: nsIMacWebAppUtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMacWebAppUtils {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMacWebAppUtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMacWebAppUtils) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMacWebAppUtilsVTable {
    pub __base: nsISupportsVTable,

    /* AString pathForAppWithIdentifier (in AString bundleIdentifier); */
    pub pathForAppWithIdentifier: unsafe extern "C" fn (this: *const nsIMacWebAppUtils, bundleIdentifier: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* void launchAppWithIdentifier (in AString bundleIdentifier); */
    pub launchAppWithIdentifier: unsafe extern "C" fn (this: *const nsIMacWebAppUtils, bundleIdentifier: *const nsAString) -> nsresult,

    /* void trashApp (in AString path, in nsITrashAppCallback callback); */
    pub trashApp: unsafe extern "C" fn (this: *const nsIMacWebAppUtils, path: *const nsAString, callback: *const nsITrashAppCallback) -> nsresult,

}


impl nsIMacWebAppUtils {
    /* AString pathForAppWithIdentifier (in AString bundleIdentifier); */
    #[inline]
    pub unsafe fn pathForAppWithIdentifier(&self, bundleIdentifier: &[u16]) -> Result<nsString, nsresult> {
        let bundleIdentifier = nsString::from(bundleIdentifier);
        let mut _retval = nsString::new();
        match ((*self.vtable).pathForAppWithIdentifier)(self as *const _, &*bundleIdentifier, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void launchAppWithIdentifier (in AString bundleIdentifier); */
    #[inline]
    pub unsafe fn launchAppWithIdentifier(&self, bundleIdentifier: &[u16]) -> Result<(), nsresult> {
        let bundleIdentifier = nsString::from(bundleIdentifier);
        match ((*self.vtable).launchAppWithIdentifier)(self as *const _, &*bundleIdentifier) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void trashApp (in AString path, in nsITrashAppCallback callback); */
    #[inline]
    pub unsafe fn trashApp(&self, path: &[u16], callback: Option<&nsITrashAppCallback>) -> Result<(), nsresult> {
        let path = nsString::from(path);
        match ((*self.vtable).trashApp)(self as *const _, &*path, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


