//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICryptoHash.idl
//


pub mod nsICryptoHash_consts {
    pub const MD2: i64 = 1;
    pub const MD5: i64 = 2;
    pub const SHA1: i64 = 3;
    pub const SHA256: i64 = 4;
    pub const SHA384: i64 = 5;
    pub const SHA512: i64 = 6;
}


#[repr(C)]
pub struct nsICryptoHash {
    vtable: *const nsICryptoHashVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICryptoHash {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1e5b7c43, 0x4688, 0x45ce,
            [0x92, 0xe1, 0x77, 0xed, 0x93, 0x1e, 0x3b, 0xbe])
    }
}

unsafe impl RefCounted for nsICryptoHash {
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
pub trait nsICryptoHashCoerce {
    fn coerce_from(v: &nsICryptoHash) -> &Self;
}

impl nsICryptoHashCoerce for nsICryptoHash {
    #[inline]
    fn coerce_from(v: &nsICryptoHash) -> &Self {
        v
    }
}

impl nsICryptoHash {
    #[inline]
    pub fn coerce<T: nsICryptoHashCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICryptoHash {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICryptoHashCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICryptoHash) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICryptoHashVTable {
    pub __base: nsISupportsVTable,

    /* void init (in unsigned long aAlgorithm); */
    pub init: unsafe extern "C" fn (this: *const nsICryptoHash, aAlgorithm: libc::uint32_t) -> nsresult,

    /* void initWithString (in ACString aAlgorithm); */
    pub initWithString: unsafe extern "C" fn (this: *const nsICryptoHash, aAlgorithm: *const nsACString) -> nsresult,

    /* void update ([array, size_is (aLen), const] in octet aData, in unsigned long aLen); */
    /// Unable to call function as its signature contains a non-rust type
    pub update: *const ::libc::c_void,

    /* void updateFromStream (in nsIInputStream aStream, in unsigned long aLen); */
    pub updateFromStream: unsafe extern "C" fn (this: *const nsICryptoHash, aStream: *const nsIInputStream, aLen: libc::uint32_t) -> nsresult,

    /* ACString finish (in boolean aASCII); */
    pub finish: unsafe extern "C" fn (this: *const nsICryptoHash, aASCII: bool, _retval: *mut nsACString) -> nsresult,

}


impl nsICryptoHash {
    /* void init (in unsigned long aAlgorithm); */
    #[inline]
    pub unsafe fn init(&self, aAlgorithm: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aAlgorithm) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void initWithString (in ACString aAlgorithm); */
    #[inline]
    pub unsafe fn initWithString(&self, aAlgorithm: &[u8]) -> Result<(), nsresult> {
        let aAlgorithm = nsCString::from(aAlgorithm);
        match ((*self.vtable).initWithString)(self as *const _, &*aAlgorithm) {
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

}


