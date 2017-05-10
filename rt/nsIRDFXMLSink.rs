//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFXMLSink.idl
//


#[repr(C)]
pub struct nsIRDFXMLSinkObserver {
    vtable: *const nsIRDFXMLSinkObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFXMLSinkObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xeb1a5d30, 0xab33, 0x11d2,
            [0x8e, 0xc6, 0x00, 0x80, 0x5f, 0x29, 0xf3, 0x70])
    }
}

unsafe impl RefCounted for nsIRDFXMLSinkObserver {
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
pub trait nsIRDFXMLSinkObserverCoerce {
    fn coerce_from(v: &nsIRDFXMLSinkObserver) -> &Self;
}

impl nsIRDFXMLSinkObserverCoerce for nsIRDFXMLSinkObserver {
    #[inline]
    fn coerce_from(v: &nsIRDFXMLSinkObserver) -> &Self {
        v
    }
}

impl nsIRDFXMLSinkObserver {
    #[inline]
    pub fn coerce<T: nsIRDFXMLSinkObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFXMLSinkObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFXMLSinkObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFXMLSinkObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFXMLSinkObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onBeginLoad (in nsIRDFXMLSink aSink); */
    pub onBeginLoad: unsafe extern "C" fn (this: *const nsIRDFXMLSinkObserver, aSink: *const nsIRDFXMLSink) -> nsresult,

    /* void onInterrupt (in nsIRDFXMLSink aSink); */
    pub onInterrupt: unsafe extern "C" fn (this: *const nsIRDFXMLSinkObserver, aSink: *const nsIRDFXMLSink) -> nsresult,

    /* void onResume (in nsIRDFXMLSink aSink); */
    pub onResume: unsafe extern "C" fn (this: *const nsIRDFXMLSinkObserver, aSink: *const nsIRDFXMLSink) -> nsresult,

    /* void onEndLoad (in nsIRDFXMLSink aSink); */
    pub onEndLoad: unsafe extern "C" fn (this: *const nsIRDFXMLSinkObserver, aSink: *const nsIRDFXMLSink) -> nsresult,

    /* void onError (in nsIRDFXMLSink aSink, in nsresult aStatus, in wstring aErrorMsg); */
    pub onError: unsafe extern "C" fn (this: *const nsIRDFXMLSinkObserver, aSink: *const nsIRDFXMLSink, aStatus: nsresult, aErrorMsg: *const libc::int16_t) -> nsresult,

}


impl nsIRDFXMLSinkObserver {
    /* void onBeginLoad (in nsIRDFXMLSink aSink); */
    #[inline]
    pub unsafe fn onBeginLoad(&self, aSink: Option<&nsIRDFXMLSink>) -> Result<(), nsresult> {

        match ((*self.vtable).onBeginLoad)(self as *const _, aSink.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onInterrupt (in nsIRDFXMLSink aSink); */
    #[inline]
    pub unsafe fn onInterrupt(&self, aSink: Option<&nsIRDFXMLSink>) -> Result<(), nsresult> {

        match ((*self.vtable).onInterrupt)(self as *const _, aSink.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onResume (in nsIRDFXMLSink aSink); */
    #[inline]
    pub unsafe fn onResume(&self, aSink: Option<&nsIRDFXMLSink>) -> Result<(), nsresult> {

        match ((*self.vtable).onResume)(self as *const _, aSink.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onEndLoad (in nsIRDFXMLSink aSink); */
    #[inline]
    pub unsafe fn onEndLoad(&self, aSink: Option<&nsIRDFXMLSink>) -> Result<(), nsresult> {

        match ((*self.vtable).onEndLoad)(self as *const _, aSink.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onError (in nsIRDFXMLSink aSink, in nsresult aStatus, in wstring aErrorMsg); */
    #[inline]
    pub unsafe fn onError(&self, aSink: Option<&nsIRDFXMLSink>, aStatus: nsresult, aErrorMsg: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).onError)(self as *const _, aSink.map_or(::std::ptr::null(), |x| x as *const _), aStatus, aErrorMsg) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIRDFXMLSink {
    vtable: *const nsIRDFXMLSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFXMLSink {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xeb1a5d31, 0xab33, 0x11d2,
            [0x8e, 0xc6, 0x00, 0x80, 0x5f, 0x29, 0xf3, 0x70])
    }
}

unsafe impl RefCounted for nsIRDFXMLSink {
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
pub trait nsIRDFXMLSinkCoerce {
    fn coerce_from(v: &nsIRDFXMLSink) -> &Self;
}

impl nsIRDFXMLSinkCoerce for nsIRDFXMLSink {
    #[inline]
    fn coerce_from(v: &nsIRDFXMLSink) -> &Self {
        v
    }
}

impl nsIRDFXMLSink {
    #[inline]
    pub fn coerce<T: nsIRDFXMLSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFXMLSink {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFXMLSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFXMLSink) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFXMLSinkVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean readOnly; */
    pub get_readOnly: unsafe extern "C" fn (this: *const nsIRDFXMLSink, aReadOnly: *mut bool) -> nsresult,
    pub set_readOnly: unsafe extern "C" fn (this: *const nsIRDFXMLSink, aReadOnly: bool) -> nsresult,

    /* void beginLoad (); */
    pub beginLoad: unsafe extern "C" fn (this: *const nsIRDFXMLSink) -> nsresult,

    /* void interrupt (); */
    pub interrupt: unsafe extern "C" fn (this: *const nsIRDFXMLSink) -> nsresult,

    /* void resume (); */
    pub resume: unsafe extern "C" fn (this: *const nsIRDFXMLSink) -> nsresult,

    /* void endLoad (); */
    pub endLoad: unsafe extern "C" fn (this: *const nsIRDFXMLSink) -> nsresult,

    /* [noscript] void addNameSpace (in nsIAtomPtr aPrefix, [const] in nsStringRef aURI); */
    /// Unable to call function as its signature contains a non-rust type
    pub addNameSpace: *const ::libc::c_void,

    /* void addXMLSinkObserver (in nsIRDFXMLSinkObserver aObserver); */
    pub addXMLSinkObserver: unsafe extern "C" fn (this: *const nsIRDFXMLSink, aObserver: *const nsIRDFXMLSinkObserver) -> nsresult,

    /* void removeXMLSinkObserver (in nsIRDFXMLSinkObserver aObserver); */
    pub removeXMLSinkObserver: unsafe extern "C" fn (this: *const nsIRDFXMLSink, aObserver: *const nsIRDFXMLSinkObserver) -> nsresult,

}


impl nsIRDFXMLSink {
    /* attribute boolean readOnly; */
    #[inline]
    pub unsafe fn get_readOnly(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_readOnly)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_readOnly(&self, aReadOnly: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_readOnly)(self as *const _, aReadOnly) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void beginLoad (); */
    #[inline]
    pub unsafe fn beginLoad(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).beginLoad)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void interrupt (); */
    #[inline]
    pub unsafe fn interrupt(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).interrupt)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resume (); */
    #[inline]
    pub unsafe fn resume(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resume)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endLoad (); */
    #[inline]
    pub unsafe fn endLoad(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).endLoad)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void addNameSpace (in nsIAtomPtr aPrefix, [const] in nsStringRef aURI); */


    /* void addXMLSinkObserver (in nsIRDFXMLSinkObserver aObserver); */
    #[inline]
    pub unsafe fn addXMLSinkObserver(&self, aObserver: Option<&nsIRDFXMLSinkObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).addXMLSinkObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeXMLSinkObserver (in nsIRDFXMLSinkObserver aObserver); */
    #[inline]
    pub unsafe fn removeXMLSinkObserver(&self, aObserver: Option<&nsIRDFXMLSinkObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeXMLSinkObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


