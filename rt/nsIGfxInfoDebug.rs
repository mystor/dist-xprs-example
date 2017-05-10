//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGfxInfoDebug.idl
//


#[repr(C)]
pub struct nsIGfxInfoDebug {
    vtable: *const nsIGfxInfoDebugVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGfxInfoDebug {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xca7b0bc7, 0xc67c, 0x4b79,
            [0x82, 0x70, 0xed, 0x7b, 0xa0, 0x02, 0xaf, 0x08])
    }
}

unsafe impl RefCounted for nsIGfxInfoDebug {
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
pub trait nsIGfxInfoDebugCoerce {
    fn coerce_from(v: &nsIGfxInfoDebug) -> &Self;
}

impl nsIGfxInfoDebugCoerce for nsIGfxInfoDebug {
    #[inline]
    fn coerce_from(v: &nsIGfxInfoDebug) -> &Self {
        v
    }
}

impl nsIGfxInfoDebug {
    #[inline]
    pub fn coerce<T: nsIGfxInfoDebugCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGfxInfoDebug {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIGfxInfoDebugCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGfxInfoDebug) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGfxInfoDebugVTable {
    pub __base: nsISupportsVTable,

    /* void spoofVendorID (in DOMString aVendorID); */
    pub spoofVendorID: unsafe extern "C" fn (this: *const nsIGfxInfoDebug, aVendorID: *const nsAString) -> nsresult,

    /* void spoofDeviceID (in DOMString aDeviceID); */
    pub spoofDeviceID: unsafe extern "C" fn (this: *const nsIGfxInfoDebug, aDeviceID: *const nsAString) -> nsresult,

    /* void spoofDriverVersion (in DOMString aDriverVersion); */
    pub spoofDriverVersion: unsafe extern "C" fn (this: *const nsIGfxInfoDebug, aDriverVersion: *const nsAString) -> nsresult,

    /* void spoofOSVersion (in unsigned long aVersion); */
    pub spoofOSVersion: unsafe extern "C" fn (this: *const nsIGfxInfoDebug, aVersion: libc::uint32_t) -> nsresult,

}


impl nsIGfxInfoDebug {
    /* void spoofVendorID (in DOMString aVendorID); */
    #[inline]
    pub unsafe fn spoofVendorID(&self, aVendorID: &[u16]) -> Result<(), nsresult> {
        let aVendorID = nsString::from(aVendorID);
        match ((*self.vtable).spoofVendorID)(self as *const _, &*aVendorID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void spoofDeviceID (in DOMString aDeviceID); */
    #[inline]
    pub unsafe fn spoofDeviceID(&self, aDeviceID: &[u16]) -> Result<(), nsresult> {
        let aDeviceID = nsString::from(aDeviceID);
        match ((*self.vtable).spoofDeviceID)(self as *const _, &*aDeviceID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void spoofDriverVersion (in DOMString aDriverVersion); */
    #[inline]
    pub unsafe fn spoofDriverVersion(&self, aDriverVersion: &[u16]) -> Result<(), nsresult> {
        let aDriverVersion = nsString::from(aDriverVersion);
        match ((*self.vtable).spoofDriverVersion)(self as *const _, &*aDriverVersion) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void spoofOSVersion (in unsigned long aVersion); */
    #[inline]
    pub unsafe fn spoofOSVersion(&self, aVersion: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).spoofOSVersion)(self as *const _, aVersion) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


