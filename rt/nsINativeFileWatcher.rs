//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINativeFileWatcher.idl
//


#[repr(C)]
pub struct nsINativeFileWatcherErrorCallback {
    vtable: *const nsINativeFileWatcherErrorCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINativeFileWatcherErrorCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5daeddc3, 0xfc94, 0x4880,
            [0x8a, 0x4f, 0x26, 0xd9, 0x10, 0xb9, 0x26, 0x62])
    }
}

unsafe impl RefCounted for nsINativeFileWatcherErrorCallback {
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
pub trait nsINativeFileWatcherErrorCallbackCoerce {
    fn coerce_from(v: &nsINativeFileWatcherErrorCallback) -> &Self;
}

impl nsINativeFileWatcherErrorCallbackCoerce for nsINativeFileWatcherErrorCallback {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherErrorCallback) -> &Self {
        v
    }
}

impl nsINativeFileWatcherErrorCallback {
    #[inline]
    pub fn coerce<T: nsINativeFileWatcherErrorCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINativeFileWatcherErrorCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINativeFileWatcherErrorCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherErrorCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINativeFileWatcherErrorCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void complete (in nsresult xpcomError, in long osError); */
    pub complete: unsafe extern "C" fn (this: *const nsINativeFileWatcherErrorCallback, xpcomError: nsresult, osError: libc::int32_t) -> nsresult,

}


impl nsINativeFileWatcherErrorCallback {
    /* void complete (in nsresult xpcomError, in long osError); */
    #[inline]
    pub unsafe fn complete(&self, xpcomError: nsresult, osError: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).complete)(self as *const _, xpcomError, osError) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsINativeFileWatcherCallback {
    vtable: *const nsINativeFileWatcherCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINativeFileWatcherCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfe4d86c9, 0x243f, 0x4195,
            [0xb5, 0x44, 0xae, 0xce, 0x3d, 0xf4, 0xb8, 0x6a])
    }
}

unsafe impl RefCounted for nsINativeFileWatcherCallback {
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
pub trait nsINativeFileWatcherCallbackCoerce {
    fn coerce_from(v: &nsINativeFileWatcherCallback) -> &Self;
}

impl nsINativeFileWatcherCallbackCoerce for nsINativeFileWatcherCallback {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherCallback) -> &Self {
        v
    }
}

impl nsINativeFileWatcherCallback {
    #[inline]
    pub fn coerce<T: nsINativeFileWatcherCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINativeFileWatcherCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINativeFileWatcherCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINativeFileWatcherCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void changed (in AString resourcePath, in int32_t flags); */
    pub changed: unsafe extern "C" fn (this: *const nsINativeFileWatcherCallback, resourcePath: *const nsAString, flags: int32_t) -> nsresult,

}


impl nsINativeFileWatcherCallback {
    /* void changed (in AString resourcePath, in int32_t flags); */
    #[inline]
    pub unsafe fn changed(&self, resourcePath: &[u16], flags: int32_t) -> Result<(), nsresult> {
        let resourcePath = nsString::from(resourcePath);
        match ((*self.vtable).changed)(self as *const _, &*resourcePath, flags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsINativeFileWatcherSuccessCallback {
    vtable: *const nsINativeFileWatcherSuccessCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINativeFileWatcherSuccessCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc3d7f542, 0x681b, 0x4abd,
            [0x9d, 0x65, 0x9d, 0x79, 0x9b, 0x29, 0xa4, 0x2b])
    }
}

unsafe impl RefCounted for nsINativeFileWatcherSuccessCallback {
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
pub trait nsINativeFileWatcherSuccessCallbackCoerce {
    fn coerce_from(v: &nsINativeFileWatcherSuccessCallback) -> &Self;
}

impl nsINativeFileWatcherSuccessCallbackCoerce for nsINativeFileWatcherSuccessCallback {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherSuccessCallback) -> &Self {
        v
    }
}

impl nsINativeFileWatcherSuccessCallback {
    #[inline]
    pub fn coerce<T: nsINativeFileWatcherSuccessCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINativeFileWatcherSuccessCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINativeFileWatcherSuccessCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherSuccessCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINativeFileWatcherSuccessCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void complete (in AString resourcePath); */
    pub complete: unsafe extern "C" fn (this: *const nsINativeFileWatcherSuccessCallback, resourcePath: *const nsAString) -> nsresult,

}


impl nsINativeFileWatcherSuccessCallback {
    /* void complete (in AString resourcePath); */
    #[inline]
    pub unsafe fn complete(&self, resourcePath: &[u16]) -> Result<(), nsresult> {
        let resourcePath = nsString::from(resourcePath);
        match ((*self.vtable).complete)(self as *const _, &*resourcePath) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsINativeFileWatcherService {
    vtable: *const nsINativeFileWatcherServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINativeFileWatcherService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb3a4e8d8, 0x7dc8, 0x47db,
            [0xa8, 0xb4, 0x83, 0x73, 0x6d, 0x7a, 0xc1, 0xaa])
    }
}

unsafe impl RefCounted for nsINativeFileWatcherService {
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
pub trait nsINativeFileWatcherServiceCoerce {
    fn coerce_from(v: &nsINativeFileWatcherService) -> &Self;
}

impl nsINativeFileWatcherServiceCoerce for nsINativeFileWatcherService {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherService) -> &Self {
        v
    }
}

impl nsINativeFileWatcherService {
    #[inline]
    pub fn coerce<T: nsINativeFileWatcherServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINativeFileWatcherService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINativeFileWatcherServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINativeFileWatcherServiceVTable {
    pub __base: nsISupportsVTable,

    /* void addPath (in AString pathToWatch, in nsINativeFileWatcherCallback onChange, [optional] in nsINativeFileWatcherErrorCallback onError, [optional] in nsINativeFileWatcherSuccessCallback onSuccess); */
    pub addPath: unsafe extern "C" fn (this: *const nsINativeFileWatcherService, pathToWatch: *const nsAString, onChange: *const nsINativeFileWatcherCallback, onError: *const nsINativeFileWatcherErrorCallback, onSuccess: *const nsINativeFileWatcherSuccessCallback) -> nsresult,

    /* void removePath (in AString pathToUnwatch, in nsINativeFileWatcherCallback onChange, [optional] in nsINativeFileWatcherErrorCallback onError, [optional] in nsINativeFileWatcherSuccessCallback onSuccess); */
    pub removePath: unsafe extern "C" fn (this: *const nsINativeFileWatcherService, pathToUnwatch: *const nsAString, onChange: *const nsINativeFileWatcherCallback, onError: *const nsINativeFileWatcherErrorCallback, onSuccess: *const nsINativeFileWatcherSuccessCallback) -> nsresult,

}


impl nsINativeFileWatcherService {
    /* void addPath (in AString pathToWatch, in nsINativeFileWatcherCallback onChange, [optional] in nsINativeFileWatcherErrorCallback onError, [optional] in nsINativeFileWatcherSuccessCallback onSuccess); */
    #[inline]
    pub unsafe fn addPath(&self, pathToWatch: &[u16], onChange: Option<&nsINativeFileWatcherCallback>, onError: Option<&nsINativeFileWatcherErrorCallback>, onSuccess: Option<&nsINativeFileWatcherSuccessCallback>) -> Result<(), nsresult> {
        let pathToWatch = nsString::from(pathToWatch);
        match ((*self.vtable).addPath)(self as *const _, &*pathToWatch, onChange.map_or(::std::ptr::null(), |x| x as *const _), onError.map_or(::std::ptr::null(), |x| x as *const _), onSuccess.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removePath (in AString pathToUnwatch, in nsINativeFileWatcherCallback onChange, [optional] in nsINativeFileWatcherErrorCallback onError, [optional] in nsINativeFileWatcherSuccessCallback onSuccess); */
    #[inline]
    pub unsafe fn removePath(&self, pathToUnwatch: &[u16], onChange: Option<&nsINativeFileWatcherCallback>, onError: Option<&nsINativeFileWatcherErrorCallback>, onSuccess: Option<&nsINativeFileWatcherSuccessCallback>) -> Result<(), nsresult> {
        let pathToUnwatch = nsString::from(pathToUnwatch);
        match ((*self.vtable).removePath)(self as *const _, &*pathToUnwatch, onChange.map_or(::std::ptr::null(), |x| x as *const _), onError.map_or(::std::ptr::null(), |x| x as *const _), onSuccess.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


