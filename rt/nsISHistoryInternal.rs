//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISHistoryInternal.idl
//


#[repr(C)]
pub struct nsISHistoryInternal {
    vtable: *const nsISHistoryInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISHistoryInternal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3dfb2f54, 0x378d, 0x4d3c,
            [0xa9, 0xf9, 0x95, 0xdd, 0x26, 0x73, 0x24, 0x8c])
    }
}

unsafe impl RefCounted for nsISHistoryInternal {
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
pub trait nsISHistoryInternalCoerce {
    fn coerce_from(v: &nsISHistoryInternal) -> &Self;
}

impl nsISHistoryInternalCoerce for nsISHistoryInternal {
    #[inline]
    fn coerce_from(v: &nsISHistoryInternal) -> &Self {
        v
    }
}

impl nsISHistoryInternal {
    #[inline]
    pub fn coerce<T: nsISHistoryInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISHistoryInternal {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISHistoryInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISHistoryInternal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISHistoryInternalVTable {
    pub __base: nsISupportsVTable,

    /* void addEntry (in nsISHEntry aEntry, in boolean aPersist); */
    pub addEntry: unsafe extern "C" fn (this: *const nsISHistoryInternal, aEntry: *const nsISHEntry, aPersist: bool) -> nsresult,

    /* readonly attribute nsISHTransaction rootTransaction; */
    pub get_rootTransaction: unsafe extern "C" fn (this: *const nsISHistoryInternal, aRootTransaction: *mut *const nsISHTransaction) -> nsresult,

    /* void setRootDocShell (in nsIDocShell rootDocShell); */
    pub setRootDocShell: unsafe extern "C" fn (this: *const nsISHistoryInternal, rootDocShell: *const nsIDocShell) -> nsresult,

    /* void updateIndex (); */
    pub updateIndex: unsafe extern "C" fn (this: *const nsISHistoryInternal) -> nsresult,

    /* void replaceEntry (in long aIndex, in nsISHEntry aReplaceEntry); */
    pub replaceEntry: unsafe extern "C" fn (this: *const nsISHistoryInternal, aIndex: libc::int32_t, aReplaceEntry: *const nsISHEntry) -> nsresult,

    /* boolean notifyOnHistoryReload (in nsIURI aReloadURI, in unsigned long aReloadFlags); */
    pub notifyOnHistoryReload: unsafe extern "C" fn (this: *const nsISHistoryInternal, aReloadURI: *const nsIURI, aReloadFlags: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* void evictOutOfRangeContentViewers (in long aIndex); */
    pub evictOutOfRangeContentViewers: unsafe extern "C" fn (this: *const nsISHistoryInternal, aIndex: libc::int32_t) -> nsresult,

    /* void evictExpiredContentViewerForEntry (in nsIBFCacheEntry aEntry); */
    pub evictExpiredContentViewerForEntry: unsafe extern "C" fn (this: *const nsISHistoryInternal, aEntry: *const nsIBFCacheEntry) -> nsresult,

    /* void evictAllContentViewers (); */
    pub evictAllContentViewers: unsafe extern "C" fn (this: *const nsISHistoryInternal) -> nsresult,

    /* void addToExpirationTracker (in nsIBFCacheEntry aEntry); */
    pub addToExpirationTracker: unsafe extern "C" fn (this: *const nsISHistoryInternal, aEntry: *const nsIBFCacheEntry) -> nsresult,

    /* void removeFromExpirationTracker (in nsIBFCacheEntry aEntry); */
    pub removeFromExpirationTracker: unsafe extern "C" fn (this: *const nsISHistoryInternal, aEntry: *const nsIBFCacheEntry) -> nsresult,

    /* [noscript,notxpcom] void RemoveDynEntries (in long aIndex, in nsISHContainer aContainer); */
    pub RemoveDynEntries: unsafe extern "C" fn (this: *const nsISHistoryInternal, aIndex: libc::int32_t, aContainer: *const nsISHContainer) -> libc::c_void,

    /* [noscript,notxpcom] void RemoveEntries (in nsDocshellIDArray aIDs, in long aStartIndex); */
    /// Unable to call function as its signature contains a non-rust type
    pub RemoveEntries: *const ::libc::c_void,

}


impl nsISHistoryInternal {
    /* void addEntry (in nsISHEntry aEntry, in boolean aPersist); */
    #[inline]
    pub unsafe fn addEntry(&self, aEntry: Option<&nsISHEntry>, aPersist: bool) -> Result<(), nsresult> {

        match ((*self.vtable).addEntry)(self as *const _, aEntry.map_or(::std::ptr::null(), |x| x as *const _), aPersist) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsISHTransaction rootTransaction; */
    #[inline]
    pub unsafe fn get_rootTransaction(&self, ) -> Result<Option<RefPtr<nsISHTransaction>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_rootTransaction)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setRootDocShell (in nsIDocShell rootDocShell); */
    #[inline]
    pub unsafe fn setRootDocShell(&self, rootDocShell: Option<&nsIDocShell>) -> Result<(), nsresult> {

        match ((*self.vtable).setRootDocShell)(self as *const _, rootDocShell.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void updateIndex (); */
    #[inline]
    pub unsafe fn updateIndex(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).updateIndex)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void replaceEntry (in long aIndex, in nsISHEntry aReplaceEntry); */
    #[inline]
    pub unsafe fn replaceEntry(&self, aIndex: libc::int32_t, aReplaceEntry: Option<&nsISHEntry>) -> Result<(), nsresult> {

        match ((*self.vtable).replaceEntry)(self as *const _, aIndex, aReplaceEntry.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean notifyOnHistoryReload (in nsIURI aReloadURI, in unsigned long aReloadFlags); */
    #[inline]
    pub unsafe fn notifyOnHistoryReload(&self, aReloadURI: Option<&nsIURI>, aReloadFlags: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).notifyOnHistoryReload)(self as *const _, aReloadURI.map_or(::std::ptr::null(), |x| x as *const _), aReloadFlags, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void evictOutOfRangeContentViewers (in long aIndex); */
    #[inline]
    pub unsafe fn evictOutOfRangeContentViewers(&self, aIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).evictOutOfRangeContentViewers)(self as *const _, aIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void evictExpiredContentViewerForEntry (in nsIBFCacheEntry aEntry); */
    #[inline]
    pub unsafe fn evictExpiredContentViewerForEntry(&self, aEntry: Option<&nsIBFCacheEntry>) -> Result<(), nsresult> {

        match ((*self.vtable).evictExpiredContentViewerForEntry)(self as *const _, aEntry.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void evictAllContentViewers (); */
    #[inline]
    pub unsafe fn evictAllContentViewers(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).evictAllContentViewers)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addToExpirationTracker (in nsIBFCacheEntry aEntry); */
    #[inline]
    pub unsafe fn addToExpirationTracker(&self, aEntry: Option<&nsIBFCacheEntry>) -> Result<(), nsresult> {

        match ((*self.vtable).addToExpirationTracker)(self as *const _, aEntry.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeFromExpirationTracker (in nsIBFCacheEntry aEntry); */
    #[inline]
    pub unsafe fn removeFromExpirationTracker(&self, aEntry: Option<&nsIBFCacheEntry>) -> Result<(), nsresult> {

        match ((*self.vtable).removeFromExpirationTracker)(self as *const _, aEntry.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,notxpcom] void RemoveDynEntries (in long aIndex, in nsISHContainer aContainer); */
    #[inline]
    pub unsafe fn RemoveDynEntries(&self, aIndex: libc::int32_t, aContainer: Option<&nsISHContainer>) -> () {

        let _retval = ((*self.vtable).RemoveDynEntries)(self as *const _, aIndex, aContainer.map_or(::std::ptr::null(), |x| x as *const _));
        ()
    }

    /* [noscript,notxpcom] void RemoveEntries (in nsDocshellIDArray aIDs, in long aStartIndex); */


}


