//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationTerminateRequest.idl
//


#[repr(C)]
pub struct nsIPresentationTerminateRequest {
    vtable: *const nsIPresentationTerminateRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationTerminateRequest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3ddbf3a4, 0x53ee, 0x4b70,
            [0x9b, 0xbc, 0x58, 0xac, 0x90, 0xdc, 0xe6, 0xb5])
    }
}

unsafe impl RefCounted for nsIPresentationTerminateRequest {
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
pub trait nsIPresentationTerminateRequestCoerce {
    fn coerce_from(v: &nsIPresentationTerminateRequest) -> &Self;
}

impl nsIPresentationTerminateRequestCoerce for nsIPresentationTerminateRequest {
    #[inline]
    fn coerce_from(v: &nsIPresentationTerminateRequest) -> &Self {
        v
    }
}

impl nsIPresentationTerminateRequest {
    #[inline]
    pub fn coerce<T: nsIPresentationTerminateRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationTerminateRequest {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationTerminateRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationTerminateRequest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationTerminateRequestVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPresentationDevice device; */
    pub get_device: unsafe extern "C" fn (this: *const nsIPresentationTerminateRequest, aDevice: *mut *const nsIPresentationDevice) -> nsresult,

    /* readonly attribute DOMString presentationId; */
    pub get_presentationId: unsafe extern "C" fn (this: *const nsIPresentationTerminateRequest, aPresentationId: *mut nsAString) -> nsresult,

    /* readonly attribute nsIPresentationControlChannel controlChannel; */
    pub get_controlChannel: unsafe extern "C" fn (this: *const nsIPresentationTerminateRequest, aControlChannel: *mut *const nsIPresentationControlChannel) -> nsresult,

    /* readonly attribute boolean isFromReceiver; */
    pub get_isFromReceiver: unsafe extern "C" fn (this: *const nsIPresentationTerminateRequest, aIsFromReceiver: *mut bool) -> nsresult,

}


impl nsIPresentationTerminateRequest {
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

    /* readonly attribute boolean isFromReceiver; */
    #[inline]
    pub unsafe fn get_isFromReceiver(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isFromReceiver)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


