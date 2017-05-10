//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserChrome.idl
//


pub mod nsIWebBrowserChrome_consts {
    pub const STATUS_SCRIPT: i64 = 1;
    pub const STATUS_LINK: i64 = 3;
    pub const CHROME_DEFAULT: i64 = 1;
    pub const CHROME_WINDOW_BORDERS: i64 = 2;
    pub const CHROME_WINDOW_CLOSE: i64 = 4;
    pub const CHROME_WINDOW_RESIZE: i64 = 8;
    pub const CHROME_MENUBAR: i64 = 16;
    pub const CHROME_TOOLBAR: i64 = 32;
    pub const CHROME_LOCATIONBAR: i64 = 64;
    pub const CHROME_STATUSBAR: i64 = 128;
    pub const CHROME_PERSONAL_TOOLBAR: i64 = 256;
    pub const CHROME_SCROLLBARS: i64 = 512;
    pub const CHROME_TITLEBAR: i64 = 1024;
    pub const CHROME_EXTRA: i64 = 2048;
    pub const CHROME_WITH_SIZE: i64 = 4096;
    pub const CHROME_WITH_POSITION: i64 = 8192;
    pub const CHROME_WINDOW_MIN: i64 = 16384;
    pub const CHROME_WINDOW_POPUP: i64 = 32768;
    pub const CHROME_PRIVATE_WINDOW: i64 = 65536;
    pub const CHROME_NON_PRIVATE_WINDOW: i64 = 131072;
    pub const CHROME_PRIVATE_LIFETIME: i64 = 262144;
    pub const CHROME_MODAL_CONTENT_WINDOW: i64 = 524288;
    pub const CHROME_REMOTE_WINDOW: i64 = 1048576;
    pub const CHROME_SUPPRESS_ANIMATION: i64 = 16777216;
    pub const CHROME_WINDOW_RAISED: i64 = 33554432;
    pub const CHROME_WINDOW_LOWERED: i64 = 67108864;
    pub const CHROME_CENTER_SCREEN: i64 = 134217728;
    pub const CHROME_DEPENDENT: i64 = 268435456;
    pub const CHROME_MODAL: i64 = 536870912;
    pub const CHROME_OPENAS_DIALOG: i64 = 1073741824;
    pub const CHROME_OPENAS_CHROME: i64 = 2147483648;
    pub const CHROME_ALL: i64 = 4094;
}


#[repr(C)]
pub struct nsIWebBrowserChrome {
    vtable: *const nsIWebBrowserChromeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserChrome {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe8c414c4, 0xdc38, 0x4ba3,
            [0xab, 0x4e, 0xec, 0x4c, 0xbb, 0xe2, 0x29, 0x07])
    }
}

unsafe impl RefCounted for nsIWebBrowserChrome {
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
pub trait nsIWebBrowserChromeCoerce {
    fn coerce_from(v: &nsIWebBrowserChrome) -> &Self;
}

impl nsIWebBrowserChromeCoerce for nsIWebBrowserChrome {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserChrome) -> &Self {
        v
    }
}

impl nsIWebBrowserChrome {
    #[inline]
    pub fn coerce<T: nsIWebBrowserChromeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserChrome {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebBrowserChromeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserChrome) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserChromeVTable {
    pub __base: nsISupportsVTable,

    /* void setStatus (in unsigned long statusType, in wstring status); */
    pub setStatus: unsafe extern "C" fn (this: *const nsIWebBrowserChrome, statusType: libc::uint32_t, status: *const libc::int16_t) -> nsresult,

    /* attribute nsIWebBrowser webBrowser; */
    pub get_webBrowser: unsafe extern "C" fn (this: *const nsIWebBrowserChrome, aWebBrowser: *mut *const nsIWebBrowser) -> nsresult,
    pub set_webBrowser: unsafe extern "C" fn (this: *const nsIWebBrowserChrome, aWebBrowser: *const nsIWebBrowser) -> nsresult,

    /* attribute unsigned long chromeFlags; */
    pub get_chromeFlags: unsafe extern "C" fn (this: *const nsIWebBrowserChrome, aChromeFlags: *mut libc::uint32_t) -> nsresult,
    pub set_chromeFlags: unsafe extern "C" fn (this: *const nsIWebBrowserChrome, aChromeFlags: libc::uint32_t) -> nsresult,

    /* void destroyBrowserWindow (); */
    pub destroyBrowserWindow: unsafe extern "C" fn (this: *const nsIWebBrowserChrome) -> nsresult,

    /* void sizeBrowserTo (in long aCX, in long aCY); */
    pub sizeBrowserTo: unsafe extern "C" fn (this: *const nsIWebBrowserChrome, aCX: libc::int32_t, aCY: libc::int32_t) -> nsresult,

    /* void showAsModal (); */
    pub showAsModal: unsafe extern "C" fn (this: *const nsIWebBrowserChrome) -> nsresult,

    /* boolean isWindowModal (); */
    pub isWindowModal: unsafe extern "C" fn (this: *const nsIWebBrowserChrome, _retval: *mut bool) -> nsresult,

    /* void exitModalEventLoop (in nsresult aStatus); */
    pub exitModalEventLoop: unsafe extern "C" fn (this: *const nsIWebBrowserChrome, aStatus: nsresult) -> nsresult,

}


impl nsIWebBrowserChrome {
    /* void setStatus (in unsigned long statusType, in wstring status); */
    #[inline]
    pub unsafe fn setStatus(&self, statusType: libc::uint32_t, status: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setStatus)(self as *const _, statusType, status) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIWebBrowser webBrowser; */
    #[inline]
    pub unsafe fn get_webBrowser(&self, ) -> Result<Option<RefPtr<nsIWebBrowser>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_webBrowser)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_webBrowser(&self, aWebBrowser: Option<&nsIWebBrowser>) -> Result<(), nsresult> {

        match ((*self.vtable).set_webBrowser)(self as *const _, aWebBrowser.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long chromeFlags; */
    #[inline]
    pub unsafe fn get_chromeFlags(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_chromeFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_chromeFlags(&self, aChromeFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_chromeFlags)(self as *const _, aChromeFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void destroyBrowserWindow (); */
    #[inline]
    pub unsafe fn destroyBrowserWindow(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).destroyBrowserWindow)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sizeBrowserTo (in long aCX, in long aCY); */
    #[inline]
    pub unsafe fn sizeBrowserTo(&self, aCX: libc::int32_t, aCY: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).sizeBrowserTo)(self as *const _, aCX, aCY) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showAsModal (); */
    #[inline]
    pub unsafe fn showAsModal(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).showAsModal)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isWindowModal (); */
    #[inline]
    pub unsafe fn isWindowModal(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isWindowModal)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void exitModalEventLoop (in nsresult aStatus); */
    #[inline]
    pub unsafe fn exitModalEventLoop(&self, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).exitModalEventLoop)(self as *const _, aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


