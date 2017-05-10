//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserStream.idl
//


#[repr(C)]
pub struct nsIWebBrowserStream {
    vtable: *const nsIWebBrowserStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x86d02f0e, 0x219b, 0x4cfc,
            [0x9c, 0x88, 0xbd, 0x98, 0xd2, 0xcc, 0xe0, 0xb8])
    }
}

unsafe impl RefCounted for nsIWebBrowserStream {
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
pub trait nsIWebBrowserStreamCoerce {
    fn coerce_from(v: &nsIWebBrowserStream) -> &Self;
}

impl nsIWebBrowserStreamCoerce for nsIWebBrowserStream {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserStream) -> &Self {
        v
    }
}

impl nsIWebBrowserStream {
    #[inline]
    pub fn coerce<T: nsIWebBrowserStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserStream {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebBrowserStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserStreamVTable {
    pub __base: nsISupportsVTable,

    /* void openStream (in nsIURI aBaseURI, in ACString aContentType); */
    pub openStream: unsafe extern "C" fn (this: *const nsIWebBrowserStream, aBaseURI: *const nsIURI, aContentType: *const nsACString) -> nsresult,

    /* void appendToStream ([array, size_is (aLen), const] in octet aData, in unsigned long aLen); */
    /// Unable to call function as its signature contains a non-rust type
    pub appendToStream: *const ::libc::c_void,

    /* void closeStream (); */
    pub closeStream: unsafe extern "C" fn (this: *const nsIWebBrowserStream) -> nsresult,

}


impl nsIWebBrowserStream {
    /* void openStream (in nsIURI aBaseURI, in ACString aContentType); */
    #[inline]
    pub unsafe fn openStream(&self, aBaseURI: Option<&nsIURI>, aContentType: &[u8]) -> Result<(), nsresult> {
        let aContentType = nsCString::from(aContentType);
        match ((*self.vtable).openStream)(self as *const _, aBaseURI.map_or(::std::ptr::null(), |x| x as *const _), &*aContentType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void appendToStream ([array, size_is (aLen), const] in octet aData, in unsigned long aLen); */


    /* void closeStream (); */
    #[inline]
    pub unsafe fn closeStream(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).closeStream)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


