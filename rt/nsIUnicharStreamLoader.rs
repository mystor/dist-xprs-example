//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUnicharStreamLoader.idl
//


#[repr(C)]
pub struct nsIUnicharStreamLoaderObserver {
    vtable: *const nsIUnicharStreamLoaderObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUnicharStreamLoaderObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc2982b39, 0x2e48, 0x429e,
            [0x92, 0xb7, 0x99, 0x34, 0x8a, 0x16, 0x33, 0xc5])
    }
}

unsafe impl RefCounted for nsIUnicharStreamLoaderObserver {
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
pub trait nsIUnicharStreamLoaderObserverCoerce {
    fn coerce_from(v: &nsIUnicharStreamLoaderObserver) -> &Self;
}

impl nsIUnicharStreamLoaderObserverCoerce for nsIUnicharStreamLoaderObserver {
    #[inline]
    fn coerce_from(v: &nsIUnicharStreamLoaderObserver) -> &Self {
        v
    }
}

impl nsIUnicharStreamLoaderObserver {
    #[inline]
    pub fn coerce<T: nsIUnicharStreamLoaderObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUnicharStreamLoaderObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUnicharStreamLoaderObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUnicharStreamLoaderObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUnicharStreamLoaderObserverVTable {
    pub __base: nsISupportsVTable,

    /* ACString onDetermineCharset (in nsIUnicharStreamLoader aLoader, in nsISupports aContext, in ACString aSegment); */
    pub onDetermineCharset: unsafe extern "C" fn (this: *const nsIUnicharStreamLoaderObserver, aLoader: *const nsIUnicharStreamLoader, aContext: *const nsISupports, aSegment: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* void onStreamComplete (in nsIUnicharStreamLoader aLoader, in nsISupports aContext, in nsresult aStatus, in AString aBuffer); */
    pub onStreamComplete: unsafe extern "C" fn (this: *const nsIUnicharStreamLoaderObserver, aLoader: *const nsIUnicharStreamLoader, aContext: *const nsISupports, aStatus: nsresult, aBuffer: *const nsAString) -> nsresult,

}


impl nsIUnicharStreamLoaderObserver {
    /* ACString onDetermineCharset (in nsIUnicharStreamLoader aLoader, in nsISupports aContext, in ACString aSegment); */
    #[inline]
    pub unsafe fn onDetermineCharset(&self, aLoader: Option<&nsIUnicharStreamLoader>, aContext: Option<&nsISupports>, aSegment: &[u8]) -> Result<nsCString, nsresult> {
        let aSegment = nsCString::from(aSegment);
        let mut _retval = nsCString::new();
        match ((*self.vtable).onDetermineCharset)(self as *const _, aLoader.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _), &*aSegment, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void onStreamComplete (in nsIUnicharStreamLoader aLoader, in nsISupports aContext, in nsresult aStatus, in AString aBuffer); */
    #[inline]
    pub unsafe fn onStreamComplete(&self, aLoader: Option<&nsIUnicharStreamLoader>, aContext: Option<&nsISupports>, aStatus: nsresult, aBuffer: &[u16]) -> Result<(), nsresult> {
        let aBuffer = nsString::from(aBuffer);
        match ((*self.vtable).onStreamComplete)(self as *const _, aLoader.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _), aStatus, &*aBuffer) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIUnicharStreamLoader {
    vtable: *const nsIUnicharStreamLoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUnicharStreamLoader {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xafb62060, 0x37c7, 0x4713,
            [0x8a, 0x84, 0x4a, 0x0c, 0x11, 0x99, 0xba, 0x5c])
    }
}

unsafe impl RefCounted for nsIUnicharStreamLoader {
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
pub trait nsIUnicharStreamLoaderCoerce {
    fn coerce_from(v: &nsIUnicharStreamLoader) -> &Self;
}

impl nsIUnicharStreamLoaderCoerce for nsIUnicharStreamLoader {
    #[inline]
    fn coerce_from(v: &nsIUnicharStreamLoader) -> &Self {
        v
    }
}

impl nsIUnicharStreamLoader {
    #[inline]
    pub fn coerce<T: nsIUnicharStreamLoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUnicharStreamLoader {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIStreamListenerCoerce> nsIUnicharStreamLoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUnicharStreamLoader) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUnicharStreamLoaderVTable {
    pub __base: nsIStreamListenerVTable,

    /* void init (in nsIUnicharStreamLoaderObserver aObserver); */
    pub init: unsafe extern "C" fn (this: *const nsIUnicharStreamLoader, aObserver: *const nsIUnicharStreamLoaderObserver) -> nsresult,

    /* readonly attribute nsIChannel channel; */
    pub get_channel: unsafe extern "C" fn (this: *const nsIUnicharStreamLoader, aChannel: *mut *const nsIChannel) -> nsresult,

    /* readonly attribute ACString charset; */
    pub get_charset: unsafe extern "C" fn (this: *const nsIUnicharStreamLoader, aCharset: *mut nsACString) -> nsresult,

    /* readonly attribute ACString rawBuffer; */
    pub get_rawBuffer: unsafe extern "C" fn (this: *const nsIUnicharStreamLoader, aRawBuffer: *mut nsACString) -> nsresult,

}


impl nsIUnicharStreamLoader {
    /* void init (in nsIUnicharStreamLoaderObserver aObserver); */
    #[inline]
    pub unsafe fn init(&self, aObserver: Option<&nsIUnicharStreamLoaderObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIChannel channel; */
    #[inline]
    pub unsafe fn get_channel(&self, ) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_channel)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute ACString charset; */
    #[inline]
    pub unsafe fn get_charset(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_charset)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString rawBuffer; */
    #[inline]
    pub unsafe fn get_rawBuffer(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_rawBuffer)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


