//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProfileMigrator.idl
//


#[repr(C)]
pub struct nsIProfileStartup {
    vtable: *const nsIProfileStartupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProfileStartup {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x048e5ca1, 0x0eb7, 0x4bb1,
            [0xa9, 0xa2, 0xa3, 0x6f, 0x7d, 0x4e, 0x0e, 0x3c])
    }
}

unsafe impl RefCounted for nsIProfileStartup {
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
pub trait nsIProfileStartupCoerce {
    fn coerce_from(v: &nsIProfileStartup) -> &Self;
}

impl nsIProfileStartupCoerce for nsIProfileStartup {
    #[inline]
    fn coerce_from(v: &nsIProfileStartup) -> &Self {
        v
    }
}

impl nsIProfileStartup {
    #[inline]
    pub fn coerce<T: nsIProfileStartupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProfileStartup {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProfileStartupCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProfileStartup) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProfileStartupVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIFile directory; */
    pub get_directory: unsafe extern "C" fn (this: *const nsIProfileStartup, aDirectory: *mut *const nsIFile) -> nsresult,

    /* void doStartup (); */
    pub doStartup: unsafe extern "C" fn (this: *const nsIProfileStartup) -> nsresult,

}


impl nsIProfileStartup {
    /* readonly attribute nsIFile directory; */
    #[inline]
    pub unsafe fn get_directory(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_directory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void doStartup (); */
    #[inline]
    pub unsafe fn doStartup(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).doStartup)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIProfileMigrator {
    vtable: *const nsIProfileMigratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProfileMigrator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3df284a5, 0x2258, 0x4d46,
            [0xa6, 0x64, 0x76, 0x1e, 0xcd, 0xc0, 0x4c, 0x22])
    }
}

unsafe impl RefCounted for nsIProfileMigrator {
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
pub trait nsIProfileMigratorCoerce {
    fn coerce_from(v: &nsIProfileMigrator) -> &Self;
}

impl nsIProfileMigratorCoerce for nsIProfileMigrator {
    #[inline]
    fn coerce_from(v: &nsIProfileMigrator) -> &Self {
        v
    }
}

impl nsIProfileMigrator {
    #[inline]
    pub fn coerce<T: nsIProfileMigratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProfileMigrator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProfileMigratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProfileMigrator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProfileMigratorVTable {
    pub __base: nsISupportsVTable,

    /* void migrate (in nsIProfileStartup aStartup, in ACString aKey, [optional] in ACString aProfileName); */
    pub migrate: unsafe extern "C" fn (this: *const nsIProfileMigrator, aStartup: *const nsIProfileStartup, aKey: *const nsACString, aProfileName: *const nsACString) -> nsresult,

}


impl nsIProfileMigrator {
    /* void migrate (in nsIProfileStartup aStartup, in ACString aKey, [optional] in ACString aProfileName); */
    #[inline]
    pub unsafe fn migrate(&self, aStartup: Option<&nsIProfileStartup>, aKey: &[u8], aProfileName: &[u8]) -> Result<(), nsresult> {
        let aKey = nsCString::from(aKey);
        let aProfileName = nsCString::from(aProfileName);
        match ((*self.vtable).migrate)(self as *const _, aStartup.map_or(::std::ptr::null(), |x| x as *const _), &*aKey, &*aProfileName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


