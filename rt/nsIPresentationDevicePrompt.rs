//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationDevicePrompt.idl
//


#[repr(C)]
pub struct nsIPresentationDeviceRequest {
    vtable: *const nsIPresentationDeviceRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationDeviceRequest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb2aa7f6a, 0x9448, 0x446a,
            [0xbb, 0xa4, 0x9c, 0x29, 0x63, 0x8b, 0x0e, 0xd4])
    }
}

unsafe impl RefCounted for nsIPresentationDeviceRequest {
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
pub trait nsIPresentationDeviceRequestCoerce {
    fn coerce_from(v: &nsIPresentationDeviceRequest) -> &Self;
}

impl nsIPresentationDeviceRequestCoerce for nsIPresentationDeviceRequest {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceRequest) -> &Self {
        v
    }
}

impl nsIPresentationDeviceRequest {
    #[inline]
    pub fn coerce<T: nsIPresentationDeviceRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationDeviceRequest {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationDeviceRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceRequest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationDeviceRequestVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString origin; */
    pub get_origin: unsafe extern "C" fn (this: *const nsIPresentationDeviceRequest, aOrigin: *mut nsAString) -> nsresult,

    /* readonly attribute nsIArray requestURLs; */
    pub get_requestURLs: unsafe extern "C" fn (this: *const nsIPresentationDeviceRequest, aRequestURLs: *mut *const nsIArray) -> nsresult,

    /* readonly attribute nsIDOMEventTarget chromeEventHandler; */
    pub get_chromeEventHandler: unsafe extern "C" fn (this: *const nsIPresentationDeviceRequest, aChromeEventHandler: *mut *const nsIDOMEventTarget) -> nsresult,

    /* readonly attribute nsIPrincipal principal; */
    pub get_principal: unsafe extern "C" fn (this: *const nsIPresentationDeviceRequest, aPrincipal: *mut *const nsIPrincipal) -> nsresult,

    /* void select (in nsIPresentationDevice device); */
    pub select: unsafe extern "C" fn (this: *const nsIPresentationDeviceRequest, device: *const nsIPresentationDevice) -> nsresult,

    /* void cancel (in nsresult reason); */
    pub cancel: unsafe extern "C" fn (this: *const nsIPresentationDeviceRequest, reason: nsresult) -> nsresult,

}


impl nsIPresentationDeviceRequest {
    /* readonly attribute DOMString origin; */
    #[inline]
    pub unsafe fn get_origin(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_origin)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIArray requestURLs; */
    #[inline]
    pub unsafe fn get_requestURLs(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_requestURLs)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMEventTarget chromeEventHandler; */
    #[inline]
    pub unsafe fn get_chromeEventHandler(&self, ) -> Result<Option<RefPtr<nsIDOMEventTarget>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_chromeEventHandler)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIPrincipal principal; */
    #[inline]
    pub unsafe fn get_principal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_principal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void select (in nsIPresentationDevice device); */
    #[inline]
    pub unsafe fn select(&self, device: Option<&nsIPresentationDevice>) -> Result<(), nsresult> {

        match ((*self.vtable).select)(self as *const _, device.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cancel (in nsresult reason); */
    #[inline]
    pub unsafe fn cancel(&self, reason: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).cancel)(self as *const _, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIPresentationDevicePrompt {
    vtable: *const nsIPresentationDevicePromptVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationDevicePrompt {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xac1a7e44, 0xde86, 0x454f,
            [0xa9, 0xf1, 0x27, 0x6d, 0xe2, 0x53, 0x98, 0x31])
    }
}

unsafe impl RefCounted for nsIPresentationDevicePrompt {
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
pub trait nsIPresentationDevicePromptCoerce {
    fn coerce_from(v: &nsIPresentationDevicePrompt) -> &Self;
}

impl nsIPresentationDevicePromptCoerce for nsIPresentationDevicePrompt {
    #[inline]
    fn coerce_from(v: &nsIPresentationDevicePrompt) -> &Self {
        v
    }
}

impl nsIPresentationDevicePrompt {
    #[inline]
    pub fn coerce<T: nsIPresentationDevicePromptCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationDevicePrompt {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationDevicePromptCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationDevicePrompt) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationDevicePromptVTable {
    pub __base: nsISupportsVTable,

    /* void promptDeviceSelection (in nsIPresentationDeviceRequest request); */
    pub promptDeviceSelection: unsafe extern "C" fn (this: *const nsIPresentationDevicePrompt, request: *const nsIPresentationDeviceRequest) -> nsresult,

}


impl nsIPresentationDevicePrompt {
    /* void promptDeviceSelection (in nsIPresentationDeviceRequest request); */
    #[inline]
    pub unsafe fn promptDeviceSelection(&self, request: Option<&nsIPresentationDeviceRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).promptDeviceSelection)(self as *const _, request.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


