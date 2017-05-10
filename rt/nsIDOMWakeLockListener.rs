//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMWakeLockListener.idl
//


#[repr(C)]
pub struct nsIDOMMozWakeLockListener {
    vtable: *const nsIDOMMozWakeLockListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMMozWakeLockListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4e258af8, 0xcffb, 0x47bc,
            [0xb1, 0x6d, 0xe8, 0x24, 0x12, 0x43, 0x42, 0x6e])
    }
}

unsafe impl RefCounted for nsIDOMMozWakeLockListener {
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
pub trait nsIDOMMozWakeLockListenerCoerce {
    fn coerce_from(v: &nsIDOMMozWakeLockListener) -> &Self;
}

impl nsIDOMMozWakeLockListenerCoerce for nsIDOMMozWakeLockListener {
    #[inline]
    fn coerce_from(v: &nsIDOMMozWakeLockListener) -> &Self {
        v
    }
}

impl nsIDOMMozWakeLockListener {
    #[inline]
    pub fn coerce<T: nsIDOMMozWakeLockListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMMozWakeLockListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMMozWakeLockListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMMozWakeLockListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMMozWakeLockListenerVTable {
    pub __base: nsISupportsVTable,

    /* void callback (in DOMString aTopic, in DOMString aState); */
    pub callback: unsafe extern "C" fn (this: *const nsIDOMMozWakeLockListener, aTopic: *const nsAString, aState: *const nsAString) -> nsresult,

}


impl nsIDOMMozWakeLockListener {
    /* void callback (in DOMString aTopic, in DOMString aState); */
    #[inline]
    pub unsafe fn callback(&self, aTopic: &[u16], aState: &[u16]) -> Result<(), nsresult> {
        let aTopic = nsString::from(aTopic);
        let aState = nsString::from(aState);
        match ((*self.vtable).callback)(self as *const _, &*aTopic, &*aState) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


