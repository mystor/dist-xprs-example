//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIncrementalStreamLoader.idl
//


#[repr(C)]
pub struct nsIIncrementalStreamLoaderObserver {
    vtable: *const nsIIncrementalStreamLoaderObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIncrementalStreamLoaderObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x07c3d2cc, 0x5454, 0x4618,
            [0x9f, 0x4f, 0xcd, 0x93, 0xde, 0x95, 0x04, 0xa4])
    }
}

unsafe impl RefCounted for nsIIncrementalStreamLoaderObserver {
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
pub trait nsIIncrementalStreamLoaderObserverCoerce {
    fn coerce_from(v: &nsIIncrementalStreamLoaderObserver) -> &Self;
}

impl nsIIncrementalStreamLoaderObserverCoerce for nsIIncrementalStreamLoaderObserver {
    #[inline]
    fn coerce_from(v: &nsIIncrementalStreamLoaderObserver) -> &Self {
        v
    }
}

impl nsIIncrementalStreamLoaderObserver {
    #[inline]
    pub fn coerce<T: nsIIncrementalStreamLoaderObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIncrementalStreamLoaderObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIIncrementalStreamLoaderObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIncrementalStreamLoaderObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIncrementalStreamLoaderObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onIncrementalData (in nsIIncrementalStreamLoader loader, in nsISupports ctxt, in unsigned long dataLength, [array, size_is (dataLength), const] in octet data, inout unsigned long consumedLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub onIncrementalData: *const ::libc::c_void,

    /* void onStreamComplete (in nsIIncrementalStreamLoader loader, in nsISupports ctxt, in nsresult status, in unsigned long resultLength, [array, size_is (resultLength), const] in octet result); */
    /// Unable to call function as its signature contains a non-rust type
    pub onStreamComplete: *const ::libc::c_void,

}


impl nsIIncrementalStreamLoaderObserver {
    /* void onIncrementalData (in nsIIncrementalStreamLoader loader, in nsISupports ctxt, in unsigned long dataLength, [array, size_is (dataLength), const] in octet data, inout unsigned long consumedLength); */


    /* void onStreamComplete (in nsIIncrementalStreamLoader loader, in nsISupports ctxt, in nsresult status, in unsigned long resultLength, [array, size_is (resultLength), const] in octet result); */


}


#[repr(C)]
pub struct nsIIncrementalStreamLoader {
    vtable: *const nsIIncrementalStreamLoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIncrementalStreamLoader {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa023b060, 0xba23, 0x431a,
            [0xb4, 0x49, 0x2d, 0xd6, 0x3e, 0x22, 0x05, 0x54])
    }
}

unsafe impl RefCounted for nsIIncrementalStreamLoader {
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
pub trait nsIIncrementalStreamLoaderCoerce {
    fn coerce_from(v: &nsIIncrementalStreamLoader) -> &Self;
}

impl nsIIncrementalStreamLoaderCoerce for nsIIncrementalStreamLoader {
    #[inline]
    fn coerce_from(v: &nsIIncrementalStreamLoader) -> &Self {
        v
    }
}

impl nsIIncrementalStreamLoader {
    #[inline]
    pub fn coerce<T: nsIIncrementalStreamLoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIncrementalStreamLoader {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIStreamListenerCoerce> nsIIncrementalStreamLoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIncrementalStreamLoader) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIncrementalStreamLoaderVTable {
    pub __base: nsIStreamListenerVTable,

    /* void init (in nsIIncrementalStreamLoaderObserver aObserver); */
    pub init: unsafe extern "C" fn (this: *const nsIIncrementalStreamLoader, aObserver: *const nsIIncrementalStreamLoaderObserver) -> nsresult,

    /* readonly attribute unsigned long numBytesRead; */
    pub get_numBytesRead: unsafe extern "C" fn (this: *const nsIIncrementalStreamLoader, aNumBytesRead: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIRequest request; */
    pub get_request: unsafe extern "C" fn (this: *const nsIIncrementalStreamLoader, aRequest: *mut *const nsIRequest) -> nsresult,

}


impl nsIIncrementalStreamLoader {
    /* void init (in nsIIncrementalStreamLoaderObserver aObserver); */
    #[inline]
    pub unsafe fn init(&self, aObserver: Option<&nsIIncrementalStreamLoaderObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
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


