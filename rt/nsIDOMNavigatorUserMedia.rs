//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMNavigatorUserMedia.idl
//


#[repr(C)]
pub struct nsIMediaDevice {
    vtable: *const nsIMediaDeviceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMediaDevice {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xba3b2e08, 0x1c07, 0x4cd3,
            [0x88, 0x22, 0xf4, 0xd7, 0xe3, 0x5f, 0xf2, 0xae])
    }
}

unsafe impl RefCounted for nsIMediaDevice {
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
pub trait nsIMediaDeviceCoerce {
    fn coerce_from(v: &nsIMediaDevice) -> &Self;
}

impl nsIMediaDeviceCoerce for nsIMediaDevice {
    #[inline]
    fn coerce_from(v: &nsIMediaDevice) -> &Self {
        v
    }
}

impl nsIMediaDevice {
    #[inline]
    pub fn coerce<T: nsIMediaDeviceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMediaDevice {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMediaDeviceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMediaDevice) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMediaDeviceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIMediaDevice, aType: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIMediaDevice, aName: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString id; */
    pub get_id: unsafe extern "C" fn (this: *const nsIMediaDevice, aId: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString mediaSource; */
    pub get_mediaSource: unsafe extern "C" fn (this: *const nsIMediaDevice, aMediaSource: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString rawId; */
    pub get_rawId: unsafe extern "C" fn (this: *const nsIMediaDevice, aRawId: *mut nsAString) -> nsresult,

    /* readonly attribute boolean scary; */
    pub get_scary: unsafe extern "C" fn (this: *const nsIMediaDevice, aScary: *mut bool) -> nsresult,

}


impl nsIMediaDevice {
    /* readonly attribute DOMString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString id; */
    #[inline]
    pub unsafe fn get_id(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_id)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString mediaSource; */
    #[inline]
    pub unsafe fn get_mediaSource(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_mediaSource)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString rawId; */
    #[inline]
    pub unsafe fn get_rawId(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_rawId)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean scary; */
    #[inline]
    pub unsafe fn get_scary(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_scary)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIGetUserMediaDevicesSuccessCallback {
    vtable: *const nsIGetUserMediaDevicesSuccessCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGetUserMediaDevicesSuccessCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x24544878, 0xd35e, 0x4962,
            [0x8c, 0x5f, 0xfb, 0x84, 0xe9, 0x7b, 0xdf, 0xee])
    }
}

unsafe impl RefCounted for nsIGetUserMediaDevicesSuccessCallback {
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
pub trait nsIGetUserMediaDevicesSuccessCallbackCoerce {
    fn coerce_from(v: &nsIGetUserMediaDevicesSuccessCallback) -> &Self;
}

impl nsIGetUserMediaDevicesSuccessCallbackCoerce for nsIGetUserMediaDevicesSuccessCallback {
    #[inline]
    fn coerce_from(v: &nsIGetUserMediaDevicesSuccessCallback) -> &Self {
        v
    }
}

impl nsIGetUserMediaDevicesSuccessCallback {
    #[inline]
    pub fn coerce<T: nsIGetUserMediaDevicesSuccessCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGetUserMediaDevicesSuccessCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIGetUserMediaDevicesSuccessCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGetUserMediaDevicesSuccessCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGetUserMediaDevicesSuccessCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onSuccess (in nsIVariant devices); */
    pub onSuccess: unsafe extern "C" fn (this: *const nsIGetUserMediaDevicesSuccessCallback, devices: *const nsIVariant) -> nsresult,

}


impl nsIGetUserMediaDevicesSuccessCallback {
    /* void onSuccess (in nsIVariant devices); */
    #[inline]
    pub unsafe fn onSuccess(&self, devices: Option<&nsIVariant>) -> Result<(), nsresult> {

        match ((*self.vtable).onSuccess)(self as *const _, devices.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIDOMGetUserMediaSuccessCallback {
    vtable: *const nsIDOMGetUserMediaSuccessCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMGetUserMediaSuccessCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf2a144fc, 0x3534, 0x4761,
            [0x8c, 0x5d, 0x98, 0x9a, 0xe7, 0x20, 0xf8, 0x9a])
    }
}

unsafe impl RefCounted for nsIDOMGetUserMediaSuccessCallback {
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
pub trait nsIDOMGetUserMediaSuccessCallbackCoerce {
    fn coerce_from(v: &nsIDOMGetUserMediaSuccessCallback) -> &Self;
}

impl nsIDOMGetUserMediaSuccessCallbackCoerce for nsIDOMGetUserMediaSuccessCallback {
    #[inline]
    fn coerce_from(v: &nsIDOMGetUserMediaSuccessCallback) -> &Self {
        v
    }
}

impl nsIDOMGetUserMediaSuccessCallback {
    #[inline]
    pub fn coerce<T: nsIDOMGetUserMediaSuccessCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMGetUserMediaSuccessCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMGetUserMediaSuccessCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMGetUserMediaSuccessCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMGetUserMediaSuccessCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onSuccess (in nsISupports value); */
    pub onSuccess: unsafe extern "C" fn (this: *const nsIDOMGetUserMediaSuccessCallback, value: *const nsISupports) -> nsresult,

}


impl nsIDOMGetUserMediaSuccessCallback {
    /* void onSuccess (in nsISupports value); */
    #[inline]
    pub unsafe fn onSuccess(&self, value: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).onSuccess)(self as *const _, value.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIDOMGetUserMediaErrorCallback {
    vtable: *const nsIDOMGetUserMediaErrorCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMGetUserMediaErrorCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x39e96c61, 0x2636, 0x4f0e,
            [0x91, 0x8e, 0x9b, 0xb6, 0x42, 0x76, 0x49, 0x2a])
    }
}

unsafe impl RefCounted for nsIDOMGetUserMediaErrorCallback {
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
pub trait nsIDOMGetUserMediaErrorCallbackCoerce {
    fn coerce_from(v: &nsIDOMGetUserMediaErrorCallback) -> &Self;
}

impl nsIDOMGetUserMediaErrorCallbackCoerce for nsIDOMGetUserMediaErrorCallback {
    #[inline]
    fn coerce_from(v: &nsIDOMGetUserMediaErrorCallback) -> &Self {
        v
    }
}

impl nsIDOMGetUserMediaErrorCallback {
    #[inline]
    pub fn coerce<T: nsIDOMGetUserMediaErrorCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMGetUserMediaErrorCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMGetUserMediaErrorCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMGetUserMediaErrorCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMGetUserMediaErrorCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onError (in nsISupports error); */
    pub onError: unsafe extern "C" fn (this: *const nsIDOMGetUserMediaErrorCallback, error: *const nsISupports) -> nsresult,

}


impl nsIDOMGetUserMediaErrorCallback {
    /* void onError (in nsISupports error); */
    #[inline]
    pub unsafe fn onError(&self, error: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).onError)(self as *const _, error.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


