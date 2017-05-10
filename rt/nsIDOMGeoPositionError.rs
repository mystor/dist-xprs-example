//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMGeoPositionError.idl
//


pub mod nsIDOMGeoPositionError_consts {
    pub const PERMISSION_DENIED: i64 = 1;
    pub const POSITION_UNAVAILABLE: i64 = 2;
    pub const TIMEOUT: i64 = 3;
}


#[repr(C)]
pub struct nsIDOMGeoPositionError {
    vtable: *const nsIDOMGeoPositionErrorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMGeoPositionError {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x85255cc3, 0x07ba, 0x49fd,
            [0xbc, 0x9b, 0x18, 0xd2, 0x96, 0x3d, 0xaf, 0x7f])
    }
}

unsafe impl RefCounted for nsIDOMGeoPositionError {
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
pub trait nsIDOMGeoPositionErrorCoerce {
    fn coerce_from(v: &nsIDOMGeoPositionError) -> &Self;
}

impl nsIDOMGeoPositionErrorCoerce for nsIDOMGeoPositionError {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoPositionError) -> &Self {
        v
    }
}

impl nsIDOMGeoPositionError {
    #[inline]
    pub fn coerce<T: nsIDOMGeoPositionErrorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMGeoPositionError {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMGeoPositionErrorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoPositionError) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMGeoPositionErrorVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute short code; */
    pub get_code: unsafe extern "C" fn (this: *const nsIDOMGeoPositionError, aCode: *mut libc::int16_t) -> nsresult,

    /* readonly attribute AString message; */
    pub get_message: unsafe extern "C" fn (this: *const nsIDOMGeoPositionError, aMessage: *mut nsAString) -> nsresult,

}


impl nsIDOMGeoPositionError {
    /* readonly attribute short code; */
    #[inline]
    pub unsafe fn get_code(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_code)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString message; */
    #[inline]
    pub unsafe fn get_message(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_message)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


