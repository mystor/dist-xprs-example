//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISHistory.idl
//


pub mod nsISHistory_consts {
    pub const VIEWER_WINDOW: i64 = 3;
}


#[repr(C)]
pub struct nsISHistory {
    vtable: *const nsISHistoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISHistory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7b807041, 0xe60a, 0x4384,
            [0x93, 0x5f, 0xaf, 0x30, 0x61, 0xd8, 0xb8, 0x15])
    }
}

unsafe impl RefCounted for nsISHistory {
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
pub trait nsISHistoryCoerce {
    fn coerce_from(v: &nsISHistory) -> &Self;
}

impl nsISHistoryCoerce for nsISHistory {
    #[inline]
    fn coerce_from(v: &nsISHistory) -> &Self {
        v
    }
}

impl nsISHistory {
    #[inline]
    pub fn coerce<T: nsISHistoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISHistory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISHistoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISHistory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISHistoryVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute bool isPartial; */
    pub get_isPartial: unsafe extern "C" fn (this: *const nsISHistory, aIsPartial: *mut bool) -> nsresult,

    /* readonly attribute long count; */
    pub get_count: unsafe extern "C" fn (this: *const nsISHistory, aCount: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long globalCount; */
    pub get_globalCount: unsafe extern "C" fn (this: *const nsISHistory, aGlobalCount: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long globalIndexOffset; */
    pub get_globalIndexOffset: unsafe extern "C" fn (this: *const nsISHistory, aGlobalIndexOffset: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long index; */
    pub get_index: unsafe extern "C" fn (this: *const nsISHistory, aIndex: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long globalIndex; */
    pub get_globalIndex: unsafe extern "C" fn (this: *const nsISHistory, aGlobalIndex: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long requestedIndex; */
    pub get_requestedIndex: unsafe extern "C" fn (this: *const nsISHistory, aRequestedIndex: *mut libc::int32_t) -> nsresult,

    /* attribute long maxLength; */
    pub get_maxLength: unsafe extern "C" fn (this: *const nsISHistory, aMaxLength: *mut libc::int32_t) -> nsresult,
    pub set_maxLength: unsafe extern "C" fn (this: *const nsISHistory, aMaxLength: libc::int32_t) -> nsresult,

    /* nsISHEntry getEntryAtIndex (in long index, in boolean modifyIndex); */
    pub getEntryAtIndex: unsafe extern "C" fn (this: *const nsISHistory, index: libc::int32_t, modifyIndex: bool, _retval: *mut *const nsISHEntry) -> nsresult,

    /* void restoreToEntryAtIndex (in long index); */
    pub restoreToEntryAtIndex: unsafe extern "C" fn (this: *const nsISHistory, index: libc::int32_t) -> nsresult,

    /* void PurgeHistory (in long numEntries); */
    pub PurgeHistory: unsafe extern "C" fn (this: *const nsISHistory, numEntries: libc::int32_t) -> nsresult,

    /* void addSHistoryListener (in nsISHistoryListener aListener); */
    pub addSHistoryListener: unsafe extern "C" fn (this: *const nsISHistory, aListener: *const nsISHistoryListener) -> nsresult,

    /* void removeSHistoryListener (in nsISHistoryListener aListener); */
    pub removeSHistoryListener: unsafe extern "C" fn (this: *const nsISHistory, aListener: *const nsISHistoryListener) -> nsresult,

    /* void setPartialSHistoryListener (in nsIPartialSHistoryListener aListener); */
    pub setPartialSHistoryListener: unsafe extern "C" fn (this: *const nsISHistory, aListener: *const nsIPartialSHistoryListener) -> nsresult,

    /* readonly attribute nsISimpleEnumerator SHistoryEnumerator; */
    pub get_SHistoryEnumerator: unsafe extern "C" fn (this: *const nsISHistory, aSHistoryEnumerator: *mut *const nsISimpleEnumerator) -> nsresult,

    /* void reloadCurrentEntry (); */
    pub reloadCurrentEntry: unsafe extern "C" fn (this: *const nsISHistory) -> nsresult,

    /* long getIndexOfEntry (in nsISHEntry aEntry); */
    pub getIndexOfEntry: unsafe extern "C" fn (this: *const nsISHistory, aEntry: *const nsISHEntry, _retval: *mut libc::int32_t) -> nsresult,

    /* void onPartialSHistoryActive (in long globalLength, in long targetIndex); */
    pub onPartialSHistoryActive: unsafe extern "C" fn (this: *const nsISHistory, globalLength: libc::int32_t, targetIndex: libc::int32_t) -> nsresult,

    /* void onPartialSHistoryDeactive (); */
    pub onPartialSHistoryDeactive: unsafe extern "C" fn (this: *const nsISHistory) -> nsresult,

    /* void onAttachGroupedSHistory (in long offset); */
    pub onAttachGroupedSHistory: unsafe extern "C" fn (this: *const nsISHistory, offset: libc::int32_t) -> nsresult,

}


impl nsISHistory {
    /* readonly attribute bool isPartial; */
    #[inline]
    pub unsafe fn get_isPartial(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isPartial)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long count; */
    #[inline]
    pub unsafe fn get_count(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_count)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long globalCount; */
    #[inline]
    pub unsafe fn get_globalCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_globalCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long globalIndexOffset; */
    #[inline]
    pub unsafe fn get_globalIndexOffset(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_globalIndexOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long index; */
    #[inline]
    pub unsafe fn get_index(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_index)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long globalIndex; */
    #[inline]
    pub unsafe fn get_globalIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_globalIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long requestedIndex; */
    #[inline]
    pub unsafe fn get_requestedIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_requestedIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute long maxLength; */
    #[inline]
    pub unsafe fn get_maxLength(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_maxLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_maxLength(&self, aMaxLength: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_maxLength)(self as *const _, aMaxLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISHEntry getEntryAtIndex (in long index, in boolean modifyIndex); */
    #[inline]
    pub unsafe fn getEntryAtIndex(&self, index: libc::int32_t, modifyIndex: bool) -> Result<Option<RefPtr<nsISHEntry>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getEntryAtIndex)(self as *const _, index, modifyIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void restoreToEntryAtIndex (in long index); */
    #[inline]
    pub unsafe fn restoreToEntryAtIndex(&self, index: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).restoreToEntryAtIndex)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void PurgeHistory (in long numEntries); */
    #[inline]
    pub unsafe fn PurgeHistory(&self, numEntries: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).PurgeHistory)(self as *const _, numEntries) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addSHistoryListener (in nsISHistoryListener aListener); */
    #[inline]
    pub unsafe fn addSHistoryListener(&self, aListener: Option<&nsISHistoryListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addSHistoryListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeSHistoryListener (in nsISHistoryListener aListener); */
    #[inline]
    pub unsafe fn removeSHistoryListener(&self, aListener: Option<&nsISHistoryListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeSHistoryListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPartialSHistoryListener (in nsIPartialSHistoryListener aListener); */
    #[inline]
    pub unsafe fn setPartialSHistoryListener(&self, aListener: Option<&nsIPartialSHistoryListener>) -> Result<(), nsresult> {

        match ((*self.vtable).setPartialSHistoryListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsISimpleEnumerator SHistoryEnumerator; */
    #[inline]
    pub unsafe fn get_SHistoryEnumerator(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_SHistoryEnumerator)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void reloadCurrentEntry (); */
    #[inline]
    pub unsafe fn reloadCurrentEntry(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reloadCurrentEntry)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* long getIndexOfEntry (in nsISHEntry aEntry); */
    #[inline]
    pub unsafe fn getIndexOfEntry(&self, aEntry: Option<&nsISHEntry>) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getIndexOfEntry)(self as *const _, aEntry.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void onPartialSHistoryActive (in long globalLength, in long targetIndex); */
    #[inline]
    pub unsafe fn onPartialSHistoryActive(&self, globalLength: libc::int32_t, targetIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onPartialSHistoryActive)(self as *const _, globalLength, targetIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onPartialSHistoryDeactive (); */
    #[inline]
    pub unsafe fn onPartialSHistoryDeactive(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onPartialSHistoryDeactive)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onAttachGroupedSHistory (in long offset); */
    #[inline]
    pub unsafe fn onAttachGroupedSHistory(&self, offset: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onAttachGroupedSHistory)(self as *const _, offset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


