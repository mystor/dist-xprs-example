//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoadContext.idl
//


#[repr(C)]
pub struct nsILoadContext {
    vtable: *const nsILoadContextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILoadContext {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2813a7a3, 0xd084, 0x4d00,
            [0xac, 0xd0, 0xf7, 0x66, 0x20, 0x31, 0x5c, 0x02])
    }
}

unsafe impl RefCounted for nsILoadContext {
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
pub trait nsILoadContextCoerce {
    fn coerce_from(v: &nsILoadContext) -> &Self;
}

impl nsILoadContextCoerce for nsILoadContext {
    #[inline]
    fn coerce_from(v: &nsILoadContext) -> &Self {
        v
    }
}

impl nsILoadContext {
    #[inline]
    pub fn coerce<T: nsILoadContextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILoadContext {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILoadContextCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoadContext) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILoadContextVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute mozIDOMWindowProxy associatedWindow; */
    pub get_associatedWindow: unsafe extern "C" fn (this: *const nsILoadContext, aAssociatedWindow: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* readonly attribute mozIDOMWindowProxy topWindow; */
    pub get_topWindow: unsafe extern "C" fn (this: *const nsILoadContext, aTopWindow: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* readonly attribute nsIDOMElement topFrameElement; */
    pub get_topFrameElement: unsafe extern "C" fn (this: *const nsILoadContext, aTopFrameElement: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute unsigned long long nestedFrameId; */
    pub get_nestedFrameId: unsafe extern "C" fn (this: *const nsILoadContext, aNestedFrameId: *mut libc::uint64_t) -> nsresult,

    /* readonly attribute boolean isContent; */
    pub get_isContent: unsafe extern "C" fn (this: *const nsILoadContext, aIsContent: *mut bool) -> nsresult,

    /* attribute boolean usePrivateBrowsing; */
    pub get_usePrivateBrowsing: unsafe extern "C" fn (this: *const nsILoadContext, aUsePrivateBrowsing: *mut bool) -> nsresult,
    pub set_usePrivateBrowsing: unsafe extern "C" fn (this: *const nsILoadContext, aUsePrivateBrowsing: bool) -> nsresult,

    /* readonly attribute boolean useRemoteTabs; */
    pub get_useRemoteTabs: unsafe extern "C" fn (this: *const nsILoadContext, aUseRemoteTabs: *mut bool) -> nsresult,

    /* attribute boolean useTrackingProtection; */
    pub get_useTrackingProtection: unsafe extern "C" fn (this: *const nsILoadContext, aUseTrackingProtection: *mut bool) -> nsresult,
    pub set_useTrackingProtection: unsafe extern "C" fn (this: *const nsILoadContext, aUseTrackingProtection: bool) -> nsresult,

    /* [noscript] void SetPrivateBrowsing (in boolean aInPrivateBrowsing); */
    pub SetPrivateBrowsing: unsafe extern "C" fn (this: *const nsILoadContext, aInPrivateBrowsing: bool) -> nsresult,

    /* [noscript] void SetRemoteTabs (in boolean aUseRemoteTabs); */
    pub SetRemoteTabs: unsafe extern "C" fn (this: *const nsILoadContext, aUseRemoteTabs: bool) -> nsresult,

    /* readonly attribute boolean isInIsolatedMozBrowserElement; */
    pub get_isInIsolatedMozBrowserElement: unsafe extern "C" fn (this: *const nsILoadContext, aIsInIsolatedMozBrowserElement: *mut bool) -> nsresult,

    /* [binaryname(ScriptableOriginAttributes)] readonly attribute jsval originAttributes; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_ScriptableOriginAttributes: *const ::libc::c_void,

    /* [noscript,notxpcom] void GetOriginAttributes (out OriginAttributes aAttrs); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetOriginAttributes: *const ::libc::c_void,

}


impl nsILoadContext {
    /* readonly attribute mozIDOMWindowProxy associatedWindow; */
    #[inline]
    pub unsafe fn get_associatedWindow(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_associatedWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute mozIDOMWindowProxy topWindow; */
    #[inline]
    pub unsafe fn get_topWindow(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_topWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMElement topFrameElement; */
    #[inline]
    pub unsafe fn get_topFrameElement(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_topFrameElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute unsigned long long nestedFrameId; */
    #[inline]
    pub unsafe fn get_nestedFrameId(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_nestedFrameId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isContent; */
    #[inline]
    pub unsafe fn get_isContent(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isContent)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean usePrivateBrowsing; */
    #[inline]
    pub unsafe fn get_usePrivateBrowsing(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_usePrivateBrowsing)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_usePrivateBrowsing(&self, aUsePrivateBrowsing: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_usePrivateBrowsing)(self as *const _, aUsePrivateBrowsing) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean useRemoteTabs; */
    #[inline]
    pub unsafe fn get_useRemoteTabs(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_useRemoteTabs)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean useTrackingProtection; */
    #[inline]
    pub unsafe fn get_useTrackingProtection(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_useTrackingProtection)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_useTrackingProtection(&self, aUseTrackingProtection: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_useTrackingProtection)(self as *const _, aUseTrackingProtection) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void SetPrivateBrowsing (in boolean aInPrivateBrowsing); */
    #[inline]
    pub unsafe fn SetPrivateBrowsing(&self, aInPrivateBrowsing: bool) -> Result<(), nsresult> {

        match ((*self.vtable).SetPrivateBrowsing)(self as *const _, aInPrivateBrowsing) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void SetRemoteTabs (in boolean aUseRemoteTabs); */
    #[inline]
    pub unsafe fn SetRemoteTabs(&self, aUseRemoteTabs: bool) -> Result<(), nsresult> {

        match ((*self.vtable).SetRemoteTabs)(self as *const _, aUseRemoteTabs) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean isInIsolatedMozBrowserElement; */
    #[inline]
    pub unsafe fn get_isInIsolatedMozBrowserElement(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isInIsolatedMozBrowserElement)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [binaryname(ScriptableOriginAttributes)] readonly attribute jsval originAttributes; */


    /* [noscript,notxpcom] void GetOriginAttributes (out OriginAttributes aAttrs); */


}


