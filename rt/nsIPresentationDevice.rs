//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationDevice.idl
//


#[repr(C)]
pub struct nsIPresentationDevice {
    vtable: *const nsIPresentationDeviceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationDevice {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb1e0a7af, 0x5936, 0x4066,
            [0x8f, 0x2e, 0xf7, 0x89, 0xfb, 0x9a, 0x7e, 0x8f])
    }
}

unsafe impl RefCounted for nsIPresentationDevice {
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
pub trait nsIPresentationDeviceCoerce {
    fn coerce_from(v: &nsIPresentationDevice) -> &Self;
}

impl nsIPresentationDeviceCoerce for nsIPresentationDevice {
    #[inline]
    fn coerce_from(v: &nsIPresentationDevice) -> &Self {
        v
    }
}

impl nsIPresentationDevice {
    #[inline]
    pub fn coerce<T: nsIPresentationDeviceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationDevice {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationDeviceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationDevice) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationDeviceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String id; */
    pub get_id: unsafe extern "C" fn (this: *const nsIPresentationDevice, aId: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIPresentationDevice, aName: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIPresentationDevice, aType: *mut nsACString) -> nsresult,

    /* nsIPresentationControlChannel establishControlChannel (); */
    pub establishControlChannel: unsafe extern "C" fn (this: *const nsIPresentationDevice, _retval: *mut *const nsIPresentationControlChannel) -> nsresult,

    /* void disconnect (); */
    pub disconnect: unsafe extern "C" fn (this: *const nsIPresentationDevice) -> nsresult,

    /* boolean isRequestedUrlSupported (in DOMString requestedUrl); */
    pub isRequestedUrlSupported: unsafe extern "C" fn (this: *const nsIPresentationDevice, requestedUrl: *const nsAString, _retval: *mut bool) -> nsresult,

}


impl nsIPresentationDevice {
    /* readonly attribute AUTF8String id; */
    #[inline]
    pub unsafe fn get_id(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_id)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIPresentationControlChannel establishControlChannel (); */
    #[inline]
    pub unsafe fn establishControlChannel(&self, ) -> Result<Option<RefPtr<nsIPresentationControlChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).establishControlChannel)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void disconnect (); */
    #[inline]
    pub unsafe fn disconnect(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).disconnect)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isRequestedUrlSupported (in DOMString requestedUrl); */
    #[inline]
    pub unsafe fn isRequestedUrlSupported(&self, requestedUrl: &[u16]) -> Result<bool, nsresult> {
        let requestedUrl = nsString::from(requestedUrl);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isRequestedUrlSupported)(self as *const _, &*requestedUrl, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


