//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIKeyModule.idl
//


pub mod nsIKeyObject_consts {
    pub const SYM_KEY: i64 = 1;
    pub const HMAC: i64 = 257;
}


#[repr(C)]
pub struct nsIKeyObject {
    vtable: *const nsIKeyObjectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIKeyObject {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xee2dc516, 0xba7b, 0x4e77,
            [0x89, 0xfe, 0xc4, 0x3b, 0x88, 0x6e, 0xf7, 0x15])
    }
}

unsafe impl RefCounted for nsIKeyObject {
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
pub trait nsIKeyObjectCoerce {
    fn coerce_from(v: &nsIKeyObject) -> &Self;
}

impl nsIKeyObjectCoerce for nsIKeyObject {
    #[inline]
    fn coerce_from(v: &nsIKeyObject) -> &Self {
        v
    }
}

impl nsIKeyObject {
    #[inline]
    pub fn coerce<T: nsIKeyObjectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIKeyObject {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIKeyObjectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIKeyObject) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIKeyObjectVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] void initKey (in short aAlgorithm, in PK11SymKeyPtr aKey); */
    /// Unable to call function as its signature contains a non-rust type
    pub initKey: *const ::libc::c_void,

    /* [noscript] PK11SymKeyPtr getKeyObj (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getKeyObj: *const ::libc::c_void,

    /* short getType (); */
    pub getType: unsafe extern "C" fn (this: *const nsIKeyObject, _retval: *mut libc::int16_t) -> nsresult,

}


impl nsIKeyObject {
    /* [noscript] void initKey (in short aAlgorithm, in PK11SymKeyPtr aKey); */


    /* [noscript] PK11SymKeyPtr getKeyObj (); */


    /* short getType (); */
    #[inline]
    pub unsafe fn getType(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIKeyObjectFactory {
    vtable: *const nsIKeyObjectFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIKeyObjectFactory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x838bdbf1, 0x8244, 0x448f,
            [0x8b, 0xcd, 0xce, 0xde, 0x70, 0x22, 0x7d, 0x75])
    }
}

unsafe impl RefCounted for nsIKeyObjectFactory {
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
pub trait nsIKeyObjectFactoryCoerce {
    fn coerce_from(v: &nsIKeyObjectFactory) -> &Self;
}

impl nsIKeyObjectFactoryCoerce for nsIKeyObjectFactory {
    #[inline]
    fn coerce_from(v: &nsIKeyObjectFactory) -> &Self {
        v
    }
}

impl nsIKeyObjectFactory {
    #[inline]
    pub fn coerce<T: nsIKeyObjectFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIKeyObjectFactory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIKeyObjectFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIKeyObjectFactory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIKeyObjectFactoryVTable {
    pub __base: nsISupportsVTable,

    /* nsIKeyObject keyFromString (in short aAlgorithm, in ACString aKey); */
    pub keyFromString: unsafe extern "C" fn (this: *const nsIKeyObjectFactory, aAlgorithm: libc::int16_t, aKey: *const nsACString, _retval: *mut *const nsIKeyObject) -> nsresult,

}


impl nsIKeyObjectFactory {
    /* nsIKeyObject keyFromString (in short aAlgorithm, in ACString aKey); */
    #[inline]
    pub unsafe fn keyFromString(&self, aAlgorithm: libc::int16_t, aKey: &[u8]) -> Result<Option<RefPtr<nsIKeyObject>>, nsresult> {
        let aKey = nsCString::from(aKey);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).keyFromString)(self as *const _, aAlgorithm, &*aKey, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


