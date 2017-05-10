//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationSessionRequest.idl
//


#[repr(C)]
pub struct nsIPresentationSessionRequest {
    vtable: *const nsIPresentationSessionRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationSessionRequest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd808a084, 0xd0f8, 0x455a,
            [0xa8, 0xdf, 0x58, 0x79, 0xe0, 0x5a, 0x75, 0x5b])
    }
}

unsafe impl RefCounted for nsIPresentationSessionRequest {
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
pub trait nsIPresentationSessionRequestCoerce {
    fn coerce_from(v: &nsIPresentationSessionRequest) -> &Self;
}

impl nsIPresentationSessionRequestCoerce for nsIPresentationSessionRequest {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionRequest) -> &Self {
        v
    }
}

impl nsIPresentationSessionRequest {
    #[inline]
    pub fn coerce<T: nsIPresentationSessionRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationSessionRequest {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationSessionRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionRequest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationSessionRequestVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPresentationDevice device; */
    pub get_device: unsafe extern "C" fn (this: *const nsIPresentationSessionRequest, aDevice: *mut *const nsIPresentationDevice) -> nsresult,

    /* readonly attribute DOMString url; */
    pub get_url: unsafe extern "C" fn (this: *const nsIPresentationSessionRequest, aUrl: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString presentationId; */
    pub get_presentationId: unsafe extern "C" fn (this: *const nsIPresentationSessionRequest, aPresentationId: *mut nsAString) -> nsresult,

    /* readonly attribute nsIPresentationControlChannel controlChannel; */
    pub get_controlChannel: unsafe extern "C" fn (this: *const nsIPresentationSessionRequest, aControlChannel: *mut *const nsIPresentationControlChannel) -> nsresult,

}


impl nsIPresentationSessionRequest {
    /* readonly attribute nsIPresentationDevice device; */
    #[inline]
    pub unsafe fn get_device(&self, ) -> Result<Option<RefPtr<nsIPresentationDevice>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_device)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute DOMString url; */
    #[inline]
    pub unsafe fn get_url(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_url)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString presentationId; */
    #[inline]
    pub unsafe fn get_presentationId(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_presentationId)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIPresentationControlChannel controlChannel; */
    #[inline]
    pub unsafe fn get_controlChannel(&self, ) -> Result<Option<RefPtr<nsIPresentationControlChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_controlChannel)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


