//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrefService.idl
//


#[repr(C)]
pub struct nsIPrefService {
    vtable: *const nsIPrefServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrefService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1f84fd56, 0x3956, 0x40df,
            [0xb8, 0x6a, 0x1e, 0xa0, 0x14, 0x02, 0xee, 0x96])
    }
}

unsafe impl RefCounted for nsIPrefService {
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
pub trait nsIPrefServiceCoerce {
    fn coerce_from(v: &nsIPrefService) -> &Self;
}

impl nsIPrefServiceCoerce for nsIPrefService {
    #[inline]
    fn coerce_from(v: &nsIPrefService) -> &Self {
        v
    }
}

impl nsIPrefService {
    #[inline]
    pub fn coerce<T: nsIPrefServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrefService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPrefServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrefService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrefServiceVTable {
    pub __base: nsISupportsVTable,

    /* void readUserPrefs (in nsIFile aFile); */
    pub readUserPrefs: unsafe extern "C" fn (this: *const nsIPrefService, aFile: *const nsIFile) -> nsresult,

    /* void resetPrefs (); */
    pub resetPrefs: unsafe extern "C" fn (this: *const nsIPrefService) -> nsresult,

    /* void resetUserPrefs (); */
    pub resetUserPrefs: unsafe extern "C" fn (this: *const nsIPrefService) -> nsresult,

    /* void savePrefFile (in nsIFile aFile); */
    pub savePrefFile: unsafe extern "C" fn (this: *const nsIPrefService, aFile: *const nsIFile) -> nsresult,

    /* nsIPrefBranch getBranch (in string aPrefRoot); */
    pub getBranch: unsafe extern "C" fn (this: *const nsIPrefService, aPrefRoot: *const libc::c_char, _retval: *mut *const nsIPrefBranch) -> nsresult,

    /* nsIPrefBranch getDefaultBranch (in string aPrefRoot); */
    pub getDefaultBranch: unsafe extern "C" fn (this: *const nsIPrefService, aPrefRoot: *const libc::c_char, _retval: *mut *const nsIPrefBranch) -> nsresult,

    /* readonly attribute boolean dirty; */
    pub get_dirty: unsafe extern "C" fn (this: *const nsIPrefService, aDirty: *mut bool) -> nsresult,

}


impl nsIPrefService {
    /* void readUserPrefs (in nsIFile aFile); */
    #[inline]
    pub unsafe fn readUserPrefs(&self, aFile: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).readUserPrefs)(self as *const _, aFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resetPrefs (); */
    #[inline]
    pub unsafe fn resetPrefs(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resetPrefs)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resetUserPrefs (); */
    #[inline]
    pub unsafe fn resetUserPrefs(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resetUserPrefs)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void savePrefFile (in nsIFile aFile); */
    #[inline]
    pub unsafe fn savePrefFile(&self, aFile: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).savePrefFile)(self as *const _, aFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIPrefBranch getBranch (in string aPrefRoot); */
    #[inline]
    pub unsafe fn getBranch(&self, aPrefRoot: *const libc::c_char) -> Result<Option<RefPtr<nsIPrefBranch>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getBranch)(self as *const _, aPrefRoot, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIPrefBranch getDefaultBranch (in string aPrefRoot); */
    #[inline]
    pub unsafe fn getDefaultBranch(&self, aPrefRoot: *const libc::c_char) -> Result<Option<RefPtr<nsIPrefBranch>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getDefaultBranch)(self as *const _, aPrefRoot, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean dirty; */
    #[inline]
    pub unsafe fn get_dirty(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_dirty)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


