//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMGeoPositionErrorCallback.idl
//


#[repr(C)]
pub struct nsIDOMGeoPositionErrorCallback {
    vtable: *const nsIDOMGeoPositionErrorCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMGeoPositionErrorCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7d9b09d9, 0x4843, 0x43eb,
            [0xa7, 0xa7, 0x67, 0xf7, 0xdd, 0xa6, 0xb3, 0xc4])
    }
}

unsafe impl RefCounted for nsIDOMGeoPositionErrorCallback {
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
pub trait nsIDOMGeoPositionErrorCallbackCoerce {
    fn coerce_from(v: &nsIDOMGeoPositionErrorCallback) -> &Self;
}

impl nsIDOMGeoPositionErrorCallbackCoerce for nsIDOMGeoPositionErrorCallback {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoPositionErrorCallback) -> &Self {
        v
    }
}

impl nsIDOMGeoPositionErrorCallback {
    #[inline]
    pub fn coerce<T: nsIDOMGeoPositionErrorCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMGeoPositionErrorCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMGeoPositionErrorCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoPositionErrorCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMGeoPositionErrorCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void handleEvent (in nsIDOMGeoPositionError positionError); */
    pub handleEvent: unsafe extern "C" fn (this: *const nsIDOMGeoPositionErrorCallback, positionError: *const nsIDOMGeoPositionError) -> nsresult,

}


impl nsIDOMGeoPositionErrorCallback {
    /* void handleEvent (in nsIDOMGeoPositionError positionError); */
    #[inline]
    pub unsafe fn handleEvent(&self, positionError: Option<&nsIDOMGeoPositionError>) -> Result<(), nsresult> {

        match ((*self.vtable).handleEvent)(self as *const _, positionError.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


