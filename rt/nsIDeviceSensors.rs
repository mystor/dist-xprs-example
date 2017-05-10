//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDeviceSensors.idl
//


pub mod nsIDeviceSensorData_consts {
    pub const TYPE_ORIENTATION: i64 = 0;
    pub const TYPE_ACCELERATION: i64 = 1;
    pub const TYPE_PROXIMITY: i64 = 2;
    pub const TYPE_LINEAR_ACCELERATION: i64 = 3;
    pub const TYPE_GYROSCOPE: i64 = 4;
    pub const TYPE_LIGHT: i64 = 5;
    pub const TYPE_ROTATION_VECTOR: i64 = 6;
    pub const TYPE_GAME_ROTATION_VECTOR: i64 = 7;
}


#[repr(C)]
pub struct nsIDeviceSensorData {
    vtable: *const nsIDeviceSensorDataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDeviceSensorData {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0462247e, 0xfe8c, 0x4aa5,
            [0xb6, 0x75, 0x37, 0x52, 0x54, 0x7e, 0x48, 0x5f])
    }
}

unsafe impl RefCounted for nsIDeviceSensorData {
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
pub trait nsIDeviceSensorDataCoerce {
    fn coerce_from(v: &nsIDeviceSensorData) -> &Self;
}

impl nsIDeviceSensorDataCoerce for nsIDeviceSensorData {
    #[inline]
    fn coerce_from(v: &nsIDeviceSensorData) -> &Self {
        v
    }
}

impl nsIDeviceSensorData {
    #[inline]
    pub fn coerce<T: nsIDeviceSensorDataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDeviceSensorData {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDeviceSensorDataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDeviceSensorData) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDeviceSensorDataVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDeviceSensorData, aType: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute double x; */
    pub get_x: unsafe extern "C" fn (this: *const nsIDeviceSensorData, aX: *mut libc::c_double) -> nsresult,

    /* readonly attribute double y; */
    pub get_y: unsafe extern "C" fn (this: *const nsIDeviceSensorData, aY: *mut libc::c_double) -> nsresult,

    /* readonly attribute double z; */
    pub get_z: unsafe extern "C" fn (this: *const nsIDeviceSensorData, aZ: *mut libc::c_double) -> nsresult,

}


impl nsIDeviceSensorData {
    /* readonly attribute unsigned long type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_type_)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double x; */
    #[inline]
    pub unsafe fn get_x(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_x)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double y; */
    #[inline]
    pub unsafe fn get_y(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_y)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double z; */
    #[inline]
    pub unsafe fn get_z(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_z)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIDeviceSensors {
    vtable: *const nsIDeviceSensorsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDeviceSensors {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe46e47c7, 0x55ff, 0x44c4,
            [0xab, 0xce, 0x21, 0xb1, 0x4b, 0xa0, 0x7f, 0x86])
    }
}

unsafe impl RefCounted for nsIDeviceSensors {
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
pub trait nsIDeviceSensorsCoerce {
    fn coerce_from(v: &nsIDeviceSensors) -> &Self;
}

impl nsIDeviceSensorsCoerce for nsIDeviceSensors {
    #[inline]
    fn coerce_from(v: &nsIDeviceSensors) -> &Self {
        v
    }
}

impl nsIDeviceSensors {
    #[inline]
    pub fn coerce<T: nsIDeviceSensorsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDeviceSensors {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDeviceSensorsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDeviceSensors) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDeviceSensorsVTable {
    pub __base: nsISupportsVTable,

    /* bool hasWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
    pub hasWindowListener: unsafe extern "C" fn (this: *const nsIDeviceSensors, aType: libc::uint32_t, aWindow: *const nsIDOMWindow, _retval: *mut bool) -> nsresult,

    /* [noscript] void addWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
    pub addWindowListener: unsafe extern "C" fn (this: *const nsIDeviceSensors, aType: libc::uint32_t, aWindow: *const nsIDOMWindow) -> nsresult,

    /* [noscript] void removeWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
    pub removeWindowListener: unsafe extern "C" fn (this: *const nsIDeviceSensors, aType: libc::uint32_t, aWindow: *const nsIDOMWindow) -> nsresult,

    /* [noscript] void removeWindowAsListener (in nsIDOMWindow aWindow); */
    pub removeWindowAsListener: unsafe extern "C" fn (this: *const nsIDeviceSensors, aWindow: *const nsIDOMWindow) -> nsresult,

}


impl nsIDeviceSensors {
    /* bool hasWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn hasWindowListener(&self, aType: libc::uint32_t, aWindow: Option<&nsIDOMWindow>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasWindowListener)(self as *const _, aType, aWindow.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void addWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn addWindowListener(&self, aType: libc::uint32_t, aWindow: Option<&nsIDOMWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).addWindowListener)(self as *const _, aType, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void removeWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn removeWindowListener(&self, aType: libc::uint32_t, aWindow: Option<&nsIDOMWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).removeWindowListener)(self as *const _, aType, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void removeWindowAsListener (in nsIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn removeWindowAsListener(&self, aWindow: Option<&nsIDOMWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).removeWindowAsListener)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


