//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURIFixup.idl
//


#[repr(C)]
pub struct nsIURIFixupInfo {
    vtable: *const nsIURIFixupInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIURIFixupInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4819f183, 0xb532, 0x4932,
            [0xac, 0x09, 0xb3, 0x09, 0xcd, 0x85, 0x3b, 0xe7])
    }
}

unsafe impl RefCounted for nsIURIFixupInfo {
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
pub trait nsIURIFixupInfoCoerce {
    fn coerce_from(v: &nsIURIFixupInfo) -> &Self;
}

impl nsIURIFixupInfoCoerce for nsIURIFixupInfo {
    #[inline]
    fn coerce_from(v: &nsIURIFixupInfo) -> &Self {
        v
    }
}

impl nsIURIFixupInfo {
    #[inline]
    pub fn coerce<T: nsIURIFixupInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIURIFixupInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIURIFixupInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURIFixupInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIURIFixupInfoVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsISupports consumer; */
    pub get_consumer: unsafe extern "C" fn (this: *const nsIURIFixupInfo, aConsumer: *mut *const nsISupports) -> nsresult,
    pub set_consumer: unsafe extern "C" fn (this: *const nsIURIFixupInfo, aConsumer: *const nsISupports) -> nsresult,

    /* readonly attribute nsIURI preferredURI; */
    pub get_preferredURI: unsafe extern "C" fn (this: *const nsIURIFixupInfo, aPreferredURI: *mut *const nsIURI) -> nsresult,

    /* readonly attribute nsIURI fixedURI; */
    pub get_fixedURI: unsafe extern "C" fn (this: *const nsIURIFixupInfo, aFixedURI: *mut *const nsIURI) -> nsresult,

    /* readonly attribute AString keywordProviderName; */
    pub get_keywordProviderName: unsafe extern "C" fn (this: *const nsIURIFixupInfo, aKeywordProviderName: *mut nsAString) -> nsresult,

    /* readonly attribute AString keywordAsSent; */
    pub get_keywordAsSent: unsafe extern "C" fn (this: *const nsIURIFixupInfo, aKeywordAsSent: *mut nsAString) -> nsresult,

    /* readonly attribute boolean fixupChangedProtocol; */
    pub get_fixupChangedProtocol: unsafe extern "C" fn (this: *const nsIURIFixupInfo, aFixupChangedProtocol: *mut bool) -> nsresult,

    /* readonly attribute boolean fixupCreatedAlternateURI; */
    pub get_fixupCreatedAlternateURI: unsafe extern "C" fn (this: *const nsIURIFixupInfo, aFixupCreatedAlternateURI: *mut bool) -> nsresult,

    /* readonly attribute AUTF8String originalInput; */
    pub get_originalInput: unsafe extern "C" fn (this: *const nsIURIFixupInfo, aOriginalInput: *mut nsACString) -> nsresult,

}


impl nsIURIFixupInfo {
    /* attribute nsISupports consumer; */
    #[inline]
    pub unsafe fn get_consumer(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_consumer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_consumer(&self, aConsumer: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).set_consumer)(self as *const _, aConsumer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIURI preferredURI; */
    #[inline]
    pub unsafe fn get_preferredURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_preferredURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIURI fixedURI; */
    #[inline]
    pub unsafe fn get_fixedURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_fixedURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AString keywordProviderName; */
    #[inline]
    pub unsafe fn get_keywordProviderName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_keywordProviderName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString keywordAsSent; */
    #[inline]
    pub unsafe fn get_keywordAsSent(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_keywordAsSent)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean fixupChangedProtocol; */
    #[inline]
    pub unsafe fn get_fixupChangedProtocol(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_fixupChangedProtocol)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean fixupCreatedAlternateURI; */
    #[inline]
    pub unsafe fn get_fixupCreatedAlternateURI(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_fixupCreatedAlternateURI)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String originalInput; */
    #[inline]
    pub unsafe fn get_originalInput(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_originalInput)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


pub mod nsIURIFixup_consts {
    pub const FIXUP_FLAG_NONE: i64 = 0;
    pub const FIXUP_FLAG_ALLOW_KEYWORD_LOOKUP: i64 = 1;
    pub const FIXUP_FLAGS_MAKE_ALTERNATE_URI: i64 = 2;
    pub const FIXUP_FLAG_FIX_SCHEME_TYPOS: i64 = 8;
}


#[repr(C)]
pub struct nsIURIFixup {
    vtable: *const nsIURIFixupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIURIFixup {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1da7e9d4, 0x620b, 0x4949,
            [0x84, 0x9a, 0x1c, 0xd6, 0x07, 0x7b, 0x1b, 0x2d])
    }
}

unsafe impl RefCounted for nsIURIFixup {
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
pub trait nsIURIFixupCoerce {
    fn coerce_from(v: &nsIURIFixup) -> &Self;
}

impl nsIURIFixupCoerce for nsIURIFixup {
    #[inline]
    fn coerce_from(v: &nsIURIFixup) -> &Self {
        v
    }
}

impl nsIURIFixup {
    #[inline]
    pub fn coerce<T: nsIURIFixupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIURIFixup {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIURIFixupCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURIFixup) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIURIFixupVTable {
    pub __base: nsISupportsVTable,

    /* nsIURI createExposableURI (in nsIURI aURI); */
    pub createExposableURI: unsafe extern "C" fn (this: *const nsIURIFixup, aURI: *const nsIURI, _retval: *mut *const nsIURI) -> nsresult,

    /* nsIURI createFixupURI (in AUTF8String aURIText, in unsigned long aFixupFlags, [optional] out nsIInputStream aPostData); */
    pub createFixupURI: unsafe extern "C" fn (this: *const nsIURIFixup, aURIText: *const nsACString, aFixupFlags: libc::uint32_t, aPostData: *mut *const nsIInputStream, _retval: *mut *const nsIURI) -> nsresult,

    /* nsIURIFixupInfo getFixupURIInfo (in AUTF8String aURIText, in unsigned long aFixupFlags, [optional] out nsIInputStream aPostData); */
    pub getFixupURIInfo: unsafe extern "C" fn (this: *const nsIURIFixup, aURIText: *const nsACString, aFixupFlags: libc::uint32_t, aPostData: *mut *const nsIInputStream, _retval: *mut *const nsIURIFixupInfo) -> nsresult,

    /* nsIURIFixupInfo keywordToURI (in AUTF8String aKeyword, [optional] out nsIInputStream aPostData); */
    pub keywordToURI: unsafe extern "C" fn (this: *const nsIURIFixup, aKeyword: *const nsACString, aPostData: *mut *const nsIInputStream, _retval: *mut *const nsIURIFixupInfo) -> nsresult,

    /* bool isDomainWhitelisted (in AUTF8String aDomain, in uint32_t aDotPos); */
    pub isDomainWhitelisted: unsafe extern "C" fn (this: *const nsIURIFixup, aDomain: *const nsACString, aDotPos: uint32_t, _retval: *mut bool) -> nsresult,

}


impl nsIURIFixup {
    /* nsIURI createExposableURI (in nsIURI aURI); */
    #[inline]
    pub unsafe fn createExposableURI(&self, aURI: Option<&nsIURI>) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createExposableURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIURI createFixupURI (in AUTF8String aURIText, in unsigned long aFixupFlags, [optional] out nsIInputStream aPostData); */
    #[inline]
    pub unsafe fn createFixupURI(&self, aURIText: &[u8], aFixupFlags: libc::uint32_t) -> Result<(Option<RefPtr<nsIInputStream>>, Option<RefPtr<nsIURI>>), nsresult> {
        let aURIText = nsCString::from(aURIText);
        let mut aPostData = GetterAddrefs::new();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createFixupURI)(self as *const _, &*aURIText, aFixupFlags, aPostData.ptr(), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aPostData.refptr(), _retval.refptr()))
    }

    /* nsIURIFixupInfo getFixupURIInfo (in AUTF8String aURIText, in unsigned long aFixupFlags, [optional] out nsIInputStream aPostData); */
    #[inline]
    pub unsafe fn getFixupURIInfo(&self, aURIText: &[u8], aFixupFlags: libc::uint32_t) -> Result<(Option<RefPtr<nsIInputStream>>, Option<RefPtr<nsIURIFixupInfo>>), nsresult> {
        let aURIText = nsCString::from(aURIText);
        let mut aPostData = GetterAddrefs::new();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFixupURIInfo)(self as *const _, &*aURIText, aFixupFlags, aPostData.ptr(), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aPostData.refptr(), _retval.refptr()))
    }

    /* nsIURIFixupInfo keywordToURI (in AUTF8String aKeyword, [optional] out nsIInputStream aPostData); */
    #[inline]
    pub unsafe fn keywordToURI(&self, aKeyword: &[u8]) -> Result<(Option<RefPtr<nsIInputStream>>, Option<RefPtr<nsIURIFixupInfo>>), nsresult> {
        let aKeyword = nsCString::from(aKeyword);
        let mut aPostData = GetterAddrefs::new();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).keywordToURI)(self as *const _, &*aKeyword, aPostData.ptr(), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aPostData.refptr(), _retval.refptr()))
    }

    /* bool isDomainWhitelisted (in AUTF8String aDomain, in uint32_t aDotPos); */
    #[inline]
    pub unsafe fn isDomainWhitelisted(&self, aDomain: &[u8], aDotPos: uint32_t) -> Result<bool, nsresult> {
        let aDomain = nsCString::from(aDomain);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isDomainWhitelisted)(self as *const _, &*aDomain, aDotPos, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


