//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocShellLoadInfo.idl
//


pub type nsDocShellInfoLoadType = libc::int32_t;


pub type nsDocShellInfoReferrerPolicy = libc::uint32_t;


pub mod nsIDocShellLoadInfo_consts {
    pub const loadNormal: i64 = 0;
    pub const loadNormalReplace: i64 = 1;
    pub const loadHistory: i64 = 2;
    pub const loadReloadNormal: i64 = 3;
    pub const loadReloadBypassCache: i64 = 4;
    pub const loadReloadBypassProxy: i64 = 5;
    pub const loadReloadBypassProxyAndCache: i64 = 6;
    pub const loadLink: i64 = 7;
    pub const loadRefresh: i64 = 8;
    pub const loadReloadCharsetChange: i64 = 9;
    pub const loadBypassHistory: i64 = 10;
    pub const loadStopContent: i64 = 11;
    pub const loadStopContentAndReplace: i64 = 12;
    pub const loadNormalExternal: i64 = 13;
    pub const loadNormalBypassCache: i64 = 14;
    pub const loadNormalBypassProxy: i64 = 15;
    pub const loadNormalBypassProxyAndCache: i64 = 16;
    pub const loadPushState: i64 = 17;
    pub const loadReplaceBypassCache: i64 = 18;
    pub const loadReloadMixedContent: i64 = 19;
    pub const loadNormalAllowMixedContent: i64 = 20;
}


#[repr(C)]
pub struct nsIDocShellLoadInfo {
    vtable: *const nsIDocShellLoadInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDocShellLoadInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe7570e5a, 0xf1d6, 0x452d,
            [0xb4, 0xf8, 0xb3, 0x5f, 0xdc, 0x63, 0xaa, 0x03])
    }
}

unsafe impl RefCounted for nsIDocShellLoadInfo {
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
pub trait nsIDocShellLoadInfoCoerce {
    fn coerce_from(v: &nsIDocShellLoadInfo) -> &Self;
}

impl nsIDocShellLoadInfoCoerce for nsIDocShellLoadInfo {
    #[inline]
    fn coerce_from(v: &nsIDocShellLoadInfo) -> &Self {
        v
    }
}

impl nsIDocShellLoadInfo {
    #[inline]
    pub fn coerce<T: nsIDocShellLoadInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDocShellLoadInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDocShellLoadInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocShellLoadInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDocShellLoadInfoVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIURI referrer; */
    pub get_referrer: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aReferrer: *mut *const nsIURI) -> nsresult,
    pub set_referrer: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aReferrer: *const nsIURI) -> nsresult,

    /* attribute nsIURI originalURI; */
    pub get_originalURI: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aOriginalURI: *mut *const nsIURI) -> nsresult,
    pub set_originalURI: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aOriginalURI: *const nsIURI) -> nsresult,

    /* attribute boolean loadReplace; */
    pub get_loadReplace: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aLoadReplace: *mut bool) -> nsresult,
    pub set_loadReplace: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aLoadReplace: bool) -> nsresult,

    /* attribute nsIPrincipal triggeringPrincipal; */
    pub get_triggeringPrincipal: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aTriggeringPrincipal: *mut *const nsIPrincipal) -> nsresult,
    pub set_triggeringPrincipal: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aTriggeringPrincipal: *const nsIPrincipal) -> nsresult,

    /* attribute boolean inheritPrincipal; */
    pub get_inheritPrincipal: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aInheritPrincipal: *mut bool) -> nsresult,
    pub set_inheritPrincipal: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aInheritPrincipal: bool) -> nsresult,

    /* attribute boolean principalIsExplicit; */
    pub get_principalIsExplicit: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aPrincipalIsExplicit: *mut bool) -> nsresult,
    pub set_principalIsExplicit: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aPrincipalIsExplicit: bool) -> nsresult,

    /* attribute nsDocShellInfoLoadType loadType; */
    pub get_loadType: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aLoadType: *mut nsDocShellInfoLoadType) -> nsresult,
    pub set_loadType: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aLoadType: nsDocShellInfoLoadType) -> nsresult,

    /* attribute nsISHEntry SHEntry; */
    pub get_SHEntry: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aSHEntry: *mut *const nsISHEntry) -> nsresult,
    pub set_SHEntry: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aSHEntry: *const nsISHEntry) -> nsresult,

    /* attribute wstring target; */
    pub get_target: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aTarget: *mut *const libc::int16_t) -> nsresult,
    pub set_target: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aTarget: *const libc::int16_t) -> nsresult,

    /* attribute nsIInputStream postDataStream; */
    pub get_postDataStream: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aPostDataStream: *mut *const nsIInputStream) -> nsresult,
    pub set_postDataStream: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aPostDataStream: *const nsIInputStream) -> nsresult,

    /* attribute nsIInputStream headersStream; */
    pub get_headersStream: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aHeadersStream: *mut *const nsIInputStream) -> nsresult,
    pub set_headersStream: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aHeadersStream: *const nsIInputStream) -> nsresult,

    /* attribute boolean sendReferrer; */
    pub get_sendReferrer: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aSendReferrer: *mut bool) -> nsresult,
    pub set_sendReferrer: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aSendReferrer: bool) -> nsresult,

    /* attribute nsDocShellInfoReferrerPolicy referrerPolicy; */
    pub get_referrerPolicy: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aReferrerPolicy: *mut nsDocShellInfoReferrerPolicy) -> nsresult,
    pub set_referrerPolicy: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aReferrerPolicy: nsDocShellInfoReferrerPolicy) -> nsresult,

    /* readonly attribute boolean isSrcdocLoad; */
    pub get_isSrcdocLoad: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aIsSrcdocLoad: *mut bool) -> nsresult,

    /* attribute AString srcdocData; */
    pub get_srcdocData: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aSrcdocData: *mut nsAString) -> nsresult,
    pub set_srcdocData: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aSrcdocData: *const nsAString) -> nsresult,

    /* attribute nsIDocShell sourceDocShell; */
    pub get_sourceDocShell: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aSourceDocShell: *mut *const nsIDocShell) -> nsresult,
    pub set_sourceDocShell: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aSourceDocShell: *const nsIDocShell) -> nsresult,

    /* attribute nsIURI baseURI; */
    pub get_baseURI: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aBaseURI: *mut *const nsIURI) -> nsresult,
    pub set_baseURI: unsafe extern "C" fn (this: *const nsIDocShellLoadInfo, aBaseURI: *const nsIURI) -> nsresult,

}


impl nsIDocShellLoadInfo {
    /* attribute nsIURI referrer; */
    #[inline]
    pub unsafe fn get_referrer(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_referrer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_referrer(&self, aReferrer: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_referrer)(self as *const _, aReferrer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIURI originalURI; */
    #[inline]
    pub unsafe fn get_originalURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_originalURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_originalURI(&self, aOriginalURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_originalURI)(self as *const _, aOriginalURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean loadReplace; */
    #[inline]
    pub unsafe fn get_loadReplace(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_loadReplace)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_loadReplace(&self, aLoadReplace: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_loadReplace)(self as *const _, aLoadReplace) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIPrincipal triggeringPrincipal; */
    #[inline]
    pub unsafe fn get_triggeringPrincipal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_triggeringPrincipal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_triggeringPrincipal(&self, aTriggeringPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {

        match ((*self.vtable).set_triggeringPrincipal)(self as *const _, aTriggeringPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean inheritPrincipal; */
    #[inline]
    pub unsafe fn get_inheritPrincipal(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_inheritPrincipal)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_inheritPrincipal(&self, aInheritPrincipal: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_inheritPrincipal)(self as *const _, aInheritPrincipal) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean principalIsExplicit; */
    #[inline]
    pub unsafe fn get_principalIsExplicit(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_principalIsExplicit)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_principalIsExplicit(&self, aPrincipalIsExplicit: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_principalIsExplicit)(self as *const _, aPrincipalIsExplicit) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsDocShellInfoLoadType loadType; */
    #[inline]
    pub unsafe fn get_loadType(&self, ) -> Result<nsDocShellInfoLoadType, nsresult> {
        let mut _retval: nsDocShellInfoLoadType = ::std::mem::zeroed();
        match ((*self.vtable).get_loadType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_loadType(&self, aLoadType: nsDocShellInfoLoadType) -> Result<(), nsresult> {

        match ((*self.vtable).set_loadType)(self as *const _, aLoadType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsISHEntry SHEntry; */
    #[inline]
    pub unsafe fn get_SHEntry(&self, ) -> Result<Option<RefPtr<nsISHEntry>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_SHEntry)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_SHEntry(&self, aSHEntry: Option<&nsISHEntry>) -> Result<(), nsresult> {

        match ((*self.vtable).set_SHEntry)(self as *const _, aSHEntry.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring target; */
    #[inline]
    pub unsafe fn get_target(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_target)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_target(&self, aTarget: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_target)(self as *const _, aTarget) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIInputStream postDataStream; */
    #[inline]
    pub unsafe fn get_postDataStream(&self, ) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_postDataStream)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_postDataStream(&self, aPostDataStream: Option<&nsIInputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).set_postDataStream)(self as *const _, aPostDataStream.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIInputStream headersStream; */
    #[inline]
    pub unsafe fn get_headersStream(&self, ) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_headersStream)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_headersStream(&self, aHeadersStream: Option<&nsIInputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).set_headersStream)(self as *const _, aHeadersStream.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean sendReferrer; */
    #[inline]
    pub unsafe fn get_sendReferrer(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_sendReferrer)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_sendReferrer(&self, aSendReferrer: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_sendReferrer)(self as *const _, aSendReferrer) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsDocShellInfoReferrerPolicy referrerPolicy; */
    #[inline]
    pub unsafe fn get_referrerPolicy(&self, ) -> Result<nsDocShellInfoReferrerPolicy, nsresult> {
        let mut _retval: nsDocShellInfoReferrerPolicy = ::std::mem::zeroed();
        match ((*self.vtable).get_referrerPolicy)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_referrerPolicy(&self, aReferrerPolicy: nsDocShellInfoReferrerPolicy) -> Result<(), nsresult> {

        match ((*self.vtable).set_referrerPolicy)(self as *const _, aReferrerPolicy) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean isSrcdocLoad; */
    #[inline]
    pub unsafe fn get_isSrcdocLoad(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSrcdocLoad)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute AString srcdocData; */
    #[inline]
    pub unsafe fn get_srcdocData(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_srcdocData)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_srcdocData(&self, aSrcdocData: &[u16]) -> Result<(), nsresult> {
        let aSrcdocData = nsString::from(aSrcdocData);
        match ((*self.vtable).set_srcdocData)(self as *const _, &*aSrcdocData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIDocShell sourceDocShell; */
    #[inline]
    pub unsafe fn get_sourceDocShell(&self, ) -> Result<Option<RefPtr<nsIDocShell>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_sourceDocShell)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_sourceDocShell(&self, aSourceDocShell: Option<&nsIDocShell>) -> Result<(), nsresult> {

        match ((*self.vtable).set_sourceDocShell)(self as *const _, aSourceDocShell.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIURI baseURI; */
    #[inline]
    pub unsafe fn get_baseURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_baseURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_baseURI(&self, aBaseURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_baseURI)(self as *const _, aBaseURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


