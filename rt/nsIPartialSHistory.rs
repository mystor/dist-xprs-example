//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPartialSHistory.idl
//


pub mod nsIPartialSHistory_consts {
    pub const STATE_INACTIVE: i64 = 0;
    pub const STATE_ACTIVE: i64 = 1;
    pub const STATE_PRERENDER: i64 = 2;
}


#[repr(C)]
pub struct nsIPartialSHistory {
    vtable: *const nsIPartialSHistoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPartialSHistory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5cd75e28, 0x838c, 0x4a0a,
            [0x97, 0x2e, 0x60, 0x05, 0xf7, 0x36, 0xef, 0x7a])
    }
}

unsafe impl RefCounted for nsIPartialSHistory {
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
pub trait nsIPartialSHistoryCoerce {
    fn coerce_from(v: &nsIPartialSHistory) -> &Self;
}

impl nsIPartialSHistoryCoerce for nsIPartialSHistory {
    #[inline]
    fn coerce_from(v: &nsIPartialSHistory) -> &Self {
        v
    }
}

impl nsIPartialSHistory {
    #[inline]
    pub fn coerce<T: nsIPartialSHistoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPartialSHistory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPartialSHistoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPartialSHistory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPartialSHistoryVTable {
    pub __base: nsISupportsVTable,

    /* [infallible] readonly attribute unsigned long count; */
    pub get_count: unsafe extern "C" fn (this: *const nsIPartialSHistory, aCount: *mut libc::uint32_t) -> nsresult,

    /* [infallible] readonly attribute long globalIndex; */
    pub get_globalIndex: unsafe extern "C" fn (this: *const nsIPartialSHistory, aGlobalIndex: *mut libc::int32_t) -> nsresult,

    /* [infallible] readonly attribute unsigned long globalIndexOffset; */
    pub get_globalIndexOffset: unsafe extern "C" fn (this: *const nsIPartialSHistory, aGlobalIndexOffset: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIFrameLoader ownerFrameLoader; */
    pub get_ownerFrameLoader: unsafe extern "C" fn (this: *const nsIPartialSHistory, aOwnerFrameLoader: *mut *const nsIFrameLoader) -> nsresult,

    /* readonly attribute nsIGroupedSHistory groupedSHistory; */
    pub get_groupedSHistory: unsafe extern "C" fn (this: *const nsIPartialSHistory, aGroupedSHistory: *mut *const nsIGroupedSHistory) -> nsresult,

    /* [infallible] attribute long activeState; */
    pub get_activeState: unsafe extern "C" fn (this: *const nsIPartialSHistory, aActiveState: *mut libc::int32_t) -> nsresult,
    pub set_activeState: unsafe extern "C" fn (this: *const nsIPartialSHistory, aActiveState: libc::int32_t) -> nsresult,

    /* void onAttachGroupedSHistory (in nsIGroupedSHistory aGroup, in unsigned long aOffset); */
    pub onAttachGroupedSHistory: unsafe extern "C" fn (this: *const nsIPartialSHistory, aGroup: *const nsIGroupedSHistory, aOffset: libc::uint32_t) -> nsresult,

    /* void handleSHistoryUpdate (in unsigned long aCount, in unsigned long aLocalIndex, in boolean aTruncate); */
    pub handleSHistoryUpdate: unsafe extern "C" fn (this: *const nsIPartialSHistory, aCount: libc::uint32_t, aLocalIndex: libc::uint32_t, aTruncate: bool) -> nsresult,

    /* void onActive (in unsigned long aGlobalLength, in unsigned long aTargetLocalIndex); */
    pub onActive: unsafe extern "C" fn (this: *const nsIPartialSHistory, aGlobalLength: libc::uint32_t, aTargetLocalIndex: libc::uint32_t) -> nsresult,

    /* void onDeactive (); */
    pub onDeactive: unsafe extern "C" fn (this: *const nsIPartialSHistory) -> nsresult,

}


impl nsIPartialSHistory {
    /* [infallible] readonly attribute unsigned long count; */
    #[inline]
    pub unsafe fn get_count(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_count)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute long globalIndex; */
    #[inline]
    pub unsafe fn get_globalIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_globalIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute unsigned long globalIndexOffset; */
    #[inline]
    pub unsafe fn get_globalIndexOffset(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_globalIndexOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIFrameLoader ownerFrameLoader; */
    #[inline]
    pub unsafe fn get_ownerFrameLoader(&self, ) -> Result<Option<RefPtr<nsIFrameLoader>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_ownerFrameLoader)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIGroupedSHistory groupedSHistory; */
    #[inline]
    pub unsafe fn get_groupedSHistory(&self, ) -> Result<Option<RefPtr<nsIGroupedSHistory>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_groupedSHistory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [infallible] attribute long activeState; */
    #[inline]
    pub unsafe fn get_activeState(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_activeState)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_activeState(&self, aActiveState: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_activeState)(self as *const _, aActiveState) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onAttachGroupedSHistory (in nsIGroupedSHistory aGroup, in unsigned long aOffset); */
    #[inline]
    pub unsafe fn onAttachGroupedSHistory(&self, aGroup: Option<&nsIGroupedSHistory>, aOffset: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onAttachGroupedSHistory)(self as *const _, aGroup.map_or(::std::ptr::null(), |x| x as *const _), aOffset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleSHistoryUpdate (in unsigned long aCount, in unsigned long aLocalIndex, in boolean aTruncate); */
    #[inline]
    pub unsafe fn handleSHistoryUpdate(&self, aCount: libc::uint32_t, aLocalIndex: libc::uint32_t, aTruncate: bool) -> Result<(), nsresult> {

        match ((*self.vtable).handleSHistoryUpdate)(self as *const _, aCount, aLocalIndex, aTruncate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onActive (in unsigned long aGlobalLength, in unsigned long aTargetLocalIndex); */
    #[inline]
    pub unsafe fn onActive(&self, aGlobalLength: libc::uint32_t, aTargetLocalIndex: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onActive)(self as *const _, aGlobalLength, aTargetLocalIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onDeactive (); */
    #[inline]
    pub unsafe fn onDeactive(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onDeactive)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


