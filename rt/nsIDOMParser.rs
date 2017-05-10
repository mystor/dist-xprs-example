//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMParser.idl
//


#[repr(C)]
pub struct nsIDOMParser {
    vtable: *const nsIDOMParserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMParser {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x70b9600e, 0x8622, 0x4c93,
            [0x9a, 0xd8, 0x22, 0xc2, 0x80, 0x58, 0xdc, 0x44])
    }
}

unsafe impl RefCounted for nsIDOMParser {
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
pub trait nsIDOMParserCoerce {
    fn coerce_from(v: &nsIDOMParser) -> &Self;
}

impl nsIDOMParserCoerce for nsIDOMParser {
    #[inline]
    fn coerce_from(v: &nsIDOMParser) -> &Self {
        v
    }
}

impl nsIDOMParser {
    #[inline]
    pub fn coerce<T: nsIDOMParserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMParser {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMParserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMParser) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMParserVTable {
    pub __base: nsISupportsVTable,

    /* nsIDOMDocument parseFromString (in wstring str, in string contentType); */
    pub parseFromString: unsafe extern "C" fn (this: *const nsIDOMParser, str: *const libc::int16_t, contentType: *const libc::c_char, _retval: *mut *const nsIDOMDocument) -> nsresult,

    /* nsIDOMDocument parseFromBuffer ([array, size_is (bufLen), const] in octet buf, in uint32_t bufLen, in string contentType); */
    /// Unable to call function as its signature contains a non-rust type
    pub parseFromBuffer: *const ::libc::c_void,

    /* nsIDOMDocument parseFromStream (in nsIInputStream stream, in string charset, in long contentLength, in string contentType); */
    pub parseFromStream: unsafe extern "C" fn (this: *const nsIDOMParser, stream: *const nsIInputStream, charset: *const libc::c_char, contentLength: libc::int32_t, contentType: *const libc::c_char, _retval: *mut *const nsIDOMDocument) -> nsresult,

    /* [noscript] void init (in nsIPrincipal principal, in nsIURI documentURI, in nsIURI baseURI, in nsIGlobalObject scriptObject); */
    pub init: unsafe extern "C" fn (this: *const nsIDOMParser, principal: *const nsIPrincipal, documentURI: *const nsIURI, baseURI: *const nsIURI, scriptObject: *const nsIGlobalObject) -> nsresult,

}


impl nsIDOMParser {
    /* nsIDOMDocument parseFromString (in wstring str, in string contentType); */
    #[inline]
    pub unsafe fn parseFromString(&self, str: *const libc::int16_t, contentType: *const libc::c_char) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).parseFromString)(self as *const _, str, contentType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMDocument parseFromBuffer ([array, size_is (bufLen), const] in octet buf, in uint32_t bufLen, in string contentType); */


    /* nsIDOMDocument parseFromStream (in nsIInputStream stream, in string charset, in long contentLength, in string contentType); */
    #[inline]
    pub unsafe fn parseFromStream(&self, stream: Option<&nsIInputStream>, charset: *const libc::c_char, contentLength: libc::int32_t, contentType: *const libc::c_char) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).parseFromStream)(self as *const _, stream.map_or(::std::ptr::null(), |x| x as *const _), charset, contentLength, contentType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] void init (in nsIPrincipal principal, in nsIURI documentURI, in nsIURI baseURI, in nsIGlobalObject scriptObject); */
    #[inline]
    pub unsafe fn init(&self, principal: Option<&nsIPrincipal>, documentURI: Option<&nsIURI>, baseURI: Option<&nsIURI>, scriptObject: Option<&nsIGlobalObject>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, principal.map_or(::std::ptr::null(), |x| x as *const _), documentURI.map_or(::std::ptr::null(), |x| x as *const _), baseURI.map_or(::std::ptr::null(), |x| x as *const _), scriptObject.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


