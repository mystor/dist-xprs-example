//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBrowserProfileMigrator.idl
//


pub mod nsIBrowserProfileMigrator_consts {
    pub const ALL: i64 = 0;
    pub const SETTINGS: i64 = 1;
    pub const COOKIES: i64 = 2;
    pub const HISTORY: i64 = 4;
    pub const FORMDATA: i64 = 8;
    pub const PASSWORDS: i64 = 16;
    pub const BOOKMARKS: i64 = 32;
    pub const OTHERDATA: i64 = 64;
    pub const SESSION: i64 = 128;
}


#[repr(C)]
pub struct nsIBrowserProfileMigrator {
    vtable: *const nsIBrowserProfileMigratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBrowserProfileMigrator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x22b56ffc, 0x3149, 0x43c5,
            [0xb5, 0xa9, 0xb3, 0xa6, 0xb6, 0x78, 0xde, 0x93])
    }
}

unsafe impl RefCounted for nsIBrowserProfileMigrator {
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
pub trait nsIBrowserProfileMigratorCoerce {
    fn coerce_from(v: &nsIBrowserProfileMigrator) -> &Self;
}

impl nsIBrowserProfileMigratorCoerce for nsIBrowserProfileMigrator {
    #[inline]
    fn coerce_from(v: &nsIBrowserProfileMigrator) -> &Self {
        v
    }
}

impl nsIBrowserProfileMigrator {
    #[inline]
    pub fn coerce<T: nsIBrowserProfileMigratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBrowserProfileMigrator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBrowserProfileMigratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserProfileMigrator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBrowserProfileMigratorVTable {
    pub __base: nsISupportsVTable,

    /* void migrate (in unsigned short aItems, in nsIProfileStartup aStartup, in jsval aProfile); */
    /// Unable to call function as its signature contains a non-rust type
    pub migrate: *const ::libc::c_void,

    /* unsigned short getMigrateData (in jsval aProfile, in boolean aDoingStartup); */
    /// Unable to call function as its signature contains a non-rust type
    pub getMigrateData: *const ::libc::c_void,

    /* jsval getLastUsedDate (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getLastUsedDate: *const ::libc::c_void,

    /* readonly attribute boolean sourceExists; */
    pub get_sourceExists: unsafe extern "C" fn (this: *const nsIBrowserProfileMigrator, aSourceExists: *mut bool) -> nsresult,

    /* readonly attribute jsval sourceProfiles; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_sourceProfiles: *const ::libc::c_void,

    /* readonly attribute AUTF8String sourceHomePageURL; */
    pub get_sourceHomePageURL: unsafe extern "C" fn (this: *const nsIBrowserProfileMigrator, aSourceHomePageURL: *mut nsACString) -> nsresult,

    /* readonly attribute boolean sourceLocked; */
    pub get_sourceLocked: unsafe extern "C" fn (this: *const nsIBrowserProfileMigrator, aSourceLocked: *mut bool) -> nsresult,

}


impl nsIBrowserProfileMigrator {
    /* void migrate (in unsigned short aItems, in nsIProfileStartup aStartup, in jsval aProfile); */


    /* unsigned short getMigrateData (in jsval aProfile, in boolean aDoingStartup); */


    /* jsval getLastUsedDate (); */


    /* readonly attribute boolean sourceExists; */
    #[inline]
    pub unsafe fn get_sourceExists(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_sourceExists)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute jsval sourceProfiles; */


    /* readonly attribute AUTF8String sourceHomePageURL; */
    #[inline]
    pub unsafe fn get_sourceHomePageURL(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_sourceHomePageURL)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean sourceLocked; */
    #[inline]
    pub unsafe fn get_sourceLocked(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_sourceLocked)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


