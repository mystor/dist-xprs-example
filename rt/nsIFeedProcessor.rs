//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedProcessor.idl
//


#[repr(C)]
pub struct nsIFeedProcessor {
    vtable: *const nsIFeedProcessorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFeedProcessor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8a0b2908, 0x21b0, 0x45d7,
            [0xb1, 0x4d, 0x30, 0xdf, 0x0f, 0x92, 0xaf, 0xc7])
    }
}

unsafe impl RefCounted for nsIFeedProcessor {
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
pub trait nsIFeedProcessorCoerce {
    fn coerce_from(v: &nsIFeedProcessor) -> &Self;
}

impl nsIFeedProcessorCoerce for nsIFeedProcessor {
    #[inline]
    fn coerce_from(v: &nsIFeedProcessor) -> &Self {
        v
    }
}

impl nsIFeedProcessor {
    #[inline]
    pub fn coerce<T: nsIFeedProcessorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFeedProcessor {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIStreamListenerCoerce> nsIFeedProcessorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFeedProcessor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFeedProcessorVTable {
    pub __base: nsIStreamListenerVTable,

    /* attribute nsIFeedResultListener listener; */
    pub get_listener: unsafe extern "C" fn (this: *const nsIFeedProcessor, aListener: *mut *const nsIFeedResultListener) -> nsresult,
    pub set_listener: unsafe extern "C" fn (this: *const nsIFeedProcessor, aListener: *const nsIFeedResultListener) -> nsresult,

    /* void parseFromStream (in nsIInputStream stream, in nsIURI uri); */
    pub parseFromStream: unsafe extern "C" fn (this: *const nsIFeedProcessor, stream: *const nsIInputStream, uri: *const nsIURI) -> nsresult,

    /* void parseFromString (in AString str, in nsIURI uri); */
    pub parseFromString: unsafe extern "C" fn (this: *const nsIFeedProcessor, str: *const nsAString, uri: *const nsIURI) -> nsresult,

    /* void parseAsync (in nsIRequestObserver requestObserver, in nsIURI uri); */
    pub parseAsync: unsafe extern "C" fn (this: *const nsIFeedProcessor, requestObserver: *const nsIRequestObserver, uri: *const nsIURI) -> nsresult,

}


impl nsIFeedProcessor {
    /* attribute nsIFeedResultListener listener; */
    #[inline]
    pub unsafe fn get_listener(&self, ) -> Result<Option<RefPtr<nsIFeedResultListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_listener)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_listener(&self, aListener: Option<&nsIFeedResultListener>) -> Result<(), nsresult> {

        match ((*self.vtable).set_listener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void parseFromStream (in nsIInputStream stream, in nsIURI uri); */
    #[inline]
    pub unsafe fn parseFromStream(&self, stream: Option<&nsIInputStream>, uri: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).parseFromStream)(self as *const _, stream.map_or(::std::ptr::null(), |x| x as *const _), uri.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void parseFromString (in AString str, in nsIURI uri); */
    #[inline]
    pub unsafe fn parseFromString(&self, str: &[u16], uri: Option<&nsIURI>) -> Result<(), nsresult> {
        let str = nsString::from(str);
        match ((*self.vtable).parseFromString)(self as *const _, &*str, uri.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void parseAsync (in nsIRequestObserver requestObserver, in nsIURI uri); */
    #[inline]
    pub unsafe fn parseAsync(&self, requestObserver: Option<&nsIRequestObserver>, uri: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).parseAsync)(self as *const _, requestObserver.map_or(::std::ptr::null(), |x| x as *const _), uri.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


