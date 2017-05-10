//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMGeoGeolocation.idl
//


#[repr(C)]
pub struct nsIDOMGeoGeolocation {
    vtable: *const nsIDOMGeoGeolocationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMGeoGeolocation {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9142ab45, 0x0ab5, 0x418c,
            [0x9b, 0xab, 0x33, 0x8a, 0x6d, 0x27, 0x1d, 0x4f])
    }
}

unsafe impl RefCounted for nsIDOMGeoGeolocation {
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
pub trait nsIDOMGeoGeolocationCoerce {
    fn coerce_from(v: &nsIDOMGeoGeolocation) -> &Self;
}

impl nsIDOMGeoGeolocationCoerce for nsIDOMGeoGeolocation {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoGeolocation) -> &Self {
        v
    }
}

impl nsIDOMGeoGeolocation {
    #[inline]
    pub fn coerce<T: nsIDOMGeoGeolocationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMGeoGeolocation {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMGeoGeolocationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoGeolocation) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMGeoGeolocationVTable {
    pub __base: nsISupportsVTable,

    /* int32_t watchPosition (in nsIDOMGeoPositionCallback callback, in nsIDOMGeoPositionErrorCallback errorCallback, in PositionOptionsRef options); */
    /// Unable to call function as its signature contains a non-rust type
    pub watchPosition: *const ::libc::c_void,

    /* void clearWatch (in long watchId); */
    pub clearWatch: unsafe extern "C" fn (this: *const nsIDOMGeoGeolocation, watchId: libc::int32_t) -> nsresult,

}


impl nsIDOMGeoGeolocation {
    /* int32_t watchPosition (in nsIDOMGeoPositionCallback callback, in nsIDOMGeoPositionErrorCallback errorCallback, in PositionOptionsRef options); */


    /* void clearWatch (in long watchId); */
    #[inline]
    pub unsafe fn clearWatch(&self, watchId: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).clearWatch)(self as *const _, watchId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


