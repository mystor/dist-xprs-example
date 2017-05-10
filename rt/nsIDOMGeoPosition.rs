//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMGeoPosition.idl
//


#[repr(C)]
pub struct nsIDOMGeoPosition {
    vtable: *const nsIDOMGeoPositionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMGeoPosition {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdd9f7e81, 0x0f74, 0x4fb5,
            [0xb3, 0x61, 0x37, 0x01, 0x9b, 0xf6, 0x0c, 0x3f])
    }
}

unsafe impl RefCounted for nsIDOMGeoPosition {
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
pub trait nsIDOMGeoPositionCoerce {
    fn coerce_from(v: &nsIDOMGeoPosition) -> &Self;
}

impl nsIDOMGeoPositionCoerce for nsIDOMGeoPosition {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoPosition) -> &Self {
        v
    }
}

impl nsIDOMGeoPosition {
    #[inline]
    pub fn coerce<T: nsIDOMGeoPositionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMGeoPosition {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMGeoPositionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoPosition) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMGeoPositionVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMTimeStamp timestamp; */
    pub get_timestamp: unsafe extern "C" fn (this: *const nsIDOMGeoPosition, aTimestamp: *mut DOMTimeStamp) -> nsresult,

    /* readonly attribute nsIDOMGeoPositionCoords coords; */
    pub get_coords: unsafe extern "C" fn (this: *const nsIDOMGeoPosition, aCoords: *mut *const nsIDOMGeoPositionCoords) -> nsresult,

}


impl nsIDOMGeoPosition {
    /* readonly attribute DOMTimeStamp timestamp; */
    #[inline]
    pub unsafe fn get_timestamp(&self, ) -> Result<DOMTimeStamp, nsresult> {
        let mut _retval: DOMTimeStamp = ::std::mem::zeroed();
        match ((*self.vtable).get_timestamp)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMGeoPositionCoords coords; */
    #[inline]
    pub unsafe fn get_coords(&self, ) -> Result<Option<RefPtr<nsIDOMGeoPositionCoords>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_coords)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


