//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIPlacesAutoComplete.idl
//


pub mod mozIPlacesAutoComplete_consts {
    pub const MATCH_ANYWHERE: i64 = 0;
    pub const MATCH_BOUNDARY_ANYWHERE: i64 = 1;
    pub const MATCH_BOUNDARY: i64 = 2;
    pub const MATCH_BEGINNING: i64 = 3;
    pub const MATCH_ANYWHERE_UNMODIFIED: i64 = 4;
    pub const MATCH_BEGINNING_CASE_SENSITIVE: i64 = 5;
    pub const BEHAVIOR_HISTORY: i64 = 1;
    pub const BEHAVIOR_BOOKMARK: i64 = 2;
    pub const BEHAVIOR_TAG: i64 = 4;
    pub const BEHAVIOR_TITLE: i64 = 8;
    pub const BEHAVIOR_URL: i64 = 16;
    pub const BEHAVIOR_TYPED: i64 = 32;
    pub const BEHAVIOR_JAVASCRIPT: i64 = 64;
    pub const BEHAVIOR_OPENPAGE: i64 = 128;
    pub const BEHAVIOR_RESTRICT: i64 = 256;
    pub const BEHAVIOR_SEARCHES: i64 = 512;
}


#[repr(C)]
pub struct mozIPlacesAutoComplete {
    vtable: *const mozIPlacesAutoCompleteVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIPlacesAutoComplete {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x61b6348a, 0x09e1, 0x4810,
            [0x80, 0x57, 0xf8, 0xcb, 0x3c, 0xec, 0x6e, 0xf8])
    }
}

unsafe impl RefCounted for mozIPlacesAutoComplete {
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
pub trait mozIPlacesAutoCompleteCoerce {
    fn coerce_from(v: &mozIPlacesAutoComplete) -> &Self;
}

impl mozIPlacesAutoCompleteCoerce for mozIPlacesAutoComplete {
    #[inline]
    fn coerce_from(v: &mozIPlacesAutoComplete) -> &Self {
        v
    }
}

impl mozIPlacesAutoComplete {
    #[inline]
    pub fn coerce<T: mozIPlacesAutoCompleteCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIPlacesAutoComplete {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIPlacesAutoCompleteCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIPlacesAutoComplete) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIPlacesAutoCompleteVTable {
    pub __base: nsISupportsVTable,

    /* void registerOpenPage (in nsIURI aURI, in uint32_t aUserContextId); */
    pub registerOpenPage: unsafe extern "C" fn (this: *const mozIPlacesAutoComplete, aURI: *const nsIURI, aUserContextId: uint32_t) -> nsresult,

    /* void unregisterOpenPage (in nsIURI aURI, in uint32_t aUserContextId); */
    pub unregisterOpenPage: unsafe extern "C" fn (this: *const mozIPlacesAutoComplete, aURI: *const nsIURI, aUserContextId: uint32_t) -> nsresult,

    /* void populatePreloadedSiteStorage (in jsval sites); */
    /// Unable to call function as its signature contains a non-rust type
    pub populatePreloadedSiteStorage: *const ::libc::c_void,

}


impl mozIPlacesAutoComplete {
    /* void registerOpenPage (in nsIURI aURI, in uint32_t aUserContextId); */
    #[inline]
    pub unsafe fn registerOpenPage(&self, aURI: Option<&nsIURI>, aUserContextId: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).registerOpenPage)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aUserContextId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterOpenPage (in nsIURI aURI, in uint32_t aUserContextId); */
    #[inline]
    pub unsafe fn unregisterOpenPage(&self, aURI: Option<&nsIURI>, aUserContextId: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterOpenPage)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aUserContextId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void populatePreloadedSiteStorage (in jsval sites); */


}


