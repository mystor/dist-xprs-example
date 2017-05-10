//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIGeckoMediaPluginChromeService.idl
//


#[repr(C)]
pub struct mozIGeckoMediaPluginChromeService {
    vtable: *const mozIGeckoMediaPluginChromeServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIGeckoMediaPluginChromeService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x32d35d21, 0x181f, 0x4630,
            [0x8c, 0xaa, 0xa4, 0x31, 0xe2, 0xeb, 0xad, 0x72])
    }
}

unsafe impl RefCounted for mozIGeckoMediaPluginChromeService {
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
pub trait mozIGeckoMediaPluginChromeServiceCoerce {
    fn coerce_from(v: &mozIGeckoMediaPluginChromeService) -> &Self;
}

impl mozIGeckoMediaPluginChromeServiceCoerce for mozIGeckoMediaPluginChromeService {
    #[inline]
    fn coerce_from(v: &mozIGeckoMediaPluginChromeService) -> &Self {
        v
    }
}

impl mozIGeckoMediaPluginChromeService {
    #[inline]
    pub fn coerce<T: mozIGeckoMediaPluginChromeServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIGeckoMediaPluginChromeService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIGeckoMediaPluginChromeServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIGeckoMediaPluginChromeService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIGeckoMediaPluginChromeServiceVTable {
    pub __base: nsISupportsVTable,

    /* void addPluginDirectory (in AString directory); */
    pub addPluginDirectory: unsafe extern "C" fn (this: *const mozIGeckoMediaPluginChromeService, directory: *const nsAString) -> nsresult,

    /* void removePluginDirectory (in AString directory); */
    pub removePluginDirectory: unsafe extern "C" fn (this: *const mozIGeckoMediaPluginChromeService, directory: *const nsAString) -> nsresult,

    /* void removeAndDeletePluginDirectory (in AString directory, [optional] in bool defer); */
    pub removeAndDeletePluginDirectory: unsafe extern "C" fn (this: *const mozIGeckoMediaPluginChromeService, directory: *const nsAString, defer: bool) -> nsresult,

    /* void forgetThisSite (in AString site, in DOMString aPattern); */
    pub forgetThisSite: unsafe extern "C" fn (this: *const mozIGeckoMediaPluginChromeService, site: *const nsAString, aPattern: *const nsAString) -> nsresult,

    /* bool isPersistentStorageAllowed (in ACString nodeId); */
    pub isPersistentStorageAllowed: unsafe extern "C" fn (this: *const mozIGeckoMediaPluginChromeService, nodeId: *const nsACString, _retval: *mut bool) -> nsresult,

    /* nsIFile getStorageDir (); */
    pub getStorageDir: unsafe extern "C" fn (this: *const mozIGeckoMediaPluginChromeService, _retval: *mut *const nsIFile) -> nsresult,

}


impl mozIGeckoMediaPluginChromeService {
    /* void addPluginDirectory (in AString directory); */
    #[inline]
    pub unsafe fn addPluginDirectory(&self, directory: &[u16]) -> Result<(), nsresult> {
        let directory = nsString::from(directory);
        match ((*self.vtable).addPluginDirectory)(self as *const _, &*directory) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removePluginDirectory (in AString directory); */
    #[inline]
    pub unsafe fn removePluginDirectory(&self, directory: &[u16]) -> Result<(), nsresult> {
        let directory = nsString::from(directory);
        match ((*self.vtable).removePluginDirectory)(self as *const _, &*directory) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeAndDeletePluginDirectory (in AString directory, [optional] in bool defer); */
    #[inline]
    pub unsafe fn removeAndDeletePluginDirectory(&self, directory: &[u16], defer: bool) -> Result<(), nsresult> {
        let directory = nsString::from(directory);
        match ((*self.vtable).removeAndDeletePluginDirectory)(self as *const _, &*directory, defer) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void forgetThisSite (in AString site, in DOMString aPattern); */
    #[inline]
    pub unsafe fn forgetThisSite(&self, site: &[u16], aPattern: &[u16]) -> Result<(), nsresult> {
        let site = nsString::from(site);
        let aPattern = nsString::from(aPattern);
        match ((*self.vtable).forgetThisSite)(self as *const _, &*site, &*aPattern) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool isPersistentStorageAllowed (in ACString nodeId); */
    #[inline]
    pub unsafe fn isPersistentStorageAllowed(&self, nodeId: &[u8]) -> Result<bool, nsresult> {
        let nodeId = nsCString::from(nodeId);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isPersistentStorageAllowed)(self as *const _, &*nodeId, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIFile getStorageDir (); */
    #[inline]
    pub unsafe fn getStorageDir(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getStorageDir)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


