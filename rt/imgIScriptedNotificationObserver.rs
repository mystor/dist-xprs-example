//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgIScriptedNotificationObserver.idl
//


#[repr(C)]
pub struct imgIScriptedNotificationObserver {
    vtable: *const imgIScriptedNotificationObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for imgIScriptedNotificationObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x10be55b3, 0x2029, 0x41a7,
            [0xa9, 0x75, 0x53, 0x8e, 0xfe, 0xd2, 0x50, 0xed])
    }
}

unsafe impl RefCounted for imgIScriptedNotificationObserver {
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
pub trait imgIScriptedNotificationObserverCoerce {
    fn coerce_from(v: &imgIScriptedNotificationObserver) -> &Self;
}

impl imgIScriptedNotificationObserverCoerce for imgIScriptedNotificationObserver {
    #[inline]
    fn coerce_from(v: &imgIScriptedNotificationObserver) -> &Self {
        v
    }
}

impl imgIScriptedNotificationObserver {
    #[inline]
    pub fn coerce<T: imgIScriptedNotificationObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for imgIScriptedNotificationObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> imgIScriptedNotificationObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &imgIScriptedNotificationObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct imgIScriptedNotificationObserverVTable {
    pub __base: nsISupportsVTable,

    /* void sizeAvailable (in imgIRequest aRequest); */
    pub sizeAvailable: unsafe extern "C" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> nsresult,

    /* void frameUpdate (in imgIRequest aRequest); */
    pub frameUpdate: unsafe extern "C" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> nsresult,

    /* void frameComplete (in imgIRequest aRequest); */
    pub frameComplete: unsafe extern "C" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> nsresult,

    /* void loadComplete (in imgIRequest aRequest); */
    pub loadComplete: unsafe extern "C" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> nsresult,

    /* void decodeComplete (in imgIRequest aRequest); */
    pub decodeComplete: unsafe extern "C" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> nsresult,

    /* void discard (in imgIRequest aRequest); */
    pub discard: unsafe extern "C" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> nsresult,

    /* void isAnimated (in imgIRequest aRequest); */
    pub isAnimated: unsafe extern "C" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> nsresult,

    /* void hasTransparency (in imgIRequest aRequest); */
    pub hasTransparency: unsafe extern "C" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> nsresult,

}


impl imgIScriptedNotificationObserver {
    /* void sizeAvailable (in imgIRequest aRequest); */
    #[inline]
    pub unsafe fn sizeAvailable(&self, aRequest: Option<&imgIRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).sizeAvailable)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void frameUpdate (in imgIRequest aRequest); */
    #[inline]
    pub unsafe fn frameUpdate(&self, aRequest: Option<&imgIRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).frameUpdate)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void frameComplete (in imgIRequest aRequest); */
    #[inline]
    pub unsafe fn frameComplete(&self, aRequest: Option<&imgIRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).frameComplete)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void loadComplete (in imgIRequest aRequest); */
    #[inline]
    pub unsafe fn loadComplete(&self, aRequest: Option<&imgIRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).loadComplete)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void decodeComplete (in imgIRequest aRequest); */
    #[inline]
    pub unsafe fn decodeComplete(&self, aRequest: Option<&imgIRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).decodeComplete)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void discard (in imgIRequest aRequest); */
    #[inline]
    pub unsafe fn discard(&self, aRequest: Option<&imgIRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).discard)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void isAnimated (in imgIRequest aRequest); */
    #[inline]
    pub unsafe fn isAnimated(&self, aRequest: Option<&imgIRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).isAnimated)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void hasTransparency (in imgIRequest aRequest); */
    #[inline]
    pub unsafe fn hasTransparency(&self, aRequest: Option<&imgIRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).hasTransparency)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


