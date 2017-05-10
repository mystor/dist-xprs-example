//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStyleSheetService.idl
//


pub mod nsIStyleSheetService_consts {
    pub const AGENT_SHEET: i64 = 0;
    pub const USER_SHEET: i64 = 1;
    pub const AUTHOR_SHEET: i64 = 2;
}


#[repr(C)]
pub struct nsIStyleSheetService {
    vtable: *const nsIStyleSheetServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStyleSheetService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4de68896, 0xe8eb, 0x41de,
            [0x82, 0x37, 0xa7, 0x97, 0xb5, 0x70, 0xac, 0x4a])
    }
}

unsafe impl RefCounted for nsIStyleSheetService {
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
pub trait nsIStyleSheetServiceCoerce {
    fn coerce_from(v: &nsIStyleSheetService) -> &Self;
}

impl nsIStyleSheetServiceCoerce for nsIStyleSheetService {
    #[inline]
    fn coerce_from(v: &nsIStyleSheetService) -> &Self {
        v
    }
}

impl nsIStyleSheetService {
    #[inline]
    pub fn coerce<T: nsIStyleSheetServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStyleSheetService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStyleSheetServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStyleSheetService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStyleSheetServiceVTable {
    pub __base: nsISupportsVTable,

    /* void loadAndRegisterSheet (in nsIURI sheetURI, in unsigned long type); */
    pub loadAndRegisterSheet: unsafe extern "C" fn (this: *const nsIStyleSheetService, sheetURI: *const nsIURI, type_: libc::uint32_t) -> nsresult,

    /* boolean sheetRegistered (in nsIURI sheetURI, in unsigned long type); */
    pub sheetRegistered: unsafe extern "C" fn (this: *const nsIStyleSheetService, sheetURI: *const nsIURI, type_: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* nsIPreloadedStyleSheet preloadSheet (in nsIURI sheetURI, in unsigned long type); */
    pub preloadSheet: unsafe extern "C" fn (this: *const nsIStyleSheetService, sheetURI: *const nsIURI, type_: libc::uint32_t, _retval: *mut *const nsIPreloadedStyleSheet) -> nsresult,

    /* [implicit_jscontext] jsval preloadSheetAsync (in nsIURI sheetURI, in unsigned long type); */
    /// Unable to call function as its signature contains a non-rust type
    pub preloadSheetAsync: *const ::libc::c_void,

    /* void unregisterSheet (in nsIURI sheetURI, in unsigned long type); */
    pub unregisterSheet: unsafe extern "C" fn (this: *const nsIStyleSheetService, sheetURI: *const nsIURI, type_: libc::uint32_t) -> nsresult,

}


impl nsIStyleSheetService {
    /* void loadAndRegisterSheet (in nsIURI sheetURI, in unsigned long type); */
    #[inline]
    pub unsafe fn loadAndRegisterSheet(&self, sheetURI: Option<&nsIURI>, type_: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).loadAndRegisterSheet)(self as *const _, sheetURI.map_or(::std::ptr::null(), |x| x as *const _), type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean sheetRegistered (in nsIURI sheetURI, in unsigned long type); */
    #[inline]
    pub unsafe fn sheetRegistered(&self, sheetURI: Option<&nsIURI>, type_: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).sheetRegistered)(self as *const _, sheetURI.map_or(::std::ptr::null(), |x| x as *const _), type_, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIPreloadedStyleSheet preloadSheet (in nsIURI sheetURI, in unsigned long type); */
    #[inline]
    pub unsafe fn preloadSheet(&self, sheetURI: Option<&nsIURI>, type_: libc::uint32_t) -> Result<Option<RefPtr<nsIPreloadedStyleSheet>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).preloadSheet)(self as *const _, sheetURI.map_or(::std::ptr::null(), |x| x as *const _), type_, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext] jsval preloadSheetAsync (in nsIURI sheetURI, in unsigned long type); */


    /* void unregisterSheet (in nsIURI sheetURI, in unsigned long type); */
    #[inline]
    pub unsafe fn unregisterSheet(&self, sheetURI: Option<&nsIURI>, type_: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterSheet)(self as *const _, sheetURI.map_or(::std::ptr::null(), |x| x as *const _), type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


