//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWindowCreator2.idl
//


#[repr(C)]
pub struct nsIWindowCreator2 {
    vtable: *const nsIWindowCreator2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWindowCreator2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb6c44689, 0xf97e, 0x4f32,
            [0xa7, 0x23, 0x29, 0xee, 0xdd, 0xfb, 0xdc, 0x53])
    }
}

unsafe impl RefCounted for nsIWindowCreator2 {
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
pub trait nsIWindowCreator2Coerce {
    fn coerce_from(v: &nsIWindowCreator2) -> &Self;
}

impl nsIWindowCreator2Coerce for nsIWindowCreator2 {
    #[inline]
    fn coerce_from(v: &nsIWindowCreator2) -> &Self {
        v
    }
}

impl nsIWindowCreator2 {
    #[inline]
    pub fn coerce<T: nsIWindowCreator2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWindowCreator2 {
    type Target = nsIWindowCreator;
    #[inline]
    fn deref(&self) -> &nsIWindowCreator {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIWindowCreatorCoerce> nsIWindowCreator2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIWindowCreator2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWindowCreator2VTable {
    pub __base: nsIWindowCreatorVTable,

    /* nsIWebBrowserChrome createChromeWindow2 (in nsIWebBrowserChrome parent, in uint32_t chromeFlags, in nsITabParent aOpeningTab, in mozIDOMWindowProxy aOpener, in unsigned long long aNextTabParentId, out boolean cancel); */
    pub createChromeWindow2: unsafe extern "C" fn (this: *const nsIWindowCreator2, parent: *const nsIWebBrowserChrome, chromeFlags: uint32_t, aOpeningTab: *const nsITabParent, aOpener: *const mozIDOMWindowProxy, aNextTabParentId: libc::uint64_t, cancel: *mut bool, _retval: *mut *const nsIWebBrowserChrome) -> nsresult,

    /* [noscript] void setScreenId (in uint32_t aScreenId); */
    pub setScreenId: unsafe extern "C" fn (this: *const nsIWindowCreator2, aScreenId: uint32_t) -> nsresult,

}


impl nsIWindowCreator2 {
    /* nsIWebBrowserChrome createChromeWindow2 (in nsIWebBrowserChrome parent, in uint32_t chromeFlags, in nsITabParent aOpeningTab, in mozIDOMWindowProxy aOpener, in unsigned long long aNextTabParentId, out boolean cancel); */
    #[inline]
    pub unsafe fn createChromeWindow2(&self, parent: Option<&nsIWebBrowserChrome>, chromeFlags: uint32_t, aOpeningTab: Option<&nsITabParent>, aOpener: Option<&mozIDOMWindowProxy>, aNextTabParentId: libc::uint64_t) -> Result<(bool, Option<RefPtr<nsIWebBrowserChrome>>), nsresult> {
        let mut cancel: bool = ::std::mem::zeroed();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createChromeWindow2)(self as *const _, parent.map_or(::std::ptr::null(), |x| x as *const _), chromeFlags, aOpeningTab.map_or(::std::ptr::null(), |x| x as *const _), aOpener.map_or(::std::ptr::null(), |x| x as *const _), aNextTabParentId, &mut cancel as *mut _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((cancel, _retval.refptr()))
    }

    /* [noscript] void setScreenId (in uint32_t aScreenId); */
    #[inline]
    pub unsafe fn setScreenId(&self, aScreenId: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setScreenId)(self as *const _, aScreenId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


