//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageVacuumParticipant.idl
//


#[repr(C)]
pub struct mozIStorageVacuumParticipant {
    vtable: *const mozIStorageVacuumParticipantVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageVacuumParticipant {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8f367508, 0x1d9a, 0x4d3f,
            [0xbe, 0x0c, 0xac, 0x11, 0xb6, 0xdd, 0x7d, 0xbf])
    }
}

unsafe impl RefCounted for mozIStorageVacuumParticipant {
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
pub trait mozIStorageVacuumParticipantCoerce {
    fn coerce_from(v: &mozIStorageVacuumParticipant) -> &Self;
}

impl mozIStorageVacuumParticipantCoerce for mozIStorageVacuumParticipant {
    #[inline]
    fn coerce_from(v: &mozIStorageVacuumParticipant) -> &Self {
        v
    }
}

impl mozIStorageVacuumParticipant {
    #[inline]
    pub fn coerce<T: mozIStorageVacuumParticipantCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageVacuumParticipant {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStorageVacuumParticipantCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageVacuumParticipant) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageVacuumParticipantVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long expectedDatabasePageSize; */
    pub get_expectedDatabasePageSize: unsafe extern "C" fn (this: *const mozIStorageVacuumParticipant, aExpectedDatabasePageSize: *mut libc::int32_t) -> nsresult,

    /* readonly attribute mozIStorageConnection databaseConnection; */
    pub get_databaseConnection: unsafe extern "C" fn (this: *const mozIStorageVacuumParticipant, aDatabaseConnection: *mut *const mozIStorageConnection) -> nsresult,

    /* boolean onBeginVacuum (); */
    pub onBeginVacuum: unsafe extern "C" fn (this: *const mozIStorageVacuumParticipant, _retval: *mut bool) -> nsresult,

    /* void onEndVacuum (in boolean aSucceeded); */
    pub onEndVacuum: unsafe extern "C" fn (this: *const mozIStorageVacuumParticipant, aSucceeded: bool) -> nsresult,

}


impl mozIStorageVacuumParticipant {
    /* readonly attribute long expectedDatabasePageSize; */
    #[inline]
    pub unsafe fn get_expectedDatabasePageSize(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_expectedDatabasePageSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute mozIStorageConnection databaseConnection; */
    #[inline]
    pub unsafe fn get_databaseConnection(&self, ) -> Result<Option<RefPtr<mozIStorageConnection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_databaseConnection)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean onBeginVacuum (); */
    #[inline]
    pub unsafe fn onBeginVacuum(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).onBeginVacuum)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void onEndVacuum (in boolean aSucceeded); */
    #[inline]
    pub unsafe fn onEndVacuum(&self, aSucceeded: bool) -> Result<(), nsresult> {

        match ((*self.vtable).onEndVacuum)(self as *const _, aSucceeded) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


