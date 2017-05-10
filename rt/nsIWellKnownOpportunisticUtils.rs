//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWellKnownOpportunisticUtils.idl
//


#[repr(C)]
pub struct nsIWellKnownOpportunisticUtils {
    vtable: *const nsIWellKnownOpportunisticUtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWellKnownOpportunisticUtils {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb4f96c89, 0x5238, 0x450c,
            [0x8b, 0xda, 0xe1, 0x2c, 0x26, 0xf1, 0xd1, 0x50])
    }
}

unsafe impl RefCounted for nsIWellKnownOpportunisticUtils {
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
pub trait nsIWellKnownOpportunisticUtilsCoerce {
    fn coerce_from(v: &nsIWellKnownOpportunisticUtils) -> &Self;
}

impl nsIWellKnownOpportunisticUtilsCoerce for nsIWellKnownOpportunisticUtils {
    #[inline]
    fn coerce_from(v: &nsIWellKnownOpportunisticUtils) -> &Self {
        v
    }
}

impl nsIWellKnownOpportunisticUtils {
    #[inline]
    pub fn coerce<T: nsIWellKnownOpportunisticUtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWellKnownOpportunisticUtils {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWellKnownOpportunisticUtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWellKnownOpportunisticUtils) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWellKnownOpportunisticUtilsVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] void verify (in ACString aJSON, in ACString aOrigin, in long aAlternatePort); */
    pub verify: unsafe extern "C" fn (this: *const nsIWellKnownOpportunisticUtils, aJSON: *const nsACString, aOrigin: *const nsACString, aAlternatePort: libc::int32_t) -> nsresult,

    /* [must_use] readonly attribute bool valid; */
    pub get_valid: unsafe extern "C" fn (this: *const nsIWellKnownOpportunisticUtils, aValid: *mut bool) -> nsresult,

    /* [must_use] readonly attribute bool mixed; */
    pub get_mixed: unsafe extern "C" fn (this: *const nsIWellKnownOpportunisticUtils, aMixed: *mut bool) -> nsresult,

    /* [must_use] readonly attribute long lifetime; */
    pub get_lifetime: unsafe extern "C" fn (this: *const nsIWellKnownOpportunisticUtils, aLifetime: *mut libc::int32_t) -> nsresult,

}


impl nsIWellKnownOpportunisticUtils {
    /* [must_use] void verify (in ACString aJSON, in ACString aOrigin, in long aAlternatePort); */
    #[inline]
    pub unsafe fn verify(&self, aJSON: &[u8], aOrigin: &[u8], aAlternatePort: libc::int32_t) -> Result<(), nsresult> {
        let aJSON = nsCString::from(aJSON);
        let aOrigin = nsCString::from(aOrigin);
        match ((*self.vtable).verify)(self as *const _, &*aJSON, &*aOrigin, aAlternatePort) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute bool valid; */
    #[inline]
    pub unsafe fn get_valid(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_valid)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute bool mixed; */
    #[inline]
    pub unsafe fn get_mixed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mixed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute long lifetime; */
    #[inline]
    pub unsafe fn get_lifetime(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lifetime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


