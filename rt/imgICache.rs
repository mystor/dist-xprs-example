//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgICache.idl
//


#[repr(C)]
pub struct imgICache {
    vtable: *const imgICacheVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for imgICache {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbfdf23ff, 0x378e, 0x402e,
            [0x8a, 0x6c, 0x84, 0x0f, 0x0c, 0x82, 0xb6, 0xc3])
    }
}

unsafe impl RefCounted for imgICache {
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
pub trait imgICacheCoerce {
    fn coerce_from(v: &imgICache) -> &Self;
}

impl imgICacheCoerce for imgICache {
    #[inline]
    fn coerce_from(v: &imgICache) -> &Self {
        v
    }
}

impl imgICache {
    #[inline]
    pub fn coerce<T: imgICacheCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for imgICache {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> imgICacheCoerce for T {
    #[inline]
    fn coerce_from(v: &imgICache) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct imgICacheVTable {
    pub __base: nsISupportsVTable,

    /* void clearCache (in boolean chrome); */
    pub clearCache: unsafe extern "C" fn (this: *const imgICache, chrome: bool) -> nsresult,

    /* [noscript] void removeEntry (in nsIURI uri, [optional] in nsIDOMDocument doc); */
    pub removeEntry: unsafe extern "C" fn (this: *const imgICache, uri: *const nsIURI, doc: *const nsIDOMDocument) -> nsresult,

    /* [must_use] nsIProperties findEntryProperties (in nsIURI uri, [optional] in nsIDOMDocument doc); */
    pub findEntryProperties: unsafe extern "C" fn (this: *const imgICache, uri: *const nsIURI, doc: *const nsIDOMDocument, _retval: *mut *const nsIProperties) -> nsresult,

    /* void respectPrivacyNotifications (); */
    pub respectPrivacyNotifications: unsafe extern "C" fn (this: *const imgICache) -> nsresult,

    /* [noscript,notxpcom] void clearCacheForControlledDocument (in nsIDocument doc); */
    pub clearCacheForControlledDocument: unsafe extern "C" fn (this: *const imgICache, doc: *const nsIDocument) -> libc::c_void,

}


impl imgICache {
    /* void clearCache (in boolean chrome); */
    #[inline]
    pub unsafe fn clearCache(&self, chrome: bool) -> Result<(), nsresult> {

        match ((*self.vtable).clearCache)(self as *const _, chrome) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void removeEntry (in nsIURI uri, [optional] in nsIDOMDocument doc); */
    #[inline]
    pub unsafe fn removeEntry(&self, uri: Option<&nsIURI>, doc: Option<&nsIDOMDocument>) -> Result<(), nsresult> {

        match ((*self.vtable).removeEntry)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), doc.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] nsIProperties findEntryProperties (in nsIURI uri, [optional] in nsIDOMDocument doc); */
    #[inline]
    pub unsafe fn findEntryProperties(&self, uri: Option<&nsIURI>, doc: Option<&nsIDOMDocument>) -> Result<Option<RefPtr<nsIProperties>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).findEntryProperties)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), doc.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void respectPrivacyNotifications (); */
    #[inline]
    pub unsafe fn respectPrivacyNotifications(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).respectPrivacyNotifications)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,notxpcom] void clearCacheForControlledDocument (in nsIDocument doc); */
    #[inline]
    pub unsafe fn clearCacheForControlledDocument(&self, doc: Option<&nsIDocument>) -> () {

        let _retval = ((*self.vtable).clearCacheForControlledDocument)(self as *const _, doc.map_or(::std::ptr::null(), |x| x as *const _));
        ()
    }

}


