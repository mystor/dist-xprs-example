//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWifiListener.idl
//


#[repr(C)]
pub struct nsIWifiListener {
    vtable: *const nsIWifiListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWifiListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbcd4bede, 0xf4a5, 0x4a62,
            [0x90, 0x71, 0xd7, 0xa6, 0x01, 0x74, 0xe3, 0x76])
    }
}

unsafe impl RefCounted for nsIWifiListener {
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
pub trait nsIWifiListenerCoerce {
    fn coerce_from(v: &nsIWifiListener) -> &Self;
}

impl nsIWifiListenerCoerce for nsIWifiListener {
    #[inline]
    fn coerce_from(v: &nsIWifiListener) -> &Self {
        v
    }
}

impl nsIWifiListener {
    #[inline]
    pub fn coerce<T: nsIWifiListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWifiListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWifiListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWifiListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWifiListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onChange ([array, size_is (aLen)] in nsIWifiAccessPoint accessPoints, in unsigned long aLen); */
    /// Unable to call function as its signature contains a non-rust type
    pub onChange: *const ::libc::c_void,

    /* void onError (in nsresult error); */
    pub onError: unsafe extern "C" fn (this: *const nsIWifiListener, error: nsresult) -> nsresult,

}


impl nsIWifiListener {
    /* void onChange ([array, size_is (aLen)] in nsIWifiAccessPoint accessPoints, in unsigned long aLen); */


    /* void onError (in nsresult error); */
    #[inline]
    pub unsafe fn onError(&self, error: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onError)(self as *const _, error) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


