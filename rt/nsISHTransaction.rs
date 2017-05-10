//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISHTransaction.idl
//


#[repr(C)]
pub struct nsISHTransaction {
    vtable: *const nsISHTransactionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISHTransaction {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2edf705f, 0xd252, 0x4971,
            [0x9f, 0x09, 0x71, 0xdd, 0x0f, 0x76, 0x0d, 0xc6])
    }
}

unsafe impl RefCounted for nsISHTransaction {
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
pub trait nsISHTransactionCoerce {
    fn coerce_from(v: &nsISHTransaction) -> &Self;
}

impl nsISHTransactionCoerce for nsISHTransaction {
    #[inline]
    fn coerce_from(v: &nsISHTransaction) -> &Self {
        v
    }
}

impl nsISHTransaction {
    #[inline]
    pub fn coerce<T: nsISHTransactionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISHTransaction {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISHTransactionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISHTransaction) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISHTransactionVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsISHEntry sHEntry; */
    pub get_sHEntry: unsafe extern "C" fn (this: *const nsISHTransaction, aSHEntry: *mut *const nsISHEntry) -> nsresult,
    pub set_sHEntry: unsafe extern "C" fn (this: *const nsISHTransaction, aSHEntry: *const nsISHEntry) -> nsresult,

    /* attribute nsISHTransaction prev; */
    pub get_prev: unsafe extern "C" fn (this: *const nsISHTransaction, aPrev: *mut *const nsISHTransaction) -> nsresult,
    pub set_prev: unsafe extern "C" fn (this: *const nsISHTransaction, aPrev: *const nsISHTransaction) -> nsresult,

    /* attribute nsISHTransaction next; */
    pub get_next: unsafe extern "C" fn (this: *const nsISHTransaction, aNext: *mut *const nsISHTransaction) -> nsresult,
    pub set_next: unsafe extern "C" fn (this: *const nsISHTransaction, aNext: *const nsISHTransaction) -> nsresult,

    /* attribute boolean persist; */
    pub get_persist: unsafe extern "C" fn (this: *const nsISHTransaction, aPersist: *mut bool) -> nsresult,
    pub set_persist: unsafe extern "C" fn (this: *const nsISHTransaction, aPersist: bool) -> nsresult,

    /* void create (in nsISHEntry aSHEntry, in nsISHTransaction aPrev); */
    pub create: unsafe extern "C" fn (this: *const nsISHTransaction, aSHEntry: *const nsISHEntry, aPrev: *const nsISHTransaction) -> nsresult,

}


impl nsISHTransaction {
    /* attribute nsISHEntry sHEntry; */
    #[inline]
    pub unsafe fn get_sHEntry(&self, ) -> Result<Option<RefPtr<nsISHEntry>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_sHEntry)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_sHEntry(&self, aSHEntry: Option<&nsISHEntry>) -> Result<(), nsresult> {

        match ((*self.vtable).set_sHEntry)(self as *const _, aSHEntry.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsISHTransaction prev; */
    #[inline]
    pub unsafe fn get_prev(&self, ) -> Result<Option<RefPtr<nsISHTransaction>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_prev)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_prev(&self, aPrev: Option<&nsISHTransaction>) -> Result<(), nsresult> {

        match ((*self.vtable).set_prev)(self as *const _, aPrev.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsISHTransaction next; */
    #[inline]
    pub unsafe fn get_next(&self, ) -> Result<Option<RefPtr<nsISHTransaction>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_next)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_next(&self, aNext: Option<&nsISHTransaction>) -> Result<(), nsresult> {

        match ((*self.vtable).set_next)(self as *const _, aNext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean persist; */
    #[inline]
    pub unsafe fn get_persist(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_persist)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_persist(&self, aPersist: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_persist)(self as *const _, aPersist) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void create (in nsISHEntry aSHEntry, in nsISHTransaction aPrev); */
    #[inline]
    pub unsafe fn create(&self, aSHEntry: Option<&nsISHEntry>, aPrev: Option<&nsISHTransaction>) -> Result<(), nsresult> {

        match ((*self.vtable).create)(self as *const _, aSHEntry.map_or(::std::ptr::null(), |x| x as *const _), aPrev.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


