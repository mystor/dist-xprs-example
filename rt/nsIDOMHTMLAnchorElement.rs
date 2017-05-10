//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLAnchorElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLAnchorElement {
    vtable: *const nsIDOMHTMLAnchorElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLAnchorElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x339c01c8, 0x2d41, 0x4626,
            [0xb2, 0x31, 0xee, 0xc6, 0x3f, 0x02, 0x41, 0xb6])
    }
}

unsafe impl RefCounted for nsIDOMHTMLAnchorElement {
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
pub trait nsIDOMHTMLAnchorElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLAnchorElement) -> &Self;
}

impl nsIDOMHTMLAnchorElementCoerce for nsIDOMHTMLAnchorElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLAnchorElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLAnchorElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLAnchorElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLAnchorElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLAnchorElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLAnchorElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLAnchorElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString href; */
    pub get_href: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aHref: *mut nsAString) -> nsresult,
    pub set_href: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aHref: *const nsAString) -> nsresult,

    /* attribute DOMString target; */
    pub get_target: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aTarget: *mut nsAString) -> nsresult,
    pub set_target: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aTarget: *const nsAString) -> nsresult,

    /* attribute DOMString ping; */
    pub get_ping: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aPing: *mut nsAString) -> nsresult,
    pub set_ping: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aPing: *const nsAString) -> nsresult,

    /* attribute DOMString download; */
    pub get_download: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aDownload: *mut nsAString) -> nsresult,
    pub set_download: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aDownload: *const nsAString) -> nsresult,

    /* attribute DOMString rel; */
    pub get_rel: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aRel: *mut nsAString) -> nsresult,
    pub set_rel: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aRel: *const nsAString) -> nsresult,

    /* attribute DOMString hreflang; */
    pub get_hreflang: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aHreflang: *mut nsAString) -> nsresult,
    pub set_hreflang: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aHreflang: *const nsAString) -> nsresult,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aType: *const nsAString) -> nsresult,

    /* [Null(Stringify)] attribute DOMString text; */
    pub get_text: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aText: *mut nsAString) -> nsresult,
    pub set_text: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aText: *const nsAString) -> nsresult,

    /* attribute DOMString protocol; */
    pub get_protocol: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aProtocol: *mut nsAString) -> nsresult,
    pub set_protocol: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aProtocol: *const nsAString) -> nsresult,

    /* attribute DOMString host; */
    pub get_host: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aHost: *mut nsAString) -> nsresult,
    pub set_host: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aHost: *const nsAString) -> nsresult,

    /* attribute DOMString hostname; */
    pub get_hostname: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aHostname: *mut nsAString) -> nsresult,
    pub set_hostname: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aHostname: *const nsAString) -> nsresult,

    /* attribute DOMString port; */
    pub get_port: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aPort: *mut nsAString) -> nsresult,
    pub set_port: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aPort: *const nsAString) -> nsresult,

    /* attribute DOMString pathname; */
    pub get_pathname: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aPathname: *mut nsAString) -> nsresult,
    pub set_pathname: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aPathname: *const nsAString) -> nsresult,

    /* attribute DOMString search; */
    pub get_search: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aSearch: *mut nsAString) -> nsresult,
    pub set_search: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aSearch: *const nsAString) -> nsresult,

    /* attribute DOMString hash; */
    pub get_hash: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aHash: *mut nsAString) -> nsresult,
    pub set_hash: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aHash: *const nsAString) -> nsresult,

    /* attribute DOMString charset; */
    pub get_charset: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aCharset: *mut nsAString) -> nsresult,
    pub set_charset: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aCharset: *const nsAString) -> nsresult,

    /* attribute DOMString coords; */
    pub get_coords: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aCoords: *mut nsAString) -> nsresult,
    pub set_coords: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aCoords: *const nsAString) -> nsresult,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aName: *const nsAString) -> nsresult,

    /* attribute DOMString rev; */
    pub get_rev: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aRev: *mut nsAString) -> nsresult,
    pub set_rev: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aRev: *const nsAString) -> nsresult,

    /* attribute DOMString shape; */
    pub get_shape: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aShape: *mut nsAString) -> nsresult,
    pub set_shape: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, aShape: *const nsAString) -> nsresult,

    /* DOMString toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsIDOMHTMLAnchorElement, _retval: *mut nsAString) -> nsresult,

}


impl nsIDOMHTMLAnchorElement {
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

    /* attribute DOMString rel; */
    #[inline]
    pub unsafe fn get_rel(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_rel)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_rel(&self, aRel: &[u16]) -> Result<(), nsresult> {
        let aRel = nsString::from(aRel);
        match ((*self.vtable).set_rel)(self as *const _, &*aRel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString hreflang; */
    #[inline]
    pub unsafe fn get_hreflang(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_hreflang)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_hreflang(&self, aHreflang: &[u16]) -> Result<(), nsresult> {
        let aHreflang = nsString::from(aHreflang);
        match ((*self.vtable).set_hreflang)(self as *const _, &*aHreflang) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_type_(&self, aType: &[u16]) -> Result<(), nsresult> {
        let aType = nsString::from(aType);
        match ((*self.vtable).set_type_)(self as *const _, &*aType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [Null(Stringify)] attribute DOMString text; */
    #[inline]
    pub unsafe fn get_text(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_text)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_text(&self, aText: &[u16]) -> Result<(), nsresult> {
        let aText = nsString::from(aText);
        match ((*self.vtable).set_text)(self as *const _, &*aText) {
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

    /* attribute DOMString charset; */
    #[inline]
    pub unsafe fn get_charset(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_charset)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_charset(&self, aCharset: &[u16]) -> Result<(), nsresult> {
        let aCharset = nsString::from(aCharset);
        match ((*self.vtable).set_charset)(self as *const _, &*aCharset) {
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

    /* attribute DOMString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_name(&self, aName: &[u16]) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).set_name)(self as *const _, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString rev; */
    #[inline]
    pub unsafe fn get_rev(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_rev)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_rev(&self, aRev: &[u16]) -> Result<(), nsresult> {
        let aRev = nsString::from(aRev);
        match ((*self.vtable).set_rev)(self as *const _, &*aRev) {
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


