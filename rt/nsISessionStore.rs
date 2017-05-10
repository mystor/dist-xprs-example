//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISessionStore.idl
//


#[repr(C)]
pub struct nsISessionStore {
    vtable: *const nsISessionStoreVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISessionStore {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4580f5eb, 0x693d, 0x423d,
            [0xb0, 0xce, 0x2c, 0xb2, 0x0a, 0x96, 0x2e, 0x4d])
    }
}

unsafe impl RefCounted for nsISessionStore {
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
pub trait nsISessionStoreCoerce {
    fn coerce_from(v: &nsISessionStore) -> &Self;
}

impl nsISessionStoreCoerce for nsISessionStore {
    #[inline]
    fn coerce_from(v: &nsISessionStore) -> &Self {
        v
    }
}

impl nsISessionStore {
    #[inline]
    pub fn coerce<T: nsISessionStoreCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISessionStore {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISessionStoreCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISessionStore) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISessionStoreVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean canRestoreLastSession; */
    pub get_canRestoreLastSession: unsafe extern "C" fn (this: *const nsISessionStore, aCanRestoreLastSession: *mut bool) -> nsresult,
    pub set_canRestoreLastSession: unsafe extern "C" fn (this: *const nsISessionStore, aCanRestoreLastSession: bool) -> nsresult,

    /* void restoreLastSession (); */
    pub restoreLastSession: unsafe extern "C" fn (this: *const nsISessionStore) -> nsresult,

    /* AString getBrowserState (); */
    pub getBrowserState: unsafe extern "C" fn (this: *const nsISessionStore, _retval: *mut nsAString) -> nsresult,

    /* void setBrowserState (in AString aState); */
    pub setBrowserState: unsafe extern "C" fn (this: *const nsISessionStore, aState: *const nsAString) -> nsresult,

    /* AString getWindowState (in nsIDOMWindow aWindow); */
    pub getWindowState: unsafe extern "C" fn (this: *const nsISessionStore, aWindow: *const nsIDOMWindow, _retval: *mut nsAString) -> nsresult,

    /* void setWindowState (in nsIDOMWindow aWindow, in AString aState, in boolean aOverwrite); */
    pub setWindowState: unsafe extern "C" fn (this: *const nsISessionStore, aWindow: *const nsIDOMWindow, aState: *const nsAString, aOverwrite: bool) -> nsresult,

    /* AString getTabState (in nsIDOMNode aTab); */
    pub getTabState: unsafe extern "C" fn (this: *const nsISessionStore, aTab: *const nsIDOMNode, _retval: *mut nsAString) -> nsresult,

    /* void setTabState (in nsIDOMNode aTab, in AString aState); */
    pub setTabState: unsafe extern "C" fn (this: *const nsISessionStore, aTab: *const nsIDOMNode, aState: *const nsAString) -> nsresult,

    /* nsIDOMNode duplicateTab (in nsIDOMWindow aWindow, in nsIDOMNode aTab, [optional] in long aDelta); */
    pub duplicateTab: unsafe extern "C" fn (this: *const nsISessionStore, aWindow: *const nsIDOMWindow, aTab: *const nsIDOMNode, aDelta: libc::int32_t, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* unsigned long getClosedTabCount (in nsIDOMWindow aWindow); */
    pub getClosedTabCount: unsafe extern "C" fn (this: *const nsISessionStore, aWindow: *const nsIDOMWindow, _retval: *mut libc::uint32_t) -> nsresult,

    /* AString getClosedTabData (in nsIDOMWindow aWindow); */
    pub getClosedTabData: unsafe extern "C" fn (this: *const nsISessionStore, aWindow: *const nsIDOMWindow, _retval: *mut nsAString) -> nsresult,

    /* nsIDOMNode undoCloseTab (in nsIDOMWindow aWindow, in unsigned long aIndex); */
    pub undoCloseTab: unsafe extern "C" fn (this: *const nsISessionStore, aWindow: *const nsIDOMWindow, aIndex: libc::uint32_t, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode forgetClosedTab (in nsIDOMWindow aWindow, in unsigned long aIndex); */
    pub forgetClosedTab: unsafe extern "C" fn (this: *const nsISessionStore, aWindow: *const nsIDOMWindow, aIndex: libc::uint32_t, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* unsigned long getClosedWindowCount (); */
    pub getClosedWindowCount: unsafe extern "C" fn (this: *const nsISessionStore, _retval: *mut libc::uint32_t) -> nsresult,

    /* AString getClosedWindowData (); */
    pub getClosedWindowData: unsafe extern "C" fn (this: *const nsISessionStore, _retval: *mut nsAString) -> nsresult,

    /* nsIDOMWindow undoCloseWindow (in unsigned long aIndex); */
    pub undoCloseWindow: unsafe extern "C" fn (this: *const nsISessionStore, aIndex: libc::uint32_t, _retval: *mut *const nsIDOMWindow) -> nsresult,

    /* nsIDOMNode forgetClosedWindow (in unsigned long aIndex); */
    pub forgetClosedWindow: unsafe extern "C" fn (this: *const nsISessionStore, aIndex: libc::uint32_t, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* AString getWindowValue (in nsIDOMWindow aWindow, in AString aKey); */
    pub getWindowValue: unsafe extern "C" fn (this: *const nsISessionStore, aWindow: *const nsIDOMWindow, aKey: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* void setWindowValue (in nsIDOMWindow aWindow, in AString aKey, in jsval aStringValue); */
    /// Unable to call function as its signature contains a non-rust type
    pub setWindowValue: *const ::libc::c_void,

    /* void deleteWindowValue (in nsIDOMWindow aWindow, in AString aKey); */
    pub deleteWindowValue: unsafe extern "C" fn (this: *const nsISessionStore, aWindow: *const nsIDOMWindow, aKey: *const nsAString) -> nsresult,

    /* AString getTabValue (in nsIDOMNode aTab, in AString aKey); */
    pub getTabValue: unsafe extern "C" fn (this: *const nsISessionStore, aTab: *const nsIDOMNode, aKey: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* void setTabValue (in nsIDOMNode aTab, in AString aKey, in jsval aStringValue); */
    /// Unable to call function as its signature contains a non-rust type
    pub setTabValue: *const ::libc::c_void,

    /* void deleteTabValue (in nsIDOMNode aTab, in AString aKey); */
    pub deleteTabValue: unsafe extern "C" fn (this: *const nsISessionStore, aTab: *const nsIDOMNode, aKey: *const nsAString) -> nsresult,

    /* AString getGlobalValue (in AString aKey); */
    pub getGlobalValue: unsafe extern "C" fn (this: *const nsISessionStore, aKey: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* void setGlobalValue (in AString aKey, in jsval aStringValue); */
    /// Unable to call function as its signature contains a non-rust type
    pub setGlobalValue: *const ::libc::c_void,

    /* void deleteGlobalValue (in AString aKey); */
    pub deleteGlobalValue: unsafe extern "C" fn (this: *const nsISessionStore, aKey: *const nsAString) -> nsresult,

    /* void persistTabAttribute (in AString aName); */
    pub persistTabAttribute: unsafe extern "C" fn (this: *const nsISessionStore, aName: *const nsAString) -> nsresult,

}


impl nsISessionStore {
    /* attribute boolean canRestoreLastSession; */
    #[inline]
    pub unsafe fn get_canRestoreLastSession(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canRestoreLastSession)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_canRestoreLastSession(&self, aCanRestoreLastSession: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_canRestoreLastSession)(self as *const _, aCanRestoreLastSession) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void restoreLastSession (); */
    #[inline]
    pub unsafe fn restoreLastSession(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).restoreLastSession)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString getBrowserState (); */
    #[inline]
    pub unsafe fn getBrowserState(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getBrowserState)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setBrowserState (in AString aState); */
    #[inline]
    pub unsafe fn setBrowserState(&self, aState: &[u16]) -> Result<(), nsresult> {
        let aState = nsString::from(aState);
        match ((*self.vtable).setBrowserState)(self as *const _, &*aState) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString getWindowState (in nsIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn getWindowState(&self, aWindow: Option<&nsIDOMWindow>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getWindowState)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setWindowState (in nsIDOMWindow aWindow, in AString aState, in boolean aOverwrite); */
    #[inline]
    pub unsafe fn setWindowState(&self, aWindow: Option<&nsIDOMWindow>, aState: &[u16], aOverwrite: bool) -> Result<(), nsresult> {
        let aState = nsString::from(aState);
        match ((*self.vtable).setWindowState)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), &*aState, aOverwrite) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString getTabState (in nsIDOMNode aTab); */
    #[inline]
    pub unsafe fn getTabState(&self, aTab: Option<&nsIDOMNode>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getTabState)(self as *const _, aTab.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setTabState (in nsIDOMNode aTab, in AString aState); */
    #[inline]
    pub unsafe fn setTabState(&self, aTab: Option<&nsIDOMNode>, aState: &[u16]) -> Result<(), nsresult> {
        let aState = nsString::from(aState);
        match ((*self.vtable).setTabState)(self as *const _, aTab.map_or(::std::ptr::null(), |x| x as *const _), &*aState) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMNode duplicateTab (in nsIDOMWindow aWindow, in nsIDOMNode aTab, [optional] in long aDelta); */
    #[inline]
    pub unsafe fn duplicateTab(&self, aWindow: Option<&nsIDOMWindow>, aTab: Option<&nsIDOMNode>, aDelta: libc::int32_t) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).duplicateTab)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aTab.map_or(::std::ptr::null(), |x| x as *const _), aDelta, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* unsigned long getClosedTabCount (in nsIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn getClosedTabCount(&self, aWindow: Option<&nsIDOMWindow>) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getClosedTabCount)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getClosedTabData (in nsIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn getClosedTabData(&self, aWindow: Option<&nsIDOMWindow>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getClosedTabData)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMNode undoCloseTab (in nsIDOMWindow aWindow, in unsigned long aIndex); */
    #[inline]
    pub unsafe fn undoCloseTab(&self, aWindow: Option<&nsIDOMWindow>, aIndex: libc::uint32_t) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).undoCloseTab)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNode forgetClosedTab (in nsIDOMWindow aWindow, in unsigned long aIndex); */
    #[inline]
    pub unsafe fn forgetClosedTab(&self, aWindow: Option<&nsIDOMWindow>, aIndex: libc::uint32_t) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).forgetClosedTab)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* unsigned long getClosedWindowCount (); */
    #[inline]
    pub unsafe fn getClosedWindowCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getClosedWindowCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getClosedWindowData (); */
    #[inline]
    pub unsafe fn getClosedWindowData(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getClosedWindowData)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMWindow undoCloseWindow (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn undoCloseWindow(&self, aIndex: libc::uint32_t) -> Result<Option<RefPtr<nsIDOMWindow>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).undoCloseWindow)(self as *const _, aIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNode forgetClosedWindow (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn forgetClosedWindow(&self, aIndex: libc::uint32_t) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).forgetClosedWindow)(self as *const _, aIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* AString getWindowValue (in nsIDOMWindow aWindow, in AString aKey); */
    #[inline]
    pub unsafe fn getWindowValue(&self, aWindow: Option<&nsIDOMWindow>, aKey: &[u16]) -> Result<nsString, nsresult> {
        let aKey = nsString::from(aKey);
        let mut _retval = nsString::new();
        match ((*self.vtable).getWindowValue)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), &*aKey, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setWindowValue (in nsIDOMWindow aWindow, in AString aKey, in jsval aStringValue); */


    /* void deleteWindowValue (in nsIDOMWindow aWindow, in AString aKey); */
    #[inline]
    pub unsafe fn deleteWindowValue(&self, aWindow: Option<&nsIDOMWindow>, aKey: &[u16]) -> Result<(), nsresult> {
        let aKey = nsString::from(aKey);
        match ((*self.vtable).deleteWindowValue)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), &*aKey) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString getTabValue (in nsIDOMNode aTab, in AString aKey); */
    #[inline]
    pub unsafe fn getTabValue(&self, aTab: Option<&nsIDOMNode>, aKey: &[u16]) -> Result<nsString, nsresult> {
        let aKey = nsString::from(aKey);
        let mut _retval = nsString::new();
        match ((*self.vtable).getTabValue)(self as *const _, aTab.map_or(::std::ptr::null(), |x| x as *const _), &*aKey, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setTabValue (in nsIDOMNode aTab, in AString aKey, in jsval aStringValue); */


    /* void deleteTabValue (in nsIDOMNode aTab, in AString aKey); */
    #[inline]
    pub unsafe fn deleteTabValue(&self, aTab: Option<&nsIDOMNode>, aKey: &[u16]) -> Result<(), nsresult> {
        let aKey = nsString::from(aKey);
        match ((*self.vtable).deleteTabValue)(self as *const _, aTab.map_or(::std::ptr::null(), |x| x as *const _), &*aKey) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString getGlobalValue (in AString aKey); */
    #[inline]
    pub unsafe fn getGlobalValue(&self, aKey: &[u16]) -> Result<nsString, nsresult> {
        let aKey = nsString::from(aKey);
        let mut _retval = nsString::new();
        match ((*self.vtable).getGlobalValue)(self as *const _, &*aKey, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setGlobalValue (in AString aKey, in jsval aStringValue); */


    /* void deleteGlobalValue (in AString aKey); */
    #[inline]
    pub unsafe fn deleteGlobalValue(&self, aKey: &[u16]) -> Result<(), nsresult> {
        let aKey = nsString::from(aKey);
        match ((*self.vtable).deleteGlobalValue)(self as *const _, &*aKey) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void persistTabAttribute (in AString aName); */
    #[inline]
    pub unsafe fn persistTabAttribute(&self, aName: &[u16]) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).persistTabAttribute)(self as *const _, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


