//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowser.idl
//


#[repr(C)]
pub struct nsIWebBrowser {
    vtable: *const nsIWebBrowserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowser {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4052b6da, 0x4faa, 0x4646,
            [0xb3, 0xa1, 0x7e, 0x16, 0xa0, 0x1c, 0x2d, 0xc2])
    }
}

unsafe impl RefCounted for nsIWebBrowser {
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
pub trait nsIWebBrowserCoerce {
    fn coerce_from(v: &nsIWebBrowser) -> &Self;
}

impl nsIWebBrowserCoerce for nsIWebBrowser {
    #[inline]
    fn coerce_from(v: &nsIWebBrowser) -> &Self {
        v
    }
}

impl nsIWebBrowser {
    #[inline]
    pub fn coerce<T: nsIWebBrowserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowser {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebBrowserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowser) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserVTable {
    pub __base: nsISupportsVTable,

    /* void addWebBrowserListener (in nsIWeakReference aListener, in nsIIDRef aIID); */
    pub addWebBrowserListener: unsafe extern "C" fn (this: *const nsIWebBrowser, aListener: *const nsIWeakReference, aIID: *const nsIID) -> nsresult,

    /* void removeWebBrowserListener (in nsIWeakReference aListener, in nsIIDRef aIID); */
    pub removeWebBrowserListener: unsafe extern "C" fn (this: *const nsIWebBrowser, aListener: *const nsIWeakReference, aIID: *const nsIID) -> nsresult,

    /* attribute nsIWebBrowserChrome containerWindow; */
    pub get_containerWindow: unsafe extern "C" fn (this: *const nsIWebBrowser, aContainerWindow: *mut *const nsIWebBrowserChrome) -> nsresult,
    pub set_containerWindow: unsafe extern "C" fn (this: *const nsIWebBrowser, aContainerWindow: *const nsIWebBrowserChrome) -> nsresult,

    /* attribute nsIURIContentListener parentURIContentListener; */
    pub get_parentURIContentListener: unsafe extern "C" fn (this: *const nsIWebBrowser, aParentURIContentListener: *mut *const nsIURIContentListener) -> nsresult,
    pub set_parentURIContentListener: unsafe extern "C" fn (this: *const nsIWebBrowser, aParentURIContentListener: *const nsIURIContentListener) -> nsresult,

    /* readonly attribute mozIDOMWindowProxy contentDOMWindow; */
    pub get_contentDOMWindow: unsafe extern "C" fn (this: *const nsIWebBrowser, aContentDOMWindow: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* attribute boolean isActive; */
    pub get_isActive: unsafe extern "C" fn (this: *const nsIWebBrowser, aIsActive: *mut bool) -> nsresult,
    pub set_isActive: unsafe extern "C" fn (this: *const nsIWebBrowser, aIsActive: bool) -> nsresult,

    /* [binaryname(SetOriginAttributes),noscript,nostdcall,notxpcom] void binarySetOriginAttributes (in const_OriginAttributesRef aOriginAttrs); */
    /// Unable to call function as its signature contains a non-rust type
    pub SetOriginAttributes: *const ::libc::c_void,

}


impl nsIWebBrowser {
    /* void addWebBrowserListener (in nsIWeakReference aListener, in nsIIDRef aIID); */
    #[inline]
    pub unsafe fn addWebBrowserListener(&self, aListener: Option<&nsIWeakReference>, aIID: *const nsIID) -> Result<(), nsresult> {

        match ((*self.vtable).addWebBrowserListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _), aIID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeWebBrowserListener (in nsIWeakReference aListener, in nsIIDRef aIID); */
    #[inline]
    pub unsafe fn removeWebBrowserListener(&self, aListener: Option<&nsIWeakReference>, aIID: *const nsIID) -> Result<(), nsresult> {

        match ((*self.vtable).removeWebBrowserListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _), aIID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIWebBrowserChrome containerWindow; */
    #[inline]
    pub unsafe fn get_containerWindow(&self, ) -> Result<Option<RefPtr<nsIWebBrowserChrome>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_containerWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_containerWindow(&self, aContainerWindow: Option<&nsIWebBrowserChrome>) -> Result<(), nsresult> {

        match ((*self.vtable).set_containerWindow)(self as *const _, aContainerWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIURIContentListener parentURIContentListener; */
    #[inline]
    pub unsafe fn get_parentURIContentListener(&self, ) -> Result<Option<RefPtr<nsIURIContentListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parentURIContentListener)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_parentURIContentListener(&self, aParentURIContentListener: Option<&nsIURIContentListener>) -> Result<(), nsresult> {

        match ((*self.vtable).set_parentURIContentListener)(self as *const _, aParentURIContentListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute mozIDOMWindowProxy contentDOMWindow; */
    #[inline]
    pub unsafe fn get_contentDOMWindow(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_contentDOMWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute boolean isActive; */
    #[inline]
    pub unsafe fn get_isActive(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isActive)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isActive(&self, aIsActive: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isActive)(self as *const _, aIsActive) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [binaryname(SetOriginAttributes),noscript,nostdcall,notxpcom] void binarySetOriginAttributes (in const_OriginAttributesRef aOriginAttrs); */


}


