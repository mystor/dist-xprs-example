//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMSimpleGestureEvent.idl
//


pub mod nsIDOMSimpleGestureEvent_consts {
    pub const DIRECTION_UP: i64 = 1;
    pub const DIRECTION_DOWN: i64 = 2;
    pub const DIRECTION_LEFT: i64 = 4;
    pub const DIRECTION_RIGHT: i64 = 8;
    pub const ROTATION_COUNTERCLOCKWISE: i64 = 1;
    pub const ROTATION_CLOCKWISE: i64 = 2;
}


#[repr(C)]
pub struct nsIDOMSimpleGestureEvent {
    vtable: *const nsIDOMSimpleGestureEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMSimpleGestureEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc397f9a2, 0x4266, 0x4291,
            [0xb2, 0x82, 0x3e, 0xfd, 0x6d, 0x7a, 0xfc, 0x57])
    }
}

unsafe impl RefCounted for nsIDOMSimpleGestureEvent {
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
pub trait nsIDOMSimpleGestureEventCoerce {
    fn coerce_from(v: &nsIDOMSimpleGestureEvent) -> &Self;
}

impl nsIDOMSimpleGestureEventCoerce for nsIDOMSimpleGestureEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMSimpleGestureEvent) -> &Self {
        v
    }
}

impl nsIDOMSimpleGestureEvent {
    #[inline]
    pub fn coerce<T: nsIDOMSimpleGestureEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMSimpleGestureEvent {
    type Target = nsIDOMMouseEvent;
    #[inline]
    fn deref(&self) -> &nsIDOMMouseEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMMouseEventCoerce> nsIDOMSimpleGestureEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMSimpleGestureEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMSimpleGestureEventVTable {
    pub __base: nsIDOMMouseEventVTable,

    /* attribute unsigned long allowedDirections; */
    pub get_allowedDirections: unsafe extern "C" fn (this: *const nsIDOMSimpleGestureEvent, aAllowedDirections: *mut libc::uint32_t) -> nsresult,
    pub set_allowedDirections: unsafe extern "C" fn (this: *const nsIDOMSimpleGestureEvent, aAllowedDirections: libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long direction; */
    pub get_direction: unsafe extern "C" fn (this: *const nsIDOMSimpleGestureEvent, aDirection: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute double delta; */
    pub get_delta: unsafe extern "C" fn (this: *const nsIDOMSimpleGestureEvent, aDelta: *mut libc::c_double) -> nsresult,

    /* readonly attribute unsigned long clickCount; */
    pub get_clickCount: unsafe extern "C" fn (this: *const nsIDOMSimpleGestureEvent, aClickCount: *mut libc::uint32_t) -> nsresult,

}


impl nsIDOMSimpleGestureEvent {
    /* attribute unsigned long allowedDirections; */
    #[inline]
    pub unsafe fn get_allowedDirections(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_allowedDirections)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowedDirections(&self, aAllowedDirections: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowedDirections)(self as *const _, aAllowedDirections) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long direction; */
    #[inline]
    pub unsafe fn get_direction(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_direction)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double delta; */
    #[inline]
    pub unsafe fn get_delta(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_delta)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long clickCount; */
    #[inline]
    pub unsafe fn get_clickCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_clickCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


