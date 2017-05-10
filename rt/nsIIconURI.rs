//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIconURI.idl
//


#[repr(C)]
pub struct nsIMozIconURI {
    vtable: *const nsIMozIconURIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMozIconURI {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf8fe5ef2, 0x5f2b, 0x43f3,
            [0x85, 0x7d, 0x5b, 0x64, 0xd1, 0x92, 0xc4, 0x27])
    }
}

unsafe impl RefCounted for nsIMozIconURI {
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
pub trait nsIMozIconURICoerce {
    fn coerce_from(v: &nsIMozIconURI) -> &Self;
}

impl nsIMozIconURICoerce for nsIMozIconURI {
    #[inline]
    fn coerce_from(v: &nsIMozIconURI) -> &Self {
        v
    }
}

impl nsIMozIconURI {
    #[inline]
    pub fn coerce<T: nsIMozIconURICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMozIconURI {
    type Target = nsIURI;
    #[inline]
    fn deref(&self) -> &nsIURI {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIURICoerce> nsIMozIconURICoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMozIconURI) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMozIconURIVTable {
    pub __base: nsIURIVTable,

    /* attribute nsIURL iconURL; */
    pub get_iconURL: unsafe extern "C" fn (this: *const nsIMozIconURI, aIconURL: *mut *const nsIURL) -> nsresult,
    pub set_iconURL: unsafe extern "C" fn (this: *const nsIMozIconURI, aIconURL: *const nsIURL) -> nsresult,

    /* attribute unsigned long imageSize; */
    pub get_imageSize: unsafe extern "C" fn (this: *const nsIMozIconURI, aImageSize: *mut libc::uint32_t) -> nsresult,
    pub set_imageSize: unsafe extern "C" fn (this: *const nsIMozIconURI, aImageSize: libc::uint32_t) -> nsresult,

    /* readonly attribute ACString stockIcon; */
    pub get_stockIcon: unsafe extern "C" fn (this: *const nsIMozIconURI, aStockIcon: *mut nsACString) -> nsresult,

    /* readonly attribute ACString iconSize; */
    pub get_iconSize: unsafe extern "C" fn (this: *const nsIMozIconURI, aIconSize: *mut nsACString) -> nsresult,

    /* readonly attribute ACString iconState; */
    pub get_iconState: unsafe extern "C" fn (this: *const nsIMozIconURI, aIconState: *mut nsACString) -> nsresult,

    /* attribute ACString contentType; */
    pub get_contentType: unsafe extern "C" fn (this: *const nsIMozIconURI, aContentType: *mut nsACString) -> nsresult,
    pub set_contentType: unsafe extern "C" fn (this: *const nsIMozIconURI, aContentType: *const nsACString) -> nsresult,

    /* readonly attribute ACString fileExtension; */
    pub get_fileExtension: unsafe extern "C" fn (this: *const nsIMozIconURI, aFileExtension: *mut nsACString) -> nsresult,

}


impl nsIMozIconURI {
    /* attribute nsIURL iconURL; */
    #[inline]
    pub unsafe fn get_iconURL(&self, ) -> Result<Option<RefPtr<nsIURL>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_iconURL)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_iconURL(&self, aIconURL: Option<&nsIURL>) -> Result<(), nsresult> {

        match ((*self.vtable).set_iconURL)(self as *const _, aIconURL.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long imageSize; */
    #[inline]
    pub unsafe fn get_imageSize(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_imageSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_imageSize(&self, aImageSize: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_imageSize)(self as *const _, aImageSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute ACString stockIcon; */
    #[inline]
    pub unsafe fn get_stockIcon(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_stockIcon)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString iconSize; */
    #[inline]
    pub unsafe fn get_iconSize(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_iconSize)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString iconState; */
    #[inline]
    pub unsafe fn get_iconState(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_iconState)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute ACString contentType; */
    #[inline]
    pub unsafe fn get_contentType(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_contentType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_contentType(&self, aContentType: &[u8]) -> Result<(), nsresult> {
        let aContentType = nsCString::from(aContentType);
        match ((*self.vtable).set_contentType)(self as *const _, &*aContentType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute ACString fileExtension; */
    #[inline]
    pub unsafe fn get_fileExtension(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_fileExtension)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


