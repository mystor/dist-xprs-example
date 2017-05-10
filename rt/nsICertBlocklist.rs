//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICertBlocklist.idl
//


#[repr(C)]
pub struct nsICertBlocklist {
    vtable: *const nsICertBlocklistVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICertBlocklist {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe0654480, 0xf433, 0x11e4,
            [0xb9, 0x39, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsICertBlocklist {
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
pub trait nsICertBlocklistCoerce {
    fn coerce_from(v: &nsICertBlocklist) -> &Self;
}

impl nsICertBlocklistCoerce for nsICertBlocklist {
    #[inline]
    fn coerce_from(v: &nsICertBlocklist) -> &Self {
        v
    }
}

impl nsICertBlocklist {
    #[inline]
    pub fn coerce<T: nsICertBlocklistCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICertBlocklist {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICertBlocklistCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICertBlocklist) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICertBlocklistVTable {
    pub __base: nsISupportsVTable,

    /* void revokeCertByIssuerAndSerial (in ACString issuer, in ACString serialNumber); */
    pub revokeCertByIssuerAndSerial: unsafe extern "C" fn (this: *const nsICertBlocklist, issuer: *const nsACString, serialNumber: *const nsACString) -> nsresult,

    /* void revokeCertBySubjectAndPubKey (in ACString subject, in ACString pubKeyHash); */
    pub revokeCertBySubjectAndPubKey: unsafe extern "C" fn (this: *const nsICertBlocklist, subject: *const nsACString, pubKeyHash: *const nsACString) -> nsresult,

    /* void saveEntries (); */
    pub saveEntries: unsafe extern "C" fn (this: *const nsICertBlocklist) -> nsresult,

    /* boolean isCertRevoked ([array, size_is (issuer_length), const] in octet issuer, in unsigned long issuer_length, [array, size_is (serial_length), const] in octet serial, in unsigned long serial_length, [array, size_is (subject_length), const] in octet subject, in unsigned long subject_length, [array, size_is (pubkey_length), const] in octet pubkey, in unsigned long pubkey_length); */
    /// Unable to call function as its signature contains a non-rust type
    pub isCertRevoked: *const ::libc::c_void,

    /* boolean isBlocklistFresh (); */
    pub isBlocklistFresh: unsafe extern "C" fn (this: *const nsICertBlocklist, _retval: *mut bool) -> nsresult,

}


impl nsICertBlocklist {
    /* void revokeCertByIssuerAndSerial (in ACString issuer, in ACString serialNumber); */
    #[inline]
    pub unsafe fn revokeCertByIssuerAndSerial(&self, issuer: &[u8], serialNumber: &[u8]) -> Result<(), nsresult> {
        let issuer = nsCString::from(issuer);
        let serialNumber = nsCString::from(serialNumber);
        match ((*self.vtable).revokeCertByIssuerAndSerial)(self as *const _, &*issuer, &*serialNumber) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void revokeCertBySubjectAndPubKey (in ACString subject, in ACString pubKeyHash); */
    #[inline]
    pub unsafe fn revokeCertBySubjectAndPubKey(&self, subject: &[u8], pubKeyHash: &[u8]) -> Result<(), nsresult> {
        let subject = nsCString::from(subject);
        let pubKeyHash = nsCString::from(pubKeyHash);
        match ((*self.vtable).revokeCertBySubjectAndPubKey)(self as *const _, &*subject, &*pubKeyHash) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void saveEntries (); */
    #[inline]
    pub unsafe fn saveEntries(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).saveEntries)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isCertRevoked ([array, size_is (issuer_length), const] in octet issuer, in unsigned long issuer_length, [array, size_is (serial_length), const] in octet serial, in unsigned long serial_length, [array, size_is (subject_length), const] in octet subject, in unsigned long subject_length, [array, size_is (pubkey_length), const] in octet pubkey, in unsigned long pubkey_length); */


    /* boolean isBlocklistFresh (); */
    #[inline]
    pub unsafe fn isBlocklistFresh(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isBlocklistFresh)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


