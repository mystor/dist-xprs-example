//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURILoader.idl
//


pub mod nsIURILoader_consts {
    pub const IS_CONTENT_PREFERRED: i64 = 1;
    pub const DONT_RETARGET: i64 = 2;
}


#[repr(C)]
pub struct nsIURILoader {
    vtable: *const nsIURILoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIURILoader {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8762c4e7, 0xbe35, 0x4958,
            [0x9b, 0x81, 0xa0, 0x56, 0x85, 0xbb, 0x51, 0x6d])
    }
}

unsafe impl RefCounted for nsIURILoader {
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
pub trait nsIURILoaderCoerce {
    fn coerce_from(v: &nsIURILoader) -> &Self;
}

impl nsIURILoaderCoerce for nsIURILoader {
    #[inline]
    fn coerce_from(v: &nsIURILoader) -> &Self {
        v
    }
}

impl nsIURILoader {
    #[inline]
    pub fn coerce<T: nsIURILoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIURILoader {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIURILoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURILoader) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIURILoaderVTable {
    pub __base: nsISupportsVTable,

    /* void registerContentListener (in nsIURIContentListener aContentListener); */
    pub registerContentListener: unsafe extern "C" fn (this: *const nsIURILoader, aContentListener: *const nsIURIContentListener) -> nsresult,

    /* void unRegisterContentListener (in nsIURIContentListener aContentListener); */
    pub unRegisterContentListener: unsafe extern "C" fn (this: *const nsIURILoader, aContentListener: *const nsIURIContentListener) -> nsresult,

    /* void openURI (in nsIChannel aChannel, in unsigned long aFlags, in nsIInterfaceRequestor aWindowContext); */
    pub openURI: unsafe extern "C" fn (this: *const nsIURILoader, aChannel: *const nsIChannel, aFlags: libc::uint32_t, aWindowContext: *const nsIInterfaceRequestor) -> nsresult,

    /* nsIStreamListener openChannel (in nsIChannel aChannel, in unsigned long aFlags, in nsIInterfaceRequestor aWindowContext); */
    pub openChannel: unsafe extern "C" fn (this: *const nsIURILoader, aChannel: *const nsIChannel, aFlags: libc::uint32_t, aWindowContext: *const nsIInterfaceRequestor, _retval: *mut *const nsIStreamListener) -> nsresult,

    /* void stop (in nsISupports aLoadCookie); */
    pub stop: unsafe extern "C" fn (this: *const nsIURILoader, aLoadCookie: *const nsISupports) -> nsresult,

}


impl nsIURILoader {
    /* void registerContentListener (in nsIURIContentListener aContentListener); */
    #[inline]
    pub unsafe fn registerContentListener(&self, aContentListener: Option<&nsIURIContentListener>) -> Result<(), nsresult> {

        match ((*self.vtable).registerContentListener)(self as *const _, aContentListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unRegisterContentListener (in nsIURIContentListener aContentListener); */
    #[inline]
    pub unsafe fn unRegisterContentListener(&self, aContentListener: Option<&nsIURIContentListener>) -> Result<(), nsresult> {

        match ((*self.vtable).unRegisterContentListener)(self as *const _, aContentListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void openURI (in nsIChannel aChannel, in unsigned long aFlags, in nsIInterfaceRequestor aWindowContext); */
    #[inline]
    pub unsafe fn openURI(&self, aChannel: Option<&nsIChannel>, aFlags: libc::uint32_t, aWindowContext: Option<&nsIInterfaceRequestor>) -> Result<(), nsresult> {

        match ((*self.vtable).openURI)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), aFlags, aWindowContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIStreamListener openChannel (in nsIChannel aChannel, in unsigned long aFlags, in nsIInterfaceRequestor aWindowContext); */
    #[inline]
    pub unsafe fn openChannel(&self, aChannel: Option<&nsIChannel>, aFlags: libc::uint32_t, aWindowContext: Option<&nsIInterfaceRequestor>) -> Result<Option<RefPtr<nsIStreamListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openChannel)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), aFlags, aWindowContext.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void stop (in nsISupports aLoadCookie); */
    #[inline]
    pub unsafe fn stop(&self, aLoadCookie: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).stop)(self as *const _, aLoadCookie.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


