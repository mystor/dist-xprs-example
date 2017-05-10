//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURIContentListener.idl
//


#[repr(C)]
pub struct nsIURIContentListener {
    vtable: *const nsIURIContentListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIURIContentListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x10a28f38, 0x32e8, 0x4c63,
            [0x8a, 0xa1, 0x12, 0xea, 0xae, 0xbc, 0x36, 0x9a])
    }
}

unsafe impl RefCounted for nsIURIContentListener {
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
pub trait nsIURIContentListenerCoerce {
    fn coerce_from(v: &nsIURIContentListener) -> &Self;
}

impl nsIURIContentListenerCoerce for nsIURIContentListener {
    #[inline]
    fn coerce_from(v: &nsIURIContentListener) -> &Self {
        v
    }
}

impl nsIURIContentListener {
    #[inline]
    pub fn coerce<T: nsIURIContentListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIURIContentListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIURIContentListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURIContentListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIURIContentListenerVTable {
    pub __base: nsISupportsVTable,

    /* boolean onStartURIOpen (in nsIURI aURI); */
    pub onStartURIOpen: unsafe extern "C" fn (this: *const nsIURIContentListener, aURI: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* boolean doContent (in ACString aContentType, in boolean aIsContentPreferred, in nsIRequest aRequest, out nsIStreamListener aContentHandler); */
    pub doContent: unsafe extern "C" fn (this: *const nsIURIContentListener, aContentType: *const nsACString, aIsContentPreferred: bool, aRequest: *const nsIRequest, aContentHandler: *mut *const nsIStreamListener, _retval: *mut bool) -> nsresult,

    /* boolean isPreferred (in string aContentType, out string aDesiredContentType); */
    pub isPreferred: unsafe extern "C" fn (this: *const nsIURIContentListener, aContentType: *const libc::c_char, aDesiredContentType: *mut *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* boolean canHandleContent (in string aContentType, in boolean aIsContentPreferred, out string aDesiredContentType); */
    pub canHandleContent: unsafe extern "C" fn (this: *const nsIURIContentListener, aContentType: *const libc::c_char, aIsContentPreferred: bool, aDesiredContentType: *mut *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* attribute nsISupports loadCookie; */
    pub get_loadCookie: unsafe extern "C" fn (this: *const nsIURIContentListener, aLoadCookie: *mut *const nsISupports) -> nsresult,
    pub set_loadCookie: unsafe extern "C" fn (this: *const nsIURIContentListener, aLoadCookie: *const nsISupports) -> nsresult,

    /* attribute nsIURIContentListener parentContentListener; */
    pub get_parentContentListener: unsafe extern "C" fn (this: *const nsIURIContentListener, aParentContentListener: *mut *const nsIURIContentListener) -> nsresult,
    pub set_parentContentListener: unsafe extern "C" fn (this: *const nsIURIContentListener, aParentContentListener: *const nsIURIContentListener) -> nsresult,

}


impl nsIURIContentListener {
    /* boolean onStartURIOpen (in nsIURI aURI); */
    #[inline]
    pub unsafe fn onStartURIOpen(&self, aURI: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).onStartURIOpen)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean doContent (in ACString aContentType, in boolean aIsContentPreferred, in nsIRequest aRequest, out nsIStreamListener aContentHandler); */
    #[inline]
    pub unsafe fn doContent(&self, aContentType: &[u8], aIsContentPreferred: bool, aRequest: Option<&nsIRequest>) -> Result<(Option<RefPtr<nsIStreamListener>>, bool), nsresult> {
        let aContentType = nsCString::from(aContentType);
        let mut aContentHandler = GetterAddrefs::new();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).doContent)(self as *const _, &*aContentType, aIsContentPreferred, aRequest.map_or(::std::ptr::null(), |x| x as *const _), aContentHandler.ptr(), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aContentHandler.refptr(), _retval))
    }

    /* boolean isPreferred (in string aContentType, out string aDesiredContentType); */
    #[inline]
    pub unsafe fn isPreferred(&self, aContentType: *const libc::c_char) -> Result<(*const libc::c_char, bool), nsresult> {
        let mut aDesiredContentType: *const libc::c_char = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isPreferred)(self as *const _, aContentType, &mut aDesiredContentType as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aDesiredContentType, _retval))
    }

    /* boolean canHandleContent (in string aContentType, in boolean aIsContentPreferred, out string aDesiredContentType); */
    #[inline]
    pub unsafe fn canHandleContent(&self, aContentType: *const libc::c_char, aIsContentPreferred: bool) -> Result<(*const libc::c_char, bool), nsresult> {
        let mut aDesiredContentType: *const libc::c_char = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canHandleContent)(self as *const _, aContentType, aIsContentPreferred, &mut aDesiredContentType as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aDesiredContentType, _retval))
    }

    /* attribute nsISupports loadCookie; */
    #[inline]
    pub unsafe fn get_loadCookie(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_loadCookie)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_loadCookie(&self, aLoadCookie: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).set_loadCookie)(self as *const _, aLoadCookie.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIURIContentListener parentContentListener; */
    #[inline]
    pub unsafe fn get_parentContentListener(&self, ) -> Result<Option<RefPtr<nsIURIContentListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parentContentListener)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_parentContentListener(&self, aParentContentListener: Option<&nsIURIContentListener>) -> Result<(), nsresult> {

        match ((*self.vtable).set_parentContentListener)(self as *const _, aParentContentListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


