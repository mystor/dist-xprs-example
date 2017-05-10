//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURI.idl
//


#[repr(C)]
pub struct nsIURI {
    vtable: *const nsIURIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIURI {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x92073a54, 0x6d78, 0x4f30,
            [0x91, 0x3a, 0xb8, 0x71, 0x81, 0x32, 0x08, 0xc6])
    }
}

unsafe impl RefCounted for nsIURI {
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
pub trait nsIURICoerce {
    fn coerce_from(v: &nsIURI) -> &Self;
}

impl nsIURICoerce for nsIURI {
    #[inline]
    fn coerce_from(v: &nsIURI) -> &Self {
        v
    }
}

impl nsIURI {
    #[inline]
    pub fn coerce<T: nsIURICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIURI {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIURICoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURI) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIURIVTable {
    pub __base: nsISupportsVTable,

    /* attribute AUTF8String spec; */
    pub get_spec: unsafe extern "C" fn (this: *const nsIURI, aSpec: *mut nsACString) -> nsresult,
    pub set_spec: unsafe extern "C" fn (this: *const nsIURI, aSpec: *const nsACString) -> nsresult,

    /* readonly attribute AUTF8String prePath; */
    pub get_prePath: unsafe extern "C" fn (this: *const nsIURI, aPrePath: *mut nsACString) -> nsresult,

    /* attribute ACString scheme; */
    pub get_scheme: unsafe extern "C" fn (this: *const nsIURI, aScheme: *mut nsACString) -> nsresult,
    pub set_scheme: unsafe extern "C" fn (this: *const nsIURI, aScheme: *const nsACString) -> nsresult,

    /* attribute AUTF8String userPass; */
    pub get_userPass: unsafe extern "C" fn (this: *const nsIURI, aUserPass: *mut nsACString) -> nsresult,
    pub set_userPass: unsafe extern "C" fn (this: *const nsIURI, aUserPass: *const nsACString) -> nsresult,

    /* attribute AUTF8String username; */
    pub get_username: unsafe extern "C" fn (this: *const nsIURI, aUsername: *mut nsACString) -> nsresult,
    pub set_username: unsafe extern "C" fn (this: *const nsIURI, aUsername: *const nsACString) -> nsresult,

    /* attribute AUTF8String password; */
    pub get_password: unsafe extern "C" fn (this: *const nsIURI, aPassword: *mut nsACString) -> nsresult,
    pub set_password: unsafe extern "C" fn (this: *const nsIURI, aPassword: *const nsACString) -> nsresult,

    /* attribute AUTF8String hostPort; */
    pub get_hostPort: unsafe extern "C" fn (this: *const nsIURI, aHostPort: *mut nsACString) -> nsresult,
    pub set_hostPort: unsafe extern "C" fn (this: *const nsIURI, aHostPort: *const nsACString) -> nsresult,

    /* void setHostAndPort (in AUTF8String hostport); */
    pub setHostAndPort: unsafe extern "C" fn (this: *const nsIURI, hostport: *const nsACString) -> nsresult,

    /* attribute AUTF8String host; */
    pub get_host: unsafe extern "C" fn (this: *const nsIURI, aHost: *mut nsACString) -> nsresult,
    pub set_host: unsafe extern "C" fn (this: *const nsIURI, aHost: *const nsACString) -> nsresult,

    /* attribute long port; */
    pub get_port: unsafe extern "C" fn (this: *const nsIURI, aPort: *mut libc::int32_t) -> nsresult,
    pub set_port: unsafe extern "C" fn (this: *const nsIURI, aPort: libc::int32_t) -> nsresult,

    /* attribute AUTF8String path; */
    pub get_path: unsafe extern "C" fn (this: *const nsIURI, aPath: *mut nsACString) -> nsresult,
    pub set_path: unsafe extern "C" fn (this: *const nsIURI, aPath: *const nsACString) -> nsresult,

    /* boolean equals (in nsIURI other); */
    pub equals: unsafe extern "C" fn (this: *const nsIURI, other: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* boolean schemeIs (in string scheme); */
    pub schemeIs: unsafe extern "C" fn (this: *const nsIURI, scheme: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* nsIURI clone (); */
    pub clone: unsafe extern "C" fn (this: *const nsIURI, _retval: *mut *const nsIURI) -> nsresult,

    /* AUTF8String resolve (in AUTF8String relativePath); */
    pub resolve: unsafe extern "C" fn (this: *const nsIURI, relativePath: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* readonly attribute ACString asciiSpec; */
    pub get_asciiSpec: unsafe extern "C" fn (this: *const nsIURI, aAsciiSpec: *mut nsACString) -> nsresult,

    /* readonly attribute ACString asciiHostPort; */
    pub get_asciiHostPort: unsafe extern "C" fn (this: *const nsIURI, aAsciiHostPort: *mut nsACString) -> nsresult,

    /* readonly attribute ACString asciiHost; */
    pub get_asciiHost: unsafe extern "C" fn (this: *const nsIURI, aAsciiHost: *mut nsACString) -> nsresult,

    /* readonly attribute ACString originCharset; */
    pub get_originCharset: unsafe extern "C" fn (this: *const nsIURI, aOriginCharset: *mut nsACString) -> nsresult,

    /* attribute AUTF8String ref; */
    pub get_ref_: unsafe extern "C" fn (this: *const nsIURI, aRef: *mut nsACString) -> nsresult,
    pub set_ref_: unsafe extern "C" fn (this: *const nsIURI, aRef: *const nsACString) -> nsresult,

    /* boolean equalsExceptRef (in nsIURI other); */
    pub equalsExceptRef: unsafe extern "C" fn (this: *const nsIURI, other: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* nsIURI cloneIgnoringRef (); */
    pub cloneIgnoringRef: unsafe extern "C" fn (this: *const nsIURI, _retval: *mut *const nsIURI) -> nsresult,

    /* nsIURI cloneWithNewRef (in AUTF8String newRef); */
    pub cloneWithNewRef: unsafe extern "C" fn (this: *const nsIURI, newRef: *const nsACString, _retval: *mut *const nsIURI) -> nsresult,

    /* readonly attribute AUTF8String specIgnoringRef; */
    pub get_specIgnoringRef: unsafe extern "C" fn (this: *const nsIURI, aSpecIgnoringRef: *mut nsACString) -> nsresult,

    /* readonly attribute boolean hasRef; */
    pub get_hasRef: unsafe extern "C" fn (this: *const nsIURI, aHasRef: *mut bool) -> nsresult,

    /* attribute AUTF8String filePath; */
    pub get_filePath: unsafe extern "C" fn (this: *const nsIURI, aFilePath: *mut nsACString) -> nsresult,
    pub set_filePath: unsafe extern "C" fn (this: *const nsIURI, aFilePath: *const nsACString) -> nsresult,

    /* attribute AUTF8String query; */
    pub get_query: unsafe extern "C" fn (this: *const nsIURI, aQuery: *mut nsACString) -> nsresult,
    pub set_query: unsafe extern "C" fn (this: *const nsIURI, aQuery: *const nsACString) -> nsresult,

}


impl nsIURI {
    /* attribute AUTF8String spec; */
    #[inline]
    pub unsafe fn get_spec(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_spec)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_spec(&self, aSpec: &[u8]) -> Result<(), nsresult> {
        let aSpec = nsCString::from(aSpec);
        match ((*self.vtable).set_spec)(self as *const _, &*aSpec) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute AUTF8String prePath; */
    #[inline]
    pub unsafe fn get_prePath(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_prePath)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute ACString scheme; */
    #[inline]
    pub unsafe fn get_scheme(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_scheme)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_scheme(&self, aScheme: &[u8]) -> Result<(), nsresult> {
        let aScheme = nsCString::from(aScheme);
        match ((*self.vtable).set_scheme)(self as *const _, &*aScheme) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String userPass; */
    #[inline]
    pub unsafe fn get_userPass(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_userPass)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_userPass(&self, aUserPass: &[u8]) -> Result<(), nsresult> {
        let aUserPass = nsCString::from(aUserPass);
        match ((*self.vtable).set_userPass)(self as *const _, &*aUserPass) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String username; */
    #[inline]
    pub unsafe fn get_username(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_username)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_username(&self, aUsername: &[u8]) -> Result<(), nsresult> {
        let aUsername = nsCString::from(aUsername);
        match ((*self.vtable).set_username)(self as *const _, &*aUsername) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String password; */
    #[inline]
    pub unsafe fn get_password(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_password)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_password(&self, aPassword: &[u8]) -> Result<(), nsresult> {
        let aPassword = nsCString::from(aPassword);
        match ((*self.vtable).set_password)(self as *const _, &*aPassword) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String hostPort; */
    #[inline]
    pub unsafe fn get_hostPort(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_hostPort)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_hostPort(&self, aHostPort: &[u8]) -> Result<(), nsresult> {
        let aHostPort = nsCString::from(aHostPort);
        match ((*self.vtable).set_hostPort)(self as *const _, &*aHostPort) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setHostAndPort (in AUTF8String hostport); */
    #[inline]
    pub unsafe fn setHostAndPort(&self, hostport: &[u8]) -> Result<(), nsresult> {
        let hostport = nsCString::from(hostport);
        match ((*self.vtable).setHostAndPort)(self as *const _, &*hostport) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String host; */
    #[inline]
    pub unsafe fn get_host(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_host)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_host(&self, aHost: &[u8]) -> Result<(), nsresult> {
        let aHost = nsCString::from(aHost);
        match ((*self.vtable).set_host)(self as *const _, &*aHost) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long port; */
    #[inline]
    pub unsafe fn get_port(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_port)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_port(&self, aPort: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_port)(self as *const _, aPort) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String path; */
    #[inline]
    pub unsafe fn get_path(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_path)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_path(&self, aPath: &[u8]) -> Result<(), nsresult> {
        let aPath = nsCString::from(aPath);
        match ((*self.vtable).set_path)(self as *const _, &*aPath) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean equals (in nsIURI other); */
    #[inline]
    pub unsafe fn equals(&self, other: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).equals)(self as *const _, other.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean schemeIs (in string scheme); */
    #[inline]
    pub unsafe fn schemeIs(&self, scheme: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).schemeIs)(self as *const _, scheme, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIURI clone (); */
    #[inline]
    pub unsafe fn clone(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clone)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* AUTF8String resolve (in AUTF8String relativePath); */
    #[inline]
    pub unsafe fn resolve(&self, relativePath: &[u8]) -> Result<nsCString, nsresult> {
        let relativePath = nsCString::from(relativePath);
        let mut _retval = nsCString::new();
        match ((*self.vtable).resolve)(self as *const _, &*relativePath, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString asciiSpec; */
    #[inline]
    pub unsafe fn get_asciiSpec(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_asciiSpec)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString asciiHostPort; */
    #[inline]
    pub unsafe fn get_asciiHostPort(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_asciiHostPort)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString asciiHost; */
    #[inline]
    pub unsafe fn get_asciiHost(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_asciiHost)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString originCharset; */
    #[inline]
    pub unsafe fn get_originCharset(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_originCharset)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute AUTF8String ref; */
    #[inline]
    pub unsafe fn get_ref_(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_ref_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_ref_(&self, aRef: &[u8]) -> Result<(), nsresult> {
        let aRef = nsCString::from(aRef);
        match ((*self.vtable).set_ref_)(self as *const _, &*aRef) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean equalsExceptRef (in nsIURI other); */
    #[inline]
    pub unsafe fn equalsExceptRef(&self, other: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).equalsExceptRef)(self as *const _, other.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIURI cloneIgnoringRef (); */
    #[inline]
    pub unsafe fn cloneIgnoringRef(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).cloneIgnoringRef)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIURI cloneWithNewRef (in AUTF8String newRef); */
    #[inline]
    pub unsafe fn cloneWithNewRef(&self, newRef: &[u8]) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let newRef = nsCString::from(newRef);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).cloneWithNewRef)(self as *const _, &*newRef, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AUTF8String specIgnoringRef; */
    #[inline]
    pub unsafe fn get_specIgnoringRef(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_specIgnoringRef)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean hasRef; */
    #[inline]
    pub unsafe fn get_hasRef(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasRef)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute AUTF8String filePath; */
    #[inline]
    pub unsafe fn get_filePath(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_filePath)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_filePath(&self, aFilePath: &[u8]) -> Result<(), nsresult> {
        let aFilePath = nsCString::from(aFilePath);
        match ((*self.vtable).set_filePath)(self as *const _, &*aFilePath) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String query; */
    #[inline]
    pub unsafe fn get_query(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_query)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_query(&self, aQuery: &[u8]) -> Result<(), nsresult> {
        let aQuery = nsCString::from(aQuery);
        match ((*self.vtable).set_query)(self as *const _, &*aQuery) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


