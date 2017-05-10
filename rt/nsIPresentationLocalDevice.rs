//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationLocalDevice.idl
//


#[repr(C)]
pub struct nsIPresentationLocalDevice {
    vtable: *const nsIPresentationLocalDeviceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationLocalDevice {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdd239720, 0xcab6, 0x4fb5,
            [0x90, 0x25, 0xcb, 0xa2, 0x3f, 0x1b, 0xbc, 0x2d])
    }
}

unsafe impl RefCounted for nsIPresentationLocalDevice {
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
pub trait nsIPresentationLocalDeviceCoerce {
    fn coerce_from(v: &nsIPresentationLocalDevice) -> &Self;
}

impl nsIPresentationLocalDeviceCoerce for nsIPresentationLocalDevice {
    #[inline]
    fn coerce_from(v: &nsIPresentationLocalDevice) -> &Self {
        v
    }
}

impl nsIPresentationLocalDevice {
    #[inline]
    pub fn coerce<T: nsIPresentationLocalDeviceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationLocalDevice {
    type Target = nsIPresentationDevice;
    #[inline]
    fn deref(&self) -> &nsIPresentationDevice {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIPresentationDeviceCoerce> nsIPresentationLocalDeviceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationLocalDevice) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationLocalDeviceVTable {
    pub __base: nsIPresentationDeviceVTable,

    /* readonly attribute AUTF8String windowId; */
    pub get_windowId: unsafe extern "C" fn (this: *const nsIPresentationLocalDevice, aWindowId: *mut nsACString) -> nsresult,

}


impl nsIPresentationLocalDevice {
    /* readonly attribute AUTF8String windowId; */
    #[inline]
    pub unsafe fn get_windowId(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_windowId)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


