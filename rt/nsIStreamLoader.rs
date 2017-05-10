//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamLoader.idl
//


#[repr(C)]
pub struct nsIStreamLoaderObserver {
    vtable: *const nsIStreamLoaderObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStreamLoaderObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x359f7990, 0xd4e9, 0x11d3,
            [0xa1, 0xa5, 0x00, 0x50, 0x04, 0x1c, 0xaf, 0x44])
    }
}

unsafe impl RefCounted for nsIStreamLoaderObserver {
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
pub trait nsIStreamLoaderObserverCoerce {
    fn coerce_from(v: &nsIStreamLoaderObserver) -> &Self;
}

impl nsIStreamLoaderObserverCoerce for nsIStreamLoaderObserver {
    #[inline]
    fn coerce_from(v: &nsIStreamLoaderObserver) -> &Self {
        v
    }
}

impl nsIStreamLoaderObserver {
    #[inline]
    pub fn coerce<T: nsIStreamLoaderObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStreamLoaderObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStreamLoaderObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamLoaderObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStreamLoaderObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onStreamComplete (in nsIStreamLoader loader, in nsISupports ctxt, in nsresult status, in unsigned long resultLength, [array, size_is (resultLength), const] in octet result); */
    /// Unable to call function as its signature contains a non-rust type
    pub onStreamComplete: *const ::libc::c_void,

}


impl nsIStreamLoaderObserver {
    /* void onStreamComplete (in nsIStreamLoader loader, in nsISupports ctxt, in nsresult status, in unsigned long resultLength, [array, size_is (resultLength), const] in octet result); */


}


#[repr(C)]
pub struct nsIStreamLoader {
    vtable: *const nsIStreamLoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStreamLoader {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x323bcff1, 0x7513, 0x4e1f,
            [0xa5, 0x41, 0x1c, 0x92, 0x13, 0xc2, 0xed, 0x1b])
    }
}

unsafe impl RefCounted for nsIStreamLoader {
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
pub trait nsIStreamLoaderCoerce {
    fn coerce_from(v: &nsIStreamLoader) -> &Self;
}

impl nsIStreamLoaderCoerce for nsIStreamLoader {
    #[inline]
    fn coerce_from(v: &nsIStreamLoader) -> &Self {
        v
    }
}

impl nsIStreamLoader {
    #[inline]
    pub fn coerce<T: nsIStreamLoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStreamLoader {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIStreamListenerCoerce> nsIStreamLoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamLoader) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStreamLoaderVTable {
    pub __base: nsIStreamListenerVTable,

    /* void init (in nsIStreamLoaderObserver aStreamObserver, [optional] in nsIRequestObserver aRequestObserver); */
    pub init: unsafe extern "C" fn (this: *const nsIStreamLoader, aStreamObserver: *const nsIStreamLoaderObserver, aRequestObserver: *const nsIRequestObserver) -> nsresult,

    /* readonly attribute unsigned long numBytesRead; */
    pub get_numBytesRead: unsafe extern "C" fn (this: *const nsIStreamLoader, aNumBytesRead: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIRequest request; */
    pub get_request: unsafe extern "C" fn (this: *const nsIStreamLoader, aRequest: *mut *const nsIRequest) -> nsresult,

}


impl nsIStreamLoader {
    /* void init (in nsIStreamLoaderObserver aStreamObserver, [optional] in nsIRequestObserver aRequestObserver); */
    #[inline]
    pub unsafe fn init(&self, aStreamObserver: Option<&nsIStreamLoaderObserver>, aRequestObserver: Option<&nsIRequestObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aStreamObserver.map_or(::std::ptr::null(), |x| x as *const _), aRequestObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long numBytesRead; */
    #[inline]
    pub unsafe fn get_numBytesRead(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_numBytesRead)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIRequest request; */
    #[inline]
    pub unsafe fn get_request(&self, ) -> Result<Option<RefPtr<nsIRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_request)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


