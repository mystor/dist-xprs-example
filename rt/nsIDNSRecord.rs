//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDNSRecord.idl
//


#[repr(C)]
pub struct nsIDNSRecord {
    vtable: *const nsIDNSRecordVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDNSRecord {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf92228ae, 0xc417, 0x4188,
            [0xa6, 0x04, 0x08, 0x30, 0xa9, 0x5e, 0x7e, 0xb9])
    }
}

unsafe impl RefCounted for nsIDNSRecord {
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
pub trait nsIDNSRecordCoerce {
    fn coerce_from(v: &nsIDNSRecord) -> &Self;
}

impl nsIDNSRecordCoerce for nsIDNSRecord {
    #[inline]
    fn coerce_from(v: &nsIDNSRecord) -> &Self {
        v
    }
}

impl nsIDNSRecord {
    #[inline]
    pub fn coerce<T: nsIDNSRecordCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDNSRecord {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDNSRecordCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSRecord) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDNSRecordVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString canonicalName; */
    pub get_canonicalName: unsafe extern "C" fn (this: *const nsIDNSRecord, aCanonicalName: *mut nsACString) -> nsresult,

    /* [noscript] NetAddr getNextAddr (in uint16_t aPort); */
    /// Unable to call function as its signature contains a non-rust type
    pub getNextAddr: *const ::libc::c_void,

    /* [noscript] void getAddresses (out nsNetAddrTArrayRef aAddressArray); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAddresses: *const ::libc::c_void,

    /* nsINetAddr getScriptableNextAddr (in uint16_t aPort); */
    pub getScriptableNextAddr: unsafe extern "C" fn (this: *const nsIDNSRecord, aPort: uint16_t, _retval: *mut *const nsINetAddr) -> nsresult,

    /* ACString getNextAddrAsString (); */
    pub getNextAddrAsString: unsafe extern "C" fn (this: *const nsIDNSRecord, _retval: *mut nsACString) -> nsresult,

    /* boolean hasMore (); */
    pub hasMore: unsafe extern "C" fn (this: *const nsIDNSRecord, _retval: *mut bool) -> nsresult,

    /* void rewind (); */
    pub rewind: unsafe extern "C" fn (this: *const nsIDNSRecord) -> nsresult,

    /* void reportUnusable (in uint16_t aPort); */
    pub reportUnusable: unsafe extern "C" fn (this: *const nsIDNSRecord, aPort: uint16_t) -> nsresult,

}


impl nsIDNSRecord {
    /* readonly attribute ACString canonicalName; */
    #[inline]
    pub unsafe fn get_canonicalName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_canonicalName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] NetAddr getNextAddr (in uint16_t aPort); */


    /* [noscript] void getAddresses (out nsNetAddrTArrayRef aAddressArray); */


    /* nsINetAddr getScriptableNextAddr (in uint16_t aPort); */
    #[inline]
    pub unsafe fn getScriptableNextAddr(&self, aPort: uint16_t) -> Result<Option<RefPtr<nsINetAddr>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getScriptableNextAddr)(self as *const _, aPort, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* ACString getNextAddrAsString (); */
    #[inline]
    pub unsafe fn getNextAddrAsString(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getNextAddrAsString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean hasMore (); */
    #[inline]
    pub unsafe fn hasMore(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasMore)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void rewind (); */
    #[inline]
    pub unsafe fn rewind(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).rewind)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void reportUnusable (in uint16_t aPort); */
    #[inline]
    pub unsafe fn reportUnusable(&self, aPort: uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).reportUnusable)(self as *const _, aPort) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


