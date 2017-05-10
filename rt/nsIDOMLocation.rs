//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMLocation.idl
//


#[repr(C)]
pub struct nsIDOMLocation {
    vtable: *const nsIDOMLocationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMLocation {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x79de76e5, 0x994e, 0x4f6b,
            [0x81, 0xaa, 0x42, 0xd9, 0xad, 0xb6, 0xe6, 0x7e])
    }
}

unsafe impl RefCounted for nsIDOMLocation {
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
pub trait nsIDOMLocationCoerce {
    fn coerce_from(v: &nsIDOMLocation) -> &Self;
}

impl nsIDOMLocationCoerce for nsIDOMLocation {
    #[inline]
    fn coerce_from(v: &nsIDOMLocation) -> &Self {
        v
    }
}

impl nsIDOMLocation {
    #[inline]
    pub fn coerce<T: nsIDOMLocationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMLocation {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMLocationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMLocation) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMLocationVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString hash; */
    pub get_hash: unsafe extern "C" fn (this: *const nsIDOMLocation, aHash: *mut nsAString) -> nsresult,
    pub set_hash: unsafe extern "C" fn (this: *const nsIDOMLocation, aHash: *const nsAString) -> nsresult,

    /* attribute DOMString host; */
    pub get_host: unsafe extern "C" fn (this: *const nsIDOMLocation, aHost: *mut nsAString) -> nsresult,
    pub set_host: unsafe extern "C" fn (this: *const nsIDOMLocation, aHost: *const nsAString) -> nsresult,

    /* attribute DOMString hostname; */
    pub get_hostname: unsafe extern "C" fn (this: *const nsIDOMLocation, aHostname: *mut nsAString) -> nsresult,
    pub set_hostname: unsafe extern "C" fn (this: *const nsIDOMLocation, aHostname: *const nsAString) -> nsresult,

    /* attribute DOMString href; */
    pub get_href: unsafe extern "C" fn (this: *const nsIDOMLocation, aHref: *mut nsAString) -> nsresult,
    pub set_href: unsafe extern "C" fn (this: *const nsIDOMLocation, aHref: *const nsAString) -> nsresult,

    /* attribute DOMString pathname; */
    pub get_pathname: unsafe extern "C" fn (this: *const nsIDOMLocation, aPathname: *mut nsAString) -> nsresult,
    pub set_pathname: unsafe extern "C" fn (this: *const nsIDOMLocation, aPathname: *const nsAString) -> nsresult,

    /* attribute DOMString port; */
    pub get_port: unsafe extern "C" fn (this: *const nsIDOMLocation, aPort: *mut nsAString) -> nsresult,
    pub set_port: unsafe extern "C" fn (this: *const nsIDOMLocation, aPort: *const nsAString) -> nsresult,

    /* attribute DOMString protocol; */
    pub get_protocol: unsafe extern "C" fn (this: *const nsIDOMLocation, aProtocol: *mut nsAString) -> nsresult,
    pub set_protocol: unsafe extern "C" fn (this: *const nsIDOMLocation, aProtocol: *const nsAString) -> nsresult,

    /* attribute DOMString search; */
    pub get_search: unsafe extern "C" fn (this: *const nsIDOMLocation, aSearch: *mut nsAString) -> nsresult,
    pub set_search: unsafe extern "C" fn (this: *const nsIDOMLocation, aSearch: *const nsAString) -> nsresult,

    /* readonly attribute DOMString origin; */
    pub get_origin: unsafe extern "C" fn (this: *const nsIDOMLocation, aOrigin: *mut nsAString) -> nsresult,

    /* void reload ([optional] in boolean forceget); */
    pub reload: unsafe extern "C" fn (this: *const nsIDOMLocation, forceget: bool) -> nsresult,

    /* void replace (in DOMString url); */
    pub replace: unsafe extern "C" fn (this: *const nsIDOMLocation, url: *const nsAString) -> nsresult,

    /* void assign (in DOMString url); */
    pub assign: unsafe extern "C" fn (this: *const nsIDOMLocation, url: *const nsAString) -> nsresult,

    /* DOMString toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsIDOMLocation, _retval: *mut nsAString) -> nsresult,

    /* nsIDOMLocation valueOf (); */
    pub valueOf: unsafe extern "C" fn (this: *const nsIDOMLocation, _retval: *mut *const nsIDOMLocation) -> nsresult,

}


impl nsIDOMLocation {
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

    /* readonly attribute DOMString origin; */
    #[inline]
    pub unsafe fn get_origin(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_origin)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void reload ([optional] in boolean forceget); */
    #[inline]
    pub unsafe fn reload(&self, forceget: bool) -> Result<(), nsresult> {

        match ((*self.vtable).reload)(self as *const _, forceget) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void replace (in DOMString url); */
    #[inline]
    pub unsafe fn replace(&self, url: &[u16]) -> Result<(), nsresult> {
        let url = nsString::from(url);
        match ((*self.vtable).replace)(self as *const _, &*url) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void assign (in DOMString url); */
    #[inline]
    pub unsafe fn assign(&self, url: &[u16]) -> Result<(), nsresult> {
        let url = nsString::from(url);
        match ((*self.vtable).assign)(self as *const _, &*url) {
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

    /* nsIDOMLocation valueOf (); */
    #[inline]
    pub unsafe fn valueOf(&self, ) -> Result<Option<RefPtr<nsIDOMLocation>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).valueOf)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


