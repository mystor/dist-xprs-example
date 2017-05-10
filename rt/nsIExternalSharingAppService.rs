//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIExternalSharingAppService.idl
//


#[repr(C)]
pub struct nsISharingHandlerApp {
    vtable: *const nsISharingHandlerAppVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISharingHandlerApp {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7111f769, 0x53ec, 0x41fd,
            [0xb3, 0x14, 0x61, 0x36, 0x61, 0xd5, 0xb6, 0xba])
    }
}

unsafe impl RefCounted for nsISharingHandlerApp {
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
pub trait nsISharingHandlerAppCoerce {
    fn coerce_from(v: &nsISharingHandlerApp) -> &Self;
}

impl nsISharingHandlerAppCoerce for nsISharingHandlerApp {
    #[inline]
    fn coerce_from(v: &nsISharingHandlerApp) -> &Self {
        v
    }
}

impl nsISharingHandlerApp {
    #[inline]
    pub fn coerce<T: nsISharingHandlerAppCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISharingHandlerApp {
    type Target = nsIHandlerApp;
    #[inline]
    fn deref(&self) -> &nsIHandlerApp {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIHandlerAppCoerce> nsISharingHandlerAppCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISharingHandlerApp) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISharingHandlerAppVTable {
    pub __base: nsIHandlerAppVTable,

    /* void share (in AString data, [optional] in AString title); */
    pub share: unsafe extern "C" fn (this: *const nsISharingHandlerApp, data: *const nsAString, title: *const nsAString) -> nsresult,

}


impl nsISharingHandlerApp {
    /* void share (in AString data, [optional] in AString title); */
    #[inline]
    pub unsafe fn share(&self, data: &[u16], title: &[u16]) -> Result<(), nsresult> {
        let data = nsString::from(data);
        let title = nsString::from(title);
        match ((*self.vtable).share)(self as *const _, &*data, &*title) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIExternalSharingAppService {
    vtable: *const nsIExternalSharingAppServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIExternalSharingAppService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcf7d04e5, 0x3892, 0x482e,
            [0x81, 0xbb, 0x07, 0x3d, 0xc1, 0xc8, 0x3f, 0x76])
    }
}

unsafe impl RefCounted for nsIExternalSharingAppService {
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
pub trait nsIExternalSharingAppServiceCoerce {
    fn coerce_from(v: &nsIExternalSharingAppService) -> &Self;
}

impl nsIExternalSharingAppServiceCoerce for nsIExternalSharingAppService {
    #[inline]
    fn coerce_from(v: &nsIExternalSharingAppService) -> &Self {
        v
    }
}

impl nsIExternalSharingAppService {
    #[inline]
    pub fn coerce<T: nsIExternalSharingAppServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIExternalSharingAppService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIExternalSharingAppServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIExternalSharingAppService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIExternalSharingAppServiceVTable {
    pub __base: nsISupportsVTable,

    /* void shareWithDefault (in AString data, in AString mime, [optional] in AString title); */
    pub shareWithDefault: unsafe extern "C" fn (this: *const nsIExternalSharingAppService, data: *const nsAString, mime: *const nsAString, title: *const nsAString) -> nsresult,

    /* void getSharingApps (in AString aMIMEType, [optional] out unsigned long aLen, [array, size_is (aLen), retval] out nsISharingHandlerApp handlerApps); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSharingApps: *const ::libc::c_void,

}


impl nsIExternalSharingAppService {
    /* void shareWithDefault (in AString data, in AString mime, [optional] in AString title); */
    #[inline]
    pub unsafe fn shareWithDefault(&self, data: &[u16], mime: &[u16], title: &[u16]) -> Result<(), nsresult> {
        let data = nsString::from(data);
        let mime = nsString::from(mime);
        let title = nsString::from(title);
        match ((*self.vtable).shareWithDefault)(self as *const _, &*data, &*mime, &*title) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getSharingApps (in AString aMIMEType, [optional] out unsigned long aLen, [array, size_is (aLen), retval] out nsISharingHandlerApp handlerApps); */


}


