//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLAreaElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLAreaElement {
    vtable: *const nsIDOMHTMLAreaElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLAreaElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x40c78026, 0x36dc, 0x40ca,
            [0x92, 0x21, 0xde, 0x73, 0x26, 0x7e, 0x9e, 0x99])
    }
}

unsafe impl RefCounted for nsIDOMHTMLAreaElement {
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
pub trait nsIDOMHTMLAreaElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLAreaElement) -> &Self;
}

impl nsIDOMHTMLAreaElementCoerce for nsIDOMHTMLAreaElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLAreaElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLAreaElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLAreaElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLAreaElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLAreaElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLAreaElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLAreaElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString alt; */
    pub get_alt: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aAlt: *mut nsAString) -> nsresult,
    pub set_alt: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aAlt: *const nsAString) -> nsresult,

    /* attribute DOMString coords; */
    pub get_coords: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aCoords: *mut nsAString) -> nsresult,
    pub set_coords: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aCoords: *const nsAString) -> nsresult,

    /* attribute DOMString shape; */
    pub get_shape: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aShape: *mut nsAString) -> nsresult,
    pub set_shape: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aShape: *const nsAString) -> nsresult,

    /* attribute DOMString href; */
    pub get_href: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aHref: *mut nsAString) -> nsresult,
    pub set_href: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aHref: *const nsAString) -> nsresult,

    /* attribute DOMString target; */
    pub get_target: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aTarget: *mut nsAString) -> nsresult,
    pub set_target: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aTarget: *const nsAString) -> nsresult,

    /* attribute DOMString ping; */
    pub get_ping: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aPing: *mut nsAString) -> nsresult,
    pub set_ping: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aPing: *const nsAString) -> nsresult,

    /* attribute DOMString download; */
    pub get_download: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aDownload: *mut nsAString) -> nsresult,
    pub set_download: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aDownload: *const nsAString) -> nsresult,

    /* attribute DOMString protocol; */
    pub get_protocol: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aProtocol: *mut nsAString) -> nsresult,
    pub set_protocol: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aProtocol: *const nsAString) -> nsresult,

    /* attribute DOMString host; */
    pub get_host: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aHost: *mut nsAString) -> nsresult,
    pub set_host: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aHost: *const nsAString) -> nsresult,

    /* attribute DOMString hostname; */
    pub get_hostname: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aHostname: *mut nsAString) -> nsresult,
    pub set_hostname: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aHostname: *const nsAString) -> nsresult,

    /* attribute DOMString port; */
    pub get_port: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aPort: *mut nsAString) -> nsresult,
    pub set_port: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aPort: *const nsAString) -> nsresult,

    /* attribute DOMString pathname; */
    pub get_pathname: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aPathname: *mut nsAString) -> nsresult,
    pub set_pathname: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aPathname: *const nsAString) -> nsresult,

    /* attribute DOMString search; */
    pub get_search: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aSearch: *mut nsAString) -> nsresult,
    pub set_search: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aSearch: *const nsAString) -> nsresult,

    /* attribute DOMString hash; */
    pub get_hash: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aHash: *mut nsAString) -> nsresult,
    pub set_hash: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aHash: *const nsAString) -> nsresult,

    /* attribute boolean noHref; */
    pub get_noHref: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aNoHref: *mut bool) -> nsresult,
    pub set_noHref: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, aNoHref: bool) -> nsresult,

    /* DOMString toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsIDOMHTMLAreaElement, _retval: *mut nsAString) -> nsresult,

}


impl nsIDOMHTMLAreaElement {
    /* attribute DOMString alt; */
    #[inline]
    pub unsafe fn get_alt(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_alt)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_alt(&self, aAlt: &[u16]) -> Result<(), nsresult> {
        let aAlt = nsString::from(aAlt);
        match ((*self.vtable).set_alt)(self as *const _, &*aAlt) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString coords; */
    #[inline]
    pub unsafe fn get_coords(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_coords)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_coords(&self, aCoords: &[u16]) -> Result<(), nsresult> {
        let aCoords = nsString::from(aCoords);
        match ((*self.vtable).set_coords)(self as *const _, &*aCoords) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString shape; */
    #[inline]
    pub unsafe fn get_shape(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_shape)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_shape(&self, aShape: &[u16]) -> Result<(), nsresult> {
        let aShape = nsString::from(aShape);
        match ((*self.vtable).set_shape)(self as *const _, &*aShape) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString href; */
    #[inline]
    pub unsafe fn get_href(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_href)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_href(&self, aHref: &[u16]) -> Result<(), nsresult> {
        let aHref = nsString::from(aHref);
        match ((*self.vtable).set_href)(self as *const _, &*aHref) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString target; */
    #[inline]
    pub unsafe fn get_target(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_target)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_target(&self, aTarget: &[u16]) -> Result<(), nsresult> {
        let aTarget = nsString::from(aTarget);
        match ((*self.vtable).set_target)(self as *const _, &*aTarget) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString ping; */
    #[inline]
    pub unsafe fn get_ping(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_ping)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_ping(&self, aPing: &[u16]) -> Result<(), nsresult> {
        let aPing = nsString::from(aPing);
        match ((*self.vtable).set_ping)(self as *const _, &*aPing) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString download; */
    #[inline]
    pub unsafe fn get_download(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_download)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_download(&self, aDownload: &[u16]) -> Result<(), nsresult> {
        let aDownload = nsString::from(aDownload);
        match ((*self.vtable).set_download)(self as *const _, &*aDownload) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString protocol; */
    #[inline]
    pub unsafe fn get_protocol(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_protocol)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_protocol(&self, aProtocol: &[u16]) -> Result<(), nsresult> {
        let aProtocol = nsString::from(aProtocol);
        match ((*self.vtable).set_protocol)(self as *const _, &*aProtocol) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString host; */
    #[inline]
    pub unsafe fn get_host(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_host)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_host(&self, aHost: &[u16]) -> Result<(), nsresult> {
        let aHost = nsString::from(aHost);
        match ((*self.vtable).set_host)(self as *const _, &*aHost) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString hostname; */
    #[inline]
    pub unsafe fn get_hostname(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_hostname)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_hostname(&self, aHostname: &[u16]) -> Result<(), nsresult> {
        let aHostname = nsString::from(aHostname);
        match ((*self.vtable).set_hostname)(self as *const _, &*aHostname) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString port; */
    #[inline]
    pub unsafe fn get_port(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_port)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_port(&self, aPort: &[u16]) -> Result<(), nsresult> {
        let aPort = nsString::from(aPort);
        match ((*self.vtable).set_port)(self as *const _, &*aPort) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString pathname; */
    #[inline]
    pub unsafe fn get_pathname(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_pathname)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_pathname(&self, aPathname: &[u16]) -> Result<(), nsresult> {
        let aPathname = nsString::from(aPathname);
        match ((*self.vtable).set_pathname)(self as *const _, &*aPathname) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString search; */
    #[inline]
    pub unsafe fn get_search(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_search)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_search(&self, aSearch: &[u16]) -> Result<(), nsresult> {
        let aSearch = nsString::from(aSearch);
        match ((*self.vtable).set_search)(self as *const _, &*aSearch) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString hash; */
    #[inline]
    pub unsafe fn get_hash(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_hash)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_hash(&self, aHash: &[u16]) -> Result<(), nsresult> {
        let aHash = nsString::from(aHash);
        match ((*self.vtable).set_hash)(self as *const _, &*aHash) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean noHref; */
    #[inline]
    pub unsafe fn get_noHref(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_noHref)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_noHref(&self, aNoHref: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_noHref)(self as *const _, aNoHref) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* DOMString toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).toString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


