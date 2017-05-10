//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICryptoHMAC.idl
//


pub mod nsICryptoHMAC_consts {
    pub const MD5: i64 = 2;
    pub const SHA1: i64 = 3;
    pub const SHA256: i64 = 4;
    pub const SHA384: i64 = 5;
    pub const SHA512: i64 = 6;
}


#[repr(C)]
pub struct nsICryptoHMAC {
    vtable: *const nsICryptoHMACVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICryptoHMAC {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8feb4c7c, 0x1641, 0x4a7b,
            [0xbc, 0x6d, 0x19, 0x64, 0xe2, 0x09, 0x94, 0x97])
    }
}

unsafe impl RefCounted for nsICryptoHMAC {
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
pub trait nsICryptoHMACCoerce {
    fn coerce_from(v: &nsICryptoHMAC) -> &Self;
}

impl nsICryptoHMACCoerce for nsICryptoHMAC {
    #[inline]
    fn coerce_from(v: &nsICryptoHMAC) -> &Self {
        v
    }
}

impl nsICryptoHMAC {
    #[inline]
    pub fn coerce<T: nsICryptoHMACCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICryptoHMAC {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICryptoHMACCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICryptoHMAC) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICryptoHMACVTable {
    pub __base: nsISupportsVTable,

    /* void init (in unsigned long aAlgorithm, in nsIKeyObject aKeyObject); */
    pub init: unsafe extern "C" fn (this: *const nsICryptoHMAC, aAlgorithm: libc::uint32_t, aKeyObject: *const nsIKeyObject) -> nsresult,

    /* void update ([array, size_is (aLen), const] in octet aData, in unsigned long aLen); */
    /// Unable to call function as its signature contains a non-rust type
    pub update: *const ::libc::c_void,

    /* void updateFromStream (in nsIInputStream aStream, in unsigned long aLen); */
    pub updateFromStream: unsafe extern "C" fn (this: *const nsICryptoHMAC, aStream: *const nsIInputStream, aLen: libc::uint32_t) -> nsresult,

    /* ACString finish (in boolean aASCII); */
    pub finish: unsafe extern "C" fn (this: *const nsICryptoHMAC, aASCII: bool, _retval: *mut nsACString) -> nsresult,

    /* void reset (); */
    pub reset: unsafe extern "C" fn (this: *const nsICryptoHMAC) -> nsresult,

}


impl nsICryptoHMAC {
    /* void init (in unsigned long aAlgorithm, in nsIKeyObject aKeyObject); */
    #[inline]
    pub unsafe fn init(&self, aAlgorithm: libc::uint32_t, aKeyObject: Option<&nsIKeyObject>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aAlgorithm, aKeyObject.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void update ([array, size_is (aLen), const] in octet aData, in unsigned long aLen); */


    /* void updateFromStream (in nsIInputStream aStream, in unsigned long aLen); */
    #[inline]
    pub unsafe fn updateFromStream(&self, aStream: Option<&nsIInputStream>, aLen: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).updateFromStream)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), aLen) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* ACString finish (in boolean aASCII); */
    #[inline]
    pub unsafe fn finish(&self, aASCII: bool) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).finish)(self as *const _, aASCII, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void reset (); */
    #[inline]
    pub unsafe fn reset(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reset)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


