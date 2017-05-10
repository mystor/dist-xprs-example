//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISHistoryListener.idl
//


#[repr(C)]
pub struct nsISHistoryListener {
    vtable: *const nsISHistoryListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISHistoryListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x125c0833, 0x746a, 0x400e,
            [0x9b, 0x89, 0xd2, 0xd1, 0x85, 0x45, 0xc0, 0x8a])
    }
}

unsafe impl RefCounted for nsISHistoryListener {
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
pub trait nsISHistoryListenerCoerce {
    fn coerce_from(v: &nsISHistoryListener) -> &Self;
}

impl nsISHistoryListenerCoerce for nsISHistoryListener {
    #[inline]
    fn coerce_from(v: &nsISHistoryListener) -> &Self {
        v
    }
}

impl nsISHistoryListener {
    #[inline]
    pub fn coerce<T: nsISHistoryListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISHistoryListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISHistoryListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISHistoryListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISHistoryListenerVTable {
    pub __base: nsISupportsVTable,

    /* void OnHistoryNewEntry (in nsIURI aNewURI, in long aOldIndex); */
    pub OnHistoryNewEntry: unsafe extern "C" fn (this: *const nsISHistoryListener, aNewURI: *const nsIURI, aOldIndex: libc::int32_t) -> nsresult,

    /* boolean OnHistoryGoBack (in nsIURI aBackURI); */
    pub OnHistoryGoBack: unsafe extern "C" fn (this: *const nsISHistoryListener, aBackURI: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* boolean OnHistoryGoForward (in nsIURI aForwardURI); */
    pub OnHistoryGoForward: unsafe extern "C" fn (this: *const nsISHistoryListener, aForwardURI: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* boolean OnHistoryReload (in nsIURI aReloadURI, in unsigned long aReloadFlags); */
    pub OnHistoryReload: unsafe extern "C" fn (this: *const nsISHistoryListener, aReloadURI: *const nsIURI, aReloadFlags: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* boolean OnHistoryGotoIndex (in long aIndex, in nsIURI aGotoURI); */
    pub OnHistoryGotoIndex: unsafe extern "C" fn (this: *const nsISHistoryListener, aIndex: libc::int32_t, aGotoURI: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* boolean OnHistoryPurge (in long aNumEntries); */
    pub OnHistoryPurge: unsafe extern "C" fn (this: *const nsISHistoryListener, aNumEntries: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* void OnHistoryReplaceEntry (in long aIndex); */
    pub OnHistoryReplaceEntry: unsafe extern "C" fn (this: *const nsISHistoryListener, aIndex: libc::int32_t) -> nsresult,

    /* void OnLengthChanged (in long aCount); */
    pub OnLengthChanged: unsafe extern "C" fn (this: *const nsISHistoryListener, aCount: libc::int32_t) -> nsresult,

    /* void OnIndexChanged (in long aIndex); */
    pub OnIndexChanged: unsafe extern "C" fn (this: *const nsISHistoryListener, aIndex: libc::int32_t) -> nsresult,

}


impl nsISHistoryListener {
    /* void OnHistoryNewEntry (in nsIURI aNewURI, in long aOldIndex); */
    #[inline]
    pub unsafe fn OnHistoryNewEntry(&self, aNewURI: Option<&nsIURI>, aOldIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).OnHistoryNewEntry)(self as *const _, aNewURI.map_or(::std::ptr::null(), |x| x as *const _), aOldIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean OnHistoryGoBack (in nsIURI aBackURI); */
    #[inline]
    pub unsafe fn OnHistoryGoBack(&self, aBackURI: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).OnHistoryGoBack)(self as *const _, aBackURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean OnHistoryGoForward (in nsIURI aForwardURI); */
    #[inline]
    pub unsafe fn OnHistoryGoForward(&self, aForwardURI: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).OnHistoryGoForward)(self as *const _, aForwardURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean OnHistoryReload (in nsIURI aReloadURI, in unsigned long aReloadFlags); */
    #[inline]
    pub unsafe fn OnHistoryReload(&self, aReloadURI: Option<&nsIURI>, aReloadFlags: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).OnHistoryReload)(self as *const _, aReloadURI.map_or(::std::ptr::null(), |x| x as *const _), aReloadFlags, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean OnHistoryGotoIndex (in long aIndex, in nsIURI aGotoURI); */
    #[inline]
    pub unsafe fn OnHistoryGotoIndex(&self, aIndex: libc::int32_t, aGotoURI: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).OnHistoryGotoIndex)(self as *const _, aIndex, aGotoURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean OnHistoryPurge (in long aNumEntries); */
    #[inline]
    pub unsafe fn OnHistoryPurge(&self, aNumEntries: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).OnHistoryPurge)(self as *const _, aNumEntries, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void OnHistoryReplaceEntry (in long aIndex); */
    #[inline]
    pub unsafe fn OnHistoryReplaceEntry(&self, aIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).OnHistoryReplaceEntry)(self as *const _, aIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void OnLengthChanged (in long aCount); */
    #[inline]
    pub unsafe fn OnLengthChanged(&self, aCount: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).OnLengthChanged)(self as *const _, aCount) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void OnIndexChanged (in long aIndex); */
    #[inline]
    pub unsafe fn OnIndexChanged(&self, aIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).OnIndexChanged)(self as *const _, aIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


