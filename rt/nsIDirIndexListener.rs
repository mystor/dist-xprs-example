//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDirIndexListener.idl
//


#[repr(C)]
pub struct nsIDirIndexListener {
    vtable: *const nsIDirIndexListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDirIndexListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfae4e9a8, 0x1dd1, 0x11b2,
            [0xb5, 0x3c, 0x8f, 0x3a, 0xa1, 0xbb, 0xf8, 0xf5])
    }
}

unsafe impl RefCounted for nsIDirIndexListener {
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
pub trait nsIDirIndexListenerCoerce {
    fn coerce_from(v: &nsIDirIndexListener) -> &Self;
}

impl nsIDirIndexListenerCoerce for nsIDirIndexListener {
    #[inline]
    fn coerce_from(v: &nsIDirIndexListener) -> &Self {
        v
    }
}

impl nsIDirIndexListener {
    #[inline]
    pub fn coerce<T: nsIDirIndexListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDirIndexListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDirIndexListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDirIndexListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDirIndexListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onIndexAvailable (in nsIRequest aRequest, in nsISupports aCtxt, in nsIDirIndex aIndex); */
    pub onIndexAvailable: unsafe extern "C" fn (this: *const nsIDirIndexListener, aRequest: *const nsIRequest, aCtxt: *const nsISupports, aIndex: *const nsIDirIndex) -> nsresult,

    /* void onInformationAvailable (in nsIRequest aRequest, in nsISupports aCtxt, in AString aInfo); */
    pub onInformationAvailable: unsafe extern "C" fn (this: *const nsIDirIndexListener, aRequest: *const nsIRequest, aCtxt: *const nsISupports, aInfo: *const nsAString) -> nsresult,

}


impl nsIDirIndexListener {
    /* void onIndexAvailable (in nsIRequest aRequest, in nsISupports aCtxt, in nsIDirIndex aIndex); */
    #[inline]
    pub unsafe fn onIndexAvailable(&self, aRequest: Option<&nsIRequest>, aCtxt: Option<&nsISupports>, aIndex: Option<&nsIDirIndex>) -> Result<(), nsresult> {

        match ((*self.vtable).onIndexAvailable)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _), aCtxt.map_or(::std::ptr::null(), |x| x as *const _), aIndex.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onInformationAvailable (in nsIRequest aRequest, in nsISupports aCtxt, in AString aInfo); */
    #[inline]
    pub unsafe fn onInformationAvailable(&self, aRequest: Option<&nsIRequest>, aCtxt: Option<&nsISupports>, aInfo: &[u16]) -> Result<(), nsresult> {
        let aInfo = nsString::from(aInfo);
        match ((*self.vtable).onInformationAvailable)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _), aCtxt.map_or(::std::ptr::null(), |x| x as *const _), &*aInfo) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIDirIndexParser {
    vtable: *const nsIDirIndexParserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDirIndexParser {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x38e3066c, 0x1dd2, 0x11b2,
            [0x9b, 0x59, 0x8b, 0xe5, 0x15, 0xc1, 0xee, 0x3f])
    }
}

unsafe impl RefCounted for nsIDirIndexParser {
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
pub trait nsIDirIndexParserCoerce {
    fn coerce_from(v: &nsIDirIndexParser) -> &Self;
}

impl nsIDirIndexParserCoerce for nsIDirIndexParser {
    #[inline]
    fn coerce_from(v: &nsIDirIndexParser) -> &Self {
        v
    }
}

impl nsIDirIndexParser {
    #[inline]
    pub fn coerce<T: nsIDirIndexParserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDirIndexParser {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIStreamListenerCoerce> nsIDirIndexParserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDirIndexParser) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDirIndexParserVTable {
    pub __base: nsIStreamListenerVTable,

    /* attribute nsIDirIndexListener listener; */
    pub get_listener: unsafe extern "C" fn (this: *const nsIDirIndexParser, aListener: *mut *const nsIDirIndexListener) -> nsresult,
    pub set_listener: unsafe extern "C" fn (this: *const nsIDirIndexParser, aListener: *const nsIDirIndexListener) -> nsresult,

    /* readonly attribute string comment; */
    pub get_comment: unsafe extern "C" fn (this: *const nsIDirIndexParser, aComment: *mut *const libc::c_char) -> nsresult,

    /* attribute string encoding; */
    pub get_encoding: unsafe extern "C" fn (this: *const nsIDirIndexParser, aEncoding: *mut *const libc::c_char) -> nsresult,
    pub set_encoding: unsafe extern "C" fn (this: *const nsIDirIndexParser, aEncoding: *const libc::c_char) -> nsresult,

}


impl nsIDirIndexParser {
    /* attribute nsIDirIndexListener listener; */
    #[inline]
    pub unsafe fn get_listener(&self, ) -> Result<Option<RefPtr<nsIDirIndexListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_listener)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_listener(&self, aListener: Option<&nsIDirIndexListener>) -> Result<(), nsresult> {

        match ((*self.vtable).set_listener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute string comment; */
    #[inline]
    pub unsafe fn get_comment(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_comment)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute string encoding; */
    #[inline]
    pub unsafe fn get_encoding(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_encoding)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_encoding(&self, aEncoding: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).set_encoding)(self as *const _, aEncoding) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


