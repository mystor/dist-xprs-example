//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUUIDGenerator.idl
//


#[repr(C)]
pub struct nsIUUIDGenerator {
    vtable: *const nsIUUIDGeneratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUUIDGenerator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x138ad1b2, 0xc694, 0x41cc,
            [0xb2, 0x01, 0x33, 0x3c, 0xe9, 0x36, 0xd8, 0xb8])
    }
}

unsafe impl RefCounted for nsIUUIDGenerator {
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
pub trait nsIUUIDGeneratorCoerce {
    fn coerce_from(v: &nsIUUIDGenerator) -> &Self;
}

impl nsIUUIDGeneratorCoerce for nsIUUIDGenerator {
    #[inline]
    fn coerce_from(v: &nsIUUIDGenerator) -> &Self {
        v
    }
}

impl nsIUUIDGenerator {
    #[inline]
    pub fn coerce<T: nsIUUIDGeneratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUUIDGenerator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUUIDGeneratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUUIDGenerator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUUIDGeneratorVTable {
    pub __base: nsISupportsVTable,

    /* nsIDPtr generateUUID (); */
    pub generateUUID: unsafe extern "C" fn (this: *const nsIUUIDGenerator, _retval: *mut *const nsID) -> nsresult,

    /* [noscript] void generateUUIDInPlace (in nsNonConstIDPtr id); */
    pub generateUUIDInPlace: unsafe extern "C" fn (this: *const nsIUUIDGenerator, id: *const nsID) -> nsresult,

}


impl nsIUUIDGenerator {
    /* nsIDPtr generateUUID (); */
    #[inline]
    pub unsafe fn generateUUID(&self, ) -> Result<*const nsID, nsresult> {
        let mut _retval: *const nsID = ::std::ptr::null();
        match ((*self.vtable).generateUUID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void generateUUIDInPlace (in nsNonConstIDPtr id); */
    #[inline]
    pub unsafe fn generateUUIDInPlace(&self, id: *const nsID) -> Result<(), nsresult> {

        match ((*self.vtable).generateUUIDInPlace)(self as *const _, id) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


