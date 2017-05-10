//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMGeoPositionCallback.idl
//


#[repr(C)]
pub struct nsIDOMGeoPositionCallback {
    vtable: *const nsIDOMGeoPositionCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMGeoPositionCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x527e8b53, 0x6f29, 0x4b6a,
            [0x8d, 0x04, 0x5c, 0x16, 0x66, 0xa4, 0xc4, 0xc1])
    }
}

unsafe impl RefCounted for nsIDOMGeoPositionCallback {
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
pub trait nsIDOMGeoPositionCallbackCoerce {
    fn coerce_from(v: &nsIDOMGeoPositionCallback) -> &Self;
}

impl nsIDOMGeoPositionCallbackCoerce for nsIDOMGeoPositionCallback {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoPositionCallback) -> &Self {
        v
    }
}

impl nsIDOMGeoPositionCallback {
    #[inline]
    pub fn coerce<T: nsIDOMGeoPositionCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMGeoPositionCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMGeoPositionCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoPositionCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMGeoPositionCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void handleEvent (in nsIDOMGeoPosition position); */
    pub handleEvent: unsafe extern "C" fn (this: *const nsIDOMGeoPositionCallback, position: *const nsIDOMGeoPosition) -> nsresult,

}


impl nsIDOMGeoPositionCallback {
    /* void handleEvent (in nsIDOMGeoPosition position); */
    #[inline]
    pub unsafe fn handleEvent(&self, position: Option<&nsIDOMGeoPosition>) -> Result<(), nsresult> {

        match ((*self.vtable).handleEvent)(self as *const _, position.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


