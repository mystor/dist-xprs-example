//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebVTTParserWrapper.idl
//


#[repr(C)]
pub struct nsIWebVTTParserWrapper {
    vtable: *const nsIWebVTTParserWrapperVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebVTTParserWrapper {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8dfe016e, 0x1701, 0x4618,
            [0x9f, 0x5e, 0x9a, 0x61, 0x54, 0xe8, 0x53, 0xf0])
    }
}

unsafe impl RefCounted for nsIWebVTTParserWrapper {
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
pub trait nsIWebVTTParserWrapperCoerce {
    fn coerce_from(v: &nsIWebVTTParserWrapper) -> &Self;
}

impl nsIWebVTTParserWrapperCoerce for nsIWebVTTParserWrapper {
    #[inline]
    fn coerce_from(v: &nsIWebVTTParserWrapper) -> &Self {
        v
    }
}

impl nsIWebVTTParserWrapper {
    #[inline]
    pub fn coerce<T: nsIWebVTTParserWrapperCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebVTTParserWrapper {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebVTTParserWrapperCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebVTTParserWrapper) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebVTTParserWrapperVTable {
    pub __base: nsISupportsVTable,

    /* void loadParser (in mozIDOMWindow window); */
    pub loadParser: unsafe extern "C" fn (this: *const nsIWebVTTParserWrapper, window: *const mozIDOMWindow) -> nsresult,

    /* void parse (in ACString data); */
    pub parse: unsafe extern "C" fn (this: *const nsIWebVTTParserWrapper, data: *const nsACString) -> nsresult,

    /* void flush (); */
    pub flush: unsafe extern "C" fn (this: *const nsIWebVTTParserWrapper) -> nsresult,

    /* void watch (in nsIWebVTTListener callback); */
    pub watch: unsafe extern "C" fn (this: *const nsIWebVTTParserWrapper, callback: *const nsIWebVTTListener) -> nsresult,

    /* nsIDOMDocumentFragment convertCueToDOMTree (in mozIDOMWindow window, in nsISupports cue); */
    pub convertCueToDOMTree: unsafe extern "C" fn (this: *const nsIWebVTTParserWrapper, window: *const mozIDOMWindow, cue: *const nsISupports, _retval: *mut *const nsIDOMDocumentFragment) -> nsresult,

    /* void processCues (in mozIDOMWindow window, in nsIVariant cues, in nsISupports overlay, in nsISupports controls); */
    pub processCues: unsafe extern "C" fn (this: *const nsIWebVTTParserWrapper, window: *const mozIDOMWindow, cues: *const nsIVariant, overlay: *const nsISupports, controls: *const nsISupports) -> nsresult,

}


impl nsIWebVTTParserWrapper {
    /* void loadParser (in mozIDOMWindow window); */
    #[inline]
    pub unsafe fn loadParser(&self, window: Option<&mozIDOMWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).loadParser)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void parse (in ACString data); */
    #[inline]
    pub unsafe fn parse(&self, data: &[u8]) -> Result<(), nsresult> {
        let data = nsCString::from(data);
        match ((*self.vtable).parse)(self as *const _, &*data) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void flush (); */
    #[inline]
    pub unsafe fn flush(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).flush)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void watch (in nsIWebVTTListener callback); */
    #[inline]
    pub unsafe fn watch(&self, callback: Option<&nsIWebVTTListener>) -> Result<(), nsresult> {

        match ((*self.vtable).watch)(self as *const _, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMDocumentFragment convertCueToDOMTree (in mozIDOMWindow window, in nsISupports cue); */
    #[inline]
    pub unsafe fn convertCueToDOMTree(&self, window: Option<&mozIDOMWindow>, cue: Option<&nsISupports>) -> Result<Option<RefPtr<nsIDOMDocumentFragment>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).convertCueToDOMTree)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), cue.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void processCues (in mozIDOMWindow window, in nsIVariant cues, in nsISupports overlay, in nsISupports controls); */
    #[inline]
    pub unsafe fn processCues(&self, window: Option<&mozIDOMWindow>, cues: Option<&nsIVariant>, overlay: Option<&nsISupports>, controls: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).processCues)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), cues.map_or(::std::ptr::null(), |x| x as *const _), overlay.map_or(::std::ptr::null(), |x| x as *const _), controls.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


