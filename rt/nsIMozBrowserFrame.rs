//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMozBrowserFrame.idl
//


#[repr(C)]
pub struct nsIMozBrowserFrame {
    vtable: *const nsIMozBrowserFrameVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMozBrowserFrame {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0c0a862c, 0x1a47, 0x43c0,
            [0xae, 0x9e, 0xd5, 0x18, 0x35, 0xe3, 0xe1, 0xa6])
    }
}

unsafe impl RefCounted for nsIMozBrowserFrame {
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
pub trait nsIMozBrowserFrameCoerce {
    fn coerce_from(v: &nsIMozBrowserFrame) -> &Self;
}

impl nsIMozBrowserFrameCoerce for nsIMozBrowserFrame {
    #[inline]
    fn coerce_from(v: &nsIMozBrowserFrame) -> &Self {
        v
    }
}

impl nsIMozBrowserFrame {
    #[inline]
    pub fn coerce<T: nsIMozBrowserFrameCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMozBrowserFrame {
    type Target = nsIDOMMozBrowserFrame;
    #[inline]
    fn deref(&self) -> &nsIDOMMozBrowserFrame {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMMozBrowserFrameCoerce> nsIMozBrowserFrameCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMozBrowserFrame) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMozBrowserFrameVTable {
    pub __base: nsIDOMMozBrowserFrameVTable,

    /* [infallible] readonly attribute boolean reallyIsBrowser; */
    pub get_reallyIsBrowser: unsafe extern "C" fn (this: *const nsIMozBrowserFrame, aReallyIsBrowser: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean isolated; */
    pub get_isolated: unsafe extern "C" fn (this: *const nsIMozBrowserFrame, aIsolated: *mut bool) -> nsresult,

    /* void disallowCreateFrameLoader (); */
    pub disallowCreateFrameLoader: unsafe extern "C" fn (this: *const nsIMozBrowserFrame) -> nsresult,

    /* void allowCreateFrameLoader (); */
    pub allowCreateFrameLoader: unsafe extern "C" fn (this: *const nsIMozBrowserFrame) -> nsresult,

    /* void createRemoteFrameLoader (in nsITabParent aTabParent); */
    pub createRemoteFrameLoader: unsafe extern "C" fn (this: *const nsIMozBrowserFrame, aTabParent: *const nsITabParent) -> nsresult,

    /* [noscript] void initializeBrowserAPI (); */
    pub initializeBrowserAPI: unsafe extern "C" fn (this: *const nsIMozBrowserFrame) -> nsresult,

    /* [noscript] void destroyBrowserFrameScripts (); */
    pub destroyBrowserFrameScripts: unsafe extern "C" fn (this: *const nsIMozBrowserFrame) -> nsresult,

}


impl nsIMozBrowserFrame {
    /* [infallible] readonly attribute boolean reallyIsBrowser; */
    #[inline]
    pub unsafe fn get_reallyIsBrowser(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_reallyIsBrowser)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean isolated; */
    #[inline]
    pub unsafe fn get_isolated(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isolated)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void disallowCreateFrameLoader (); */
    #[inline]
    pub unsafe fn disallowCreateFrameLoader(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).disallowCreateFrameLoader)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void allowCreateFrameLoader (); */
    #[inline]
    pub unsafe fn allowCreateFrameLoader(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).allowCreateFrameLoader)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void createRemoteFrameLoader (in nsITabParent aTabParent); */
    #[inline]
    pub unsafe fn createRemoteFrameLoader(&self, aTabParent: Option<&nsITabParent>) -> Result<(), nsresult> {

        match ((*self.vtable).createRemoteFrameLoader)(self as *const _, aTabParent.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void initializeBrowserAPI (); */
    #[inline]
    pub unsafe fn initializeBrowserAPI(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).initializeBrowserAPI)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void destroyBrowserFrameScripts (); */
    #[inline]
    pub unsafe fn destroyBrowserFrameScripts(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).destroyBrowserFrameScripts)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


