//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBrowser.idl
//


pub mod nsIBrowser_consts {
    pub const SWAP_DEFAULT: i64 = 0;
    pub const SWAP_KEEP_PERMANENT_KEY: i64 = 1;
}


#[repr(C)]
pub struct nsIBrowser {
    vtable: *const nsIBrowserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBrowser {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x14e5a0cb, 0xe223, 0x4202,
            [0x95, 0xe8, 0xfe, 0x53, 0x27, 0x51, 0x93, 0xea])
    }
}

unsafe impl RefCounted for nsIBrowser {
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
pub trait nsIBrowserCoerce {
    fn coerce_from(v: &nsIBrowser) -> &Self;
}

impl nsIBrowserCoerce for nsIBrowser {
    #[inline]
    fn coerce_from(v: &nsIBrowser) -> &Self {
        v
    }
}

impl nsIBrowser {
    #[inline]
    pub fn coerce<T: nsIBrowserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBrowser {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBrowserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowser) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBrowserVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIFrameLoader sameProcessAsFrameLoader; */
    pub get_sameProcessAsFrameLoader: unsafe extern "C" fn (this: *const nsIBrowser, aSameProcessAsFrameLoader: *mut *const nsIFrameLoader) -> nsresult,

    /* void dropLinks (in unsigned long linksCount, [array, size_is (linksCount)] in wstring links); */
    /// Unable to call function as its signature contains a non-rust type
    pub dropLinks: *const ::libc::c_void,

    /* void swapBrowsers (in nsIBrowser aOtherBrowser, in unsigned long aFlags); */
    pub swapBrowsers: unsafe extern "C" fn (this: *const nsIBrowser, aOtherBrowser: *const nsIBrowser, aFlags: libc::uint32_t) -> nsresult,

    /* void closeBrowser (); */
    pub closeBrowser: unsafe extern "C" fn (this: *const nsIBrowser) -> nsresult,

}


impl nsIBrowser {
    /* readonly attribute nsIFrameLoader sameProcessAsFrameLoader; */
    #[inline]
    pub unsafe fn get_sameProcessAsFrameLoader(&self, ) -> Result<Option<RefPtr<nsIFrameLoader>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_sameProcessAsFrameLoader)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void dropLinks (in unsigned long linksCount, [array, size_is (linksCount)] in wstring links); */


    /* void swapBrowsers (in nsIBrowser aOtherBrowser, in unsigned long aFlags); */
    #[inline]
    pub unsafe fn swapBrowsers(&self, aOtherBrowser: Option<&nsIBrowser>, aFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).swapBrowsers)(self as *const _, aOtherBrowser.map_or(::std::ptr::null(), |x| x as *const _), aFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void closeBrowser (); */
    #[inline]
    pub unsafe fn closeBrowser(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).closeBrowser)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


