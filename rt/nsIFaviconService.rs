//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFaviconService.idl
//


pub mod nsIFaviconService_consts {
    pub const FAVICON_LOAD_PRIVATE: i64 = 1;
    pub const FAVICON_LOAD_NON_PRIVATE: i64 = 2;
    pub const MAX_FAVICON_BUFFER_SIZE: i64 = 65536;
}


#[repr(C)]
pub struct nsIFaviconService {
    vtable: *const nsIFaviconServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFaviconService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe81e0b0c, 0xb9f1, 0x4c2e,
            [0x8f, 0x3c, 0xb8, 0x09, 0x93, 0x3c, 0xf7, 0x3c])
    }
}

unsafe impl RefCounted for nsIFaviconService {
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
pub trait nsIFaviconServiceCoerce {
    fn coerce_from(v: &nsIFaviconService) -> &Self;
}

impl nsIFaviconServiceCoerce for nsIFaviconService {
    #[inline]
    fn coerce_from(v: &nsIFaviconService) -> &Self {
        v
    }
}

impl nsIFaviconService {
    #[inline]
    pub fn coerce<T: nsIFaviconServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFaviconService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFaviconServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFaviconService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFaviconServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIURI getFaviconLinkForIcon (in nsIURI aFaviconURI); */
    pub getFaviconLinkForIcon: unsafe extern "C" fn (this: *const nsIFaviconService, aFaviconURI: *const nsIURI, _retval: *mut *const nsIURI) -> nsresult,

    /* void expireAllFavicons (); */
    pub expireAllFavicons: unsafe extern "C" fn (this: *const nsIFaviconService) -> nsresult,

    /* unsigned short preferredSizeFromURI (in nsIURI aURI); */
    pub preferredSizeFromURI: unsafe extern "C" fn (this: *const nsIFaviconService, aURI: *const nsIURI, _retval: *mut libc::uint16_t) -> nsresult,

    /* void addFailedFavicon (in nsIURI aFaviconURI); */
    pub addFailedFavicon: unsafe extern "C" fn (this: *const nsIFaviconService, aFaviconURI: *const nsIURI) -> nsresult,

    /* void removeFailedFavicon (in nsIURI aFaviconURI); */
    pub removeFailedFavicon: unsafe extern "C" fn (this: *const nsIFaviconService, aFaviconURI: *const nsIURI) -> nsresult,

    /* boolean isFailedFavicon (in nsIURI aFaviconURI); */
    pub isFailedFavicon: unsafe extern "C" fn (this: *const nsIFaviconService, aFaviconURI: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* readonly attribute nsIURI defaultFavicon; */
    pub get_defaultFavicon: unsafe extern "C" fn (this: *const nsIFaviconService, aDefaultFavicon: *mut *const nsIURI) -> nsresult,

}


impl nsIFaviconService {
    /* nsIURI getFaviconLinkForIcon (in nsIURI aFaviconURI); */
    #[inline]
    pub unsafe fn getFaviconLinkForIcon(&self, aFaviconURI: Option<&nsIURI>) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFaviconLinkForIcon)(self as *const _, aFaviconURI.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void expireAllFavicons (); */
    #[inline]
    pub unsafe fn expireAllFavicons(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).expireAllFavicons)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* unsigned short preferredSizeFromURI (in nsIURI aURI); */
    #[inline]
    pub unsafe fn preferredSizeFromURI(&self, aURI: Option<&nsIURI>) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).preferredSizeFromURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addFailedFavicon (in nsIURI aFaviconURI); */
    #[inline]
    pub unsafe fn addFailedFavicon(&self, aFaviconURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).addFailedFavicon)(self as *const _, aFaviconURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeFailedFavicon (in nsIURI aFaviconURI); */
    #[inline]
    pub unsafe fn removeFailedFavicon(&self, aFaviconURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).removeFailedFavicon)(self as *const _, aFaviconURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isFailedFavicon (in nsIURI aFaviconURI); */
    #[inline]
    pub unsafe fn isFailedFavicon(&self, aFaviconURI: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isFailedFavicon)(self as *const _, aFaviconURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIURI defaultFavicon; */
    #[inline]
    pub unsafe fn get_defaultFavicon(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_defaultFavicon)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIFaviconDataCallback {
    vtable: *const nsIFaviconDataCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFaviconDataCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc85e5c82, 0xb70f, 0x4621,
            [0x95, 0x28, 0xbe, 0xb2, 0xaa, 0x47, 0xfb, 0x44])
    }
}

unsafe impl RefCounted for nsIFaviconDataCallback {
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
pub trait nsIFaviconDataCallbackCoerce {
    fn coerce_from(v: &nsIFaviconDataCallback) -> &Self;
}

impl nsIFaviconDataCallbackCoerce for nsIFaviconDataCallback {
    #[inline]
    fn coerce_from(v: &nsIFaviconDataCallback) -> &Self {
        v
    }
}

impl nsIFaviconDataCallback {
    #[inline]
    pub fn coerce<T: nsIFaviconDataCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFaviconDataCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFaviconDataCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFaviconDataCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFaviconDataCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onComplete (in nsIURI aFaviconURI, in unsigned long aDataLen, [array, size_is (aDataLen), const] in octet aData, in AUTF8String aMimeType, in unsigned short aWidth); */
    /// Unable to call function as its signature contains a non-rust type
    pub onComplete: *const ::libc::c_void,

}


impl nsIFaviconDataCallback {
    /* void onComplete (in nsIURI aFaviconURI, in unsigned long aDataLen, [array, size_is (aDataLen), const] in octet aData, in AUTF8String aMimeType, in unsigned short aWidth); */


}


