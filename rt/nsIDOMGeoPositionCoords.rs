//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMGeoPositionCoords.idl
//


#[repr(C)]
pub struct nsIDOMGeoPositionCoords {
    vtable: *const nsIDOMGeoPositionCoordsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMGeoPositionCoords {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb31702d0, 0x6dac, 0x4fa0,
            [0xb9, 0x3b, 0xf0, 0x43, 0xe7, 0x1c, 0x8f, 0x9a])
    }
}

unsafe impl RefCounted for nsIDOMGeoPositionCoords {
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
pub trait nsIDOMGeoPositionCoordsCoerce {
    fn coerce_from(v: &nsIDOMGeoPositionCoords) -> &Self;
}

impl nsIDOMGeoPositionCoordsCoerce for nsIDOMGeoPositionCoords {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoPositionCoords) -> &Self {
        v
    }
}

impl nsIDOMGeoPositionCoords {
    #[inline]
    pub fn coerce<T: nsIDOMGeoPositionCoordsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMGeoPositionCoords {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMGeoPositionCoordsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoPositionCoords) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMGeoPositionCoordsVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute double latitude; */
    pub get_latitude: unsafe extern "C" fn (this: *const nsIDOMGeoPositionCoords, aLatitude: *mut libc::c_double) -> nsresult,

    /* readonly attribute double longitude; */
    pub get_longitude: unsafe extern "C" fn (this: *const nsIDOMGeoPositionCoords, aLongitude: *mut libc::c_double) -> nsresult,

    /* readonly attribute double altitude; */
    pub get_altitude: unsafe extern "C" fn (this: *const nsIDOMGeoPositionCoords, aAltitude: *mut libc::c_double) -> nsresult,

    /* readonly attribute double accuracy; */
    pub get_accuracy: unsafe extern "C" fn (this: *const nsIDOMGeoPositionCoords, aAccuracy: *mut libc::c_double) -> nsresult,

    /* readonly attribute double altitudeAccuracy; */
    pub get_altitudeAccuracy: unsafe extern "C" fn (this: *const nsIDOMGeoPositionCoords, aAltitudeAccuracy: *mut libc::c_double) -> nsresult,

    /* readonly attribute double heading; */
    pub get_heading: unsafe extern "C" fn (this: *const nsIDOMGeoPositionCoords, aHeading: *mut libc::c_double) -> nsresult,

    /* readonly attribute double speed; */
    pub get_speed: unsafe extern "C" fn (this: *const nsIDOMGeoPositionCoords, aSpeed: *mut libc::c_double) -> nsresult,

}


impl nsIDOMGeoPositionCoords {
    /* readonly attribute double latitude; */
    #[inline]
    pub unsafe fn get_latitude(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_latitude)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double longitude; */
    #[inline]
    pub unsafe fn get_longitude(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_longitude)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double altitude; */
    #[inline]
    pub unsafe fn get_altitude(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_altitude)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double accuracy; */
    #[inline]
    pub unsafe fn get_accuracy(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_accuracy)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double altitudeAccuracy; */
    #[inline]
    pub unsafe fn get_altitudeAccuracy(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_altitudeAccuracy)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double heading; */
    #[inline]
    pub unsafe fn get_heading(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_heading)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double speed; */
    #[inline]
    pub unsafe fn get_speed(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_speed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


