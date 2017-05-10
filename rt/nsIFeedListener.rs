//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedListener.idl
//


#[repr(C)]
pub struct nsIFeedResultListener {
    vtable: *const nsIFeedResultListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFeedResultListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4d2ebe88, 0x36eb, 0x4e20,
            [0xbc, 0xd1, 0x99, 0x7b, 0x3c, 0x1f, 0x24, 0xce])
    }
}

unsafe impl RefCounted for nsIFeedResultListener {
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
pub trait nsIFeedResultListenerCoerce {
    fn coerce_from(v: &nsIFeedResultListener) -> &Self;
}

impl nsIFeedResultListenerCoerce for nsIFeedResultListener {
    #[inline]
    fn coerce_from(v: &nsIFeedResultListener) -> &Self {
        v
    }
}

impl nsIFeedResultListener {
    #[inline]
    pub fn coerce<T: nsIFeedResultListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFeedResultListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFeedResultListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFeedResultListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFeedResultListenerVTable {
    pub __base: nsISupportsVTable,

    /* void handleResult (in nsIFeedResult result); */
    pub handleResult: unsafe extern "C" fn (this: *const nsIFeedResultListener, result: *const nsIFeedResult) -> nsresult,

}


impl nsIFeedResultListener {
    /* void handleResult (in nsIFeedResult result); */
    #[inline]
    pub unsafe fn handleResult(&self, result: Option<&nsIFeedResult>) -> Result<(), nsresult> {

        match ((*self.vtable).handleResult)(self as *const _, result.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIFeedProgressListener {
    vtable: *const nsIFeedProgressListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFeedProgressListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xebfd5de5, 0x713c, 0x40c0,
            [0xad, 0x7c, 0xf0, 0x95, 0x11, 0x7f, 0xa5, 0x80])
    }
}

unsafe impl RefCounted for nsIFeedProgressListener {
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
pub trait nsIFeedProgressListenerCoerce {
    fn coerce_from(v: &nsIFeedProgressListener) -> &Self;
}

impl nsIFeedProgressListenerCoerce for nsIFeedProgressListener {
    #[inline]
    fn coerce_from(v: &nsIFeedProgressListener) -> &Self {
        v
    }
}

impl nsIFeedProgressListener {
    #[inline]
    pub fn coerce<T: nsIFeedProgressListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFeedProgressListener {
    type Target = nsIFeedResultListener;
    #[inline]
    fn deref(&self) -> &nsIFeedResultListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIFeedResultListenerCoerce> nsIFeedProgressListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFeedProgressListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFeedProgressListenerVTable {
    pub __base: nsIFeedResultListenerVTable,

    /* void reportError (in AString errorText, in long lineNumber, in boolean bozo); */
    pub reportError: unsafe extern "C" fn (this: *const nsIFeedProgressListener, errorText: *const nsAString, lineNumber: libc::int32_t, bozo: bool) -> nsresult,

    /* void handleStartFeed (in nsIFeedResult result); */
    pub handleStartFeed: unsafe extern "C" fn (this: *const nsIFeedProgressListener, result: *const nsIFeedResult) -> nsresult,

    /* void handleFeedAtFirstEntry (in nsIFeedResult result); */
    pub handleFeedAtFirstEntry: unsafe extern "C" fn (this: *const nsIFeedProgressListener, result: *const nsIFeedResult) -> nsresult,

    /* void handleEntry (in nsIFeedEntry entry, in nsIFeedResult result); */
    pub handleEntry: unsafe extern "C" fn (this: *const nsIFeedProgressListener, entry: *const nsIFeedEntry, result: *const nsIFeedResult) -> nsresult,

}


impl nsIFeedProgressListener {
    /* void reportError (in AString errorText, in long lineNumber, in boolean bozo); */
    #[inline]
    pub unsafe fn reportError(&self, errorText: &[u16], lineNumber: libc::int32_t, bozo: bool) -> Result<(), nsresult> {
        let errorText = nsString::from(errorText);
        match ((*self.vtable).reportError)(self as *const _, &*errorText, lineNumber, bozo) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleStartFeed (in nsIFeedResult result); */
    #[inline]
    pub unsafe fn handleStartFeed(&self, result: Option<&nsIFeedResult>) -> Result<(), nsresult> {

        match ((*self.vtable).handleStartFeed)(self as *const _, result.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleFeedAtFirstEntry (in nsIFeedResult result); */
    #[inline]
    pub unsafe fn handleFeedAtFirstEntry(&self, result: Option<&nsIFeedResult>) -> Result<(), nsresult> {

        match ((*self.vtable).handleFeedAtFirstEntry)(self as *const _, result.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleEntry (in nsIFeedEntry entry, in nsIFeedResult result); */
    #[inline]
    pub unsafe fn handleEntry(&self, entry: Option<&nsIFeedEntry>, result: Option<&nsIFeedResult>) -> Result<(), nsresult> {

        match ((*self.vtable).handleEntry)(self as *const _, entry.map_or(::std::ptr::null(), |x| x as *const _), result.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


