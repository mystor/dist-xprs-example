//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGroupedSHistory.idl
//


#[repr(C)]
pub struct nsIGroupedSHistory {
    vtable: *const nsIGroupedSHistoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGroupedSHistory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x813e498d, 0x73a8, 0x449a,
            [0xbe, 0x09, 0x61, 0x87, 0xe6, 0x2c, 0x53, 0x52])
    }
}

unsafe impl RefCounted for nsIGroupedSHistory {
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
pub trait nsIGroupedSHistoryCoerce {
    fn coerce_from(v: &nsIGroupedSHistory) -> &Self;
}

impl nsIGroupedSHistoryCoerce for nsIGroupedSHistory {
    #[inline]
    fn coerce_from(v: &nsIGroupedSHistory) -> &Self {
        v
    }
}

impl nsIGroupedSHistory {
    #[inline]
    pub fn coerce<T: nsIGroupedSHistoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGroupedSHistory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIGroupedSHistoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGroupedSHistory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGroupedSHistoryVTable {
    pub __base: nsISupportsVTable,

    /* [infallible] readonly attribute unsigned long count; */
    pub get_count: unsafe extern "C" fn (this: *const nsIGroupedSHistory, aCount: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIFrameLoader activeFrameLoader; */
    pub get_activeFrameLoader: unsafe extern "C" fn (this: *const nsIGroupedSHistory, aActiveFrameLoader: *mut *const nsIFrameLoader) -> nsresult,

    /* void appendPartialSHistory (in nsIPartialSHistory aPartialHistory); */
    pub appendPartialSHistory: unsafe extern "C" fn (this: *const nsIGroupedSHistory, aPartialHistory: *const nsIPartialSHistory) -> nsresult,

    /* void handleSHistoryUpdate (in nsIPartialSHistory aPartialHistory, in boolean aTruncate); */
    pub handleSHistoryUpdate: unsafe extern "C" fn (this: *const nsIGroupedSHistory, aPartialHistory: *const nsIPartialSHistory, aTruncate: bool) -> nsresult,

    /* nsIFrameLoader gotoIndex (in unsigned long aGlobalIndex); */
    pub gotoIndex: unsafe extern "C" fn (this: *const nsIGroupedSHistory, aGlobalIndex: libc::uint32_t, _retval: *mut *const nsIFrameLoader) -> nsresult,

    /* void closeInactiveFrameLoaderOwners (); */
    pub closeInactiveFrameLoaderOwners: unsafe extern "C" fn (this: *const nsIGroupedSHistory) -> nsresult,

    /* void addPrerenderingPartialSHistory (in nsIPartialSHistory aPrerendering, in long aId); */
    pub addPrerenderingPartialSHistory: unsafe extern "C" fn (this: *const nsIGroupedSHistory, aPrerendering: *const nsIPartialSHistory, aId: libc::int32_t) -> nsresult,

    /* [implicit_jscontext] nsISupports activatePrerendering (in long aId); */
    /// Unable to call function as its signature contains a non-rust type
    pub activatePrerendering: *const ::libc::c_void,

    /* void cancelPrerendering (in long aId); */
    pub cancelPrerendering: unsafe extern "C" fn (this: *const nsIGroupedSHistory, aId: libc::int32_t) -> nsresult,

}


impl nsIGroupedSHistory {
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

    /* readonly attribute nsIFrameLoader activeFrameLoader; */
    #[inline]
    pub unsafe fn get_activeFrameLoader(&self, ) -> Result<Option<RefPtr<nsIFrameLoader>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_activeFrameLoader)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void appendPartialSHistory (in nsIPartialSHistory aPartialHistory); */
    #[inline]
    pub unsafe fn appendPartialSHistory(&self, aPartialHistory: Option<&nsIPartialSHistory>) -> Result<(), nsresult> {

        match ((*self.vtable).appendPartialSHistory)(self as *const _, aPartialHistory.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleSHistoryUpdate (in nsIPartialSHistory aPartialHistory, in boolean aTruncate); */
    #[inline]
    pub unsafe fn handleSHistoryUpdate(&self, aPartialHistory: Option<&nsIPartialSHistory>, aTruncate: bool) -> Result<(), nsresult> {

        match ((*self.vtable).handleSHistoryUpdate)(self as *const _, aPartialHistory.map_or(::std::ptr::null(), |x| x as *const _), aTruncate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIFrameLoader gotoIndex (in unsigned long aGlobalIndex); */
    #[inline]
    pub unsafe fn gotoIndex(&self, aGlobalIndex: libc::uint32_t) -> Result<Option<RefPtr<nsIFrameLoader>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).gotoIndex)(self as *const _, aGlobalIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void closeInactiveFrameLoaderOwners (); */
    #[inline]
    pub unsafe fn closeInactiveFrameLoaderOwners(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).closeInactiveFrameLoaderOwners)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addPrerenderingPartialSHistory (in nsIPartialSHistory aPrerendering, in long aId); */
    #[inline]
    pub unsafe fn addPrerenderingPartialSHistory(&self, aPrerendering: Option<&nsIPartialSHistory>, aId: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).addPrerenderingPartialSHistory)(self as *const _, aPrerendering.map_or(::std::ptr::null(), |x| x as *const _), aId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] nsISupports activatePrerendering (in long aId); */


    /* void cancelPrerendering (in long aId); */
    #[inline]
    pub unsafe fn cancelPrerendering(&self, aId: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).cancelPrerendering)(self as *const _, aId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


