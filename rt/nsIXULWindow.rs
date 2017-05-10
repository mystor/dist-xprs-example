//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULWindow.idl
//


pub mod nsIXULWindow_consts {
    pub const lowestZ: i64 = 0;
    pub const loweredZ: i64 = 4;
    pub const normalZ: i64 = 5;
    pub const raisedZ: i64 = 6;
    pub const highestZ: i64 = 9;
}


#[repr(C)]
pub struct nsIXULWindow {
    vtable: *const nsIXULWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXULWindow {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd6d7a014, 0xe28d, 0x4c9d,
            [0x87, 0x27, 0x1c, 0xf6, 0xd8, 0x70, 0x61, 0x9b])
    }
}

unsafe impl RefCounted for nsIXULWindow {
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
pub trait nsIXULWindowCoerce {
    fn coerce_from(v: &nsIXULWindow) -> &Self;
}

impl nsIXULWindowCoerce for nsIXULWindow {
    #[inline]
    fn coerce_from(v: &nsIXULWindow) -> &Self {
        v
    }
}

impl nsIXULWindow {
    #[inline]
    pub fn coerce<T: nsIXULWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXULWindow {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXULWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULWindow) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXULWindowVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDocShell docShell; */
    pub get_docShell: unsafe extern "C" fn (this: *const nsIXULWindow, aDocShell: *mut *const nsIDocShell) -> nsresult,

    /* attribute boolean intrinsicallySized; */
    pub get_intrinsicallySized: unsafe extern "C" fn (this: *const nsIXULWindow, aIntrinsicallySized: *mut bool) -> nsresult,
    pub set_intrinsicallySized: unsafe extern "C" fn (this: *const nsIXULWindow, aIntrinsicallySized: bool) -> nsresult,

    /* readonly attribute nsIDocShellTreeItem primaryContentShell; */
    pub get_primaryContentShell: unsafe extern "C" fn (this: *const nsIXULWindow, aPrimaryContentShell: *mut *const nsIDocShellTreeItem) -> nsresult,

    /* readonly attribute nsITabParent primaryTabParent; */
    pub get_primaryTabParent: unsafe extern "C" fn (this: *const nsIXULWindow, aPrimaryTabParent: *mut *const nsITabParent) -> nsresult,

    /* void tabParentAdded (in nsITabParent aTab, in boolean aPrimary); */
    pub tabParentAdded: unsafe extern "C" fn (this: *const nsIXULWindow, aTab: *const nsITabParent, aPrimary: bool) -> nsresult,

    /* void tabParentRemoved (in nsITabParent aTab); */
    pub tabParentRemoved: unsafe extern "C" fn (this: *const nsIXULWindow, aTab: *const nsITabParent) -> nsresult,

    /* [noscript,notxpcom] LiveResizeListenerArray getLiveResizeListeners (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getLiveResizeListeners: *const ::libc::c_void,

    /* void addChildWindow (in nsIXULWindow aChild); */
    pub addChildWindow: unsafe extern "C" fn (this: *const nsIXULWindow, aChild: *const nsIXULWindow) -> nsresult,

    /* void removeChildWindow (in nsIXULWindow aChild); */
    pub removeChildWindow: unsafe extern "C" fn (this: *const nsIXULWindow, aChild: *const nsIXULWindow) -> nsresult,

    /* void center (in nsIXULWindow aRelative, in boolean aScreen, in boolean aAlert); */
    pub center: unsafe extern "C" fn (this: *const nsIXULWindow, aRelative: *const nsIXULWindow, aScreen: bool, aAlert: bool) -> nsresult,

    /* void showModal (); */
    pub showModal: unsafe extern "C" fn (this: *const nsIXULWindow) -> nsresult,

    /* attribute unsigned long zLevel; */
    pub get_zLevel: unsafe extern "C" fn (this: *const nsIXULWindow, aZLevel: *mut libc::uint32_t) -> nsresult,
    pub set_zLevel: unsafe extern "C" fn (this: *const nsIXULWindow, aZLevel: libc::uint32_t) -> nsresult,

    /* attribute uint32_t chromeFlags; */
    pub get_chromeFlags: unsafe extern "C" fn (this: *const nsIXULWindow, aChromeFlags: *mut uint32_t) -> nsresult,
    pub set_chromeFlags: unsafe extern "C" fn (this: *const nsIXULWindow, aChromeFlags: uint32_t) -> nsresult,

    /* void assumeChromeFlagsAreFrozen (); */
    pub assumeChromeFlagsAreFrozen: unsafe extern "C" fn (this: *const nsIXULWindow) -> nsresult,

    /* nsIXULWindow createNewWindow (in int32_t aChromeFlags, in nsITabParent aOpeningTab, in mozIDOMWindowProxy aOpener, in unsigned long long aNextTabParentId); */
    pub createNewWindow: unsafe extern "C" fn (this: *const nsIXULWindow, aChromeFlags: int32_t, aOpeningTab: *const nsITabParent, aOpener: *const mozIDOMWindowProxy, aNextTabParentId: libc::uint64_t, _retval: *mut *const nsIXULWindow) -> nsresult,

    /* attribute nsIXULBrowserWindow XULBrowserWindow; */
    pub get_XULBrowserWindow: unsafe extern "C" fn (this: *const nsIXULWindow, aXULBrowserWindow: *mut *const nsIXULBrowserWindow) -> nsresult,
    pub set_XULBrowserWindow: unsafe extern "C" fn (this: *const nsIXULWindow, aXULBrowserWindow: *const nsIXULBrowserWindow) -> nsresult,

    /* [noscript] void applyChromeFlags (); */
    pub applyChromeFlags: unsafe extern "C" fn (this: *const nsIXULWindow) -> nsresult,

    /* [noscript,notxpcom] void sizeShellToWithLimit (in int32_t aDesiredWidth, in int32_t aDesiredHeight, in int32_t shellItemWidth, in int32_t shellItemHeight); */
    pub sizeShellToWithLimit: unsafe extern "C" fn (this: *const nsIXULWindow, aDesiredWidth: int32_t, aDesiredHeight: int32_t, shellItemWidth: int32_t, shellItemHeight: int32_t) -> libc::c_void,

    /* [noscript] readonly attribute unsigned long long nextTabParentId; */
    pub get_nextTabParentId: unsafe extern "C" fn (this: *const nsIXULWindow, aNextTabParentId: *mut libc::uint64_t) -> nsresult,

}


impl nsIXULWindow {
    /* readonly attribute nsIDocShell docShell; */
    #[inline]
    pub unsafe fn get_docShell(&self, ) -> Result<Option<RefPtr<nsIDocShell>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_docShell)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute boolean intrinsicallySized; */
    #[inline]
    pub unsafe fn get_intrinsicallySized(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_intrinsicallySized)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_intrinsicallySized(&self, aIntrinsicallySized: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_intrinsicallySized)(self as *const _, aIntrinsicallySized) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDocShellTreeItem primaryContentShell; */
    #[inline]
    pub unsafe fn get_primaryContentShell(&self, ) -> Result<Option<RefPtr<nsIDocShellTreeItem>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_primaryContentShell)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsITabParent primaryTabParent; */
    #[inline]
    pub unsafe fn get_primaryTabParent(&self, ) -> Result<Option<RefPtr<nsITabParent>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_primaryTabParent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void tabParentAdded (in nsITabParent aTab, in boolean aPrimary); */
    #[inline]
    pub unsafe fn tabParentAdded(&self, aTab: Option<&nsITabParent>, aPrimary: bool) -> Result<(), nsresult> {

        match ((*self.vtable).tabParentAdded)(self as *const _, aTab.map_or(::std::ptr::null(), |x| x as *const _), aPrimary) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void tabParentRemoved (in nsITabParent aTab); */
    #[inline]
    pub unsafe fn tabParentRemoved(&self, aTab: Option<&nsITabParent>) -> Result<(), nsresult> {

        match ((*self.vtable).tabParentRemoved)(self as *const _, aTab.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,notxpcom] LiveResizeListenerArray getLiveResizeListeners (); */


    /* void addChildWindow (in nsIXULWindow aChild); */
    #[inline]
    pub unsafe fn addChildWindow(&self, aChild: Option<&nsIXULWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).addChildWindow)(self as *const _, aChild.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeChildWindow (in nsIXULWindow aChild); */
    #[inline]
    pub unsafe fn removeChildWindow(&self, aChild: Option<&nsIXULWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).removeChildWindow)(self as *const _, aChild.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void center (in nsIXULWindow aRelative, in boolean aScreen, in boolean aAlert); */
    #[inline]
    pub unsafe fn center(&self, aRelative: Option<&nsIXULWindow>, aScreen: bool, aAlert: bool) -> Result<(), nsresult> {

        match ((*self.vtable).center)(self as *const _, aRelative.map_or(::std::ptr::null(), |x| x as *const _), aScreen, aAlert) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showModal (); */
    #[inline]
    pub unsafe fn showModal(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).showModal)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long zLevel; */
    #[inline]
    pub unsafe fn get_zLevel(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_zLevel)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_zLevel(&self, aZLevel: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_zLevel)(self as *const _, aZLevel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute uint32_t chromeFlags; */
    #[inline]
    pub unsafe fn get_chromeFlags(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_chromeFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_chromeFlags(&self, aChromeFlags: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_chromeFlags)(self as *const _, aChromeFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void assumeChromeFlagsAreFrozen (); */
    #[inline]
    pub unsafe fn assumeChromeFlagsAreFrozen(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).assumeChromeFlagsAreFrozen)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIXULWindow createNewWindow (in int32_t aChromeFlags, in nsITabParent aOpeningTab, in mozIDOMWindowProxy aOpener, in unsigned long long aNextTabParentId); */
    #[inline]
    pub unsafe fn createNewWindow(&self, aChromeFlags: int32_t, aOpeningTab: Option<&nsITabParent>, aOpener: Option<&mozIDOMWindowProxy>, aNextTabParentId: libc::uint64_t) -> Result<Option<RefPtr<nsIXULWindow>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createNewWindow)(self as *const _, aChromeFlags, aOpeningTab.map_or(::std::ptr::null(), |x| x as *const _), aOpener.map_or(::std::ptr::null(), |x| x as *const _), aNextTabParentId, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsIXULBrowserWindow XULBrowserWindow; */
    #[inline]
    pub unsafe fn get_XULBrowserWindow(&self, ) -> Result<Option<RefPtr<nsIXULBrowserWindow>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_XULBrowserWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_XULBrowserWindow(&self, aXULBrowserWindow: Option<&nsIXULBrowserWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).set_XULBrowserWindow)(self as *const _, aXULBrowserWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void applyChromeFlags (); */
    #[inline]
    pub unsafe fn applyChromeFlags(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).applyChromeFlags)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,notxpcom] void sizeShellToWithLimit (in int32_t aDesiredWidth, in int32_t aDesiredHeight, in int32_t shellItemWidth, in int32_t shellItemHeight); */
    #[inline]
    pub unsafe fn sizeShellToWithLimit(&self, aDesiredWidth: int32_t, aDesiredHeight: int32_t, shellItemWidth: int32_t, shellItemHeight: int32_t) -> () {

        let _retval = ((*self.vtable).sizeShellToWithLimit)(self as *const _, aDesiredWidth, aDesiredHeight, shellItemWidth, shellItemHeight);
        ()
    }

    /* [noscript] readonly attribute unsigned long long nextTabParentId; */
    #[inline]
    pub unsafe fn get_nextTabParentId(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_nextTabParentId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


