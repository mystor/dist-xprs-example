//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransaction.idl
//


#[repr(C)]
pub struct nsITransaction {
    vtable: *const nsITransactionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITransaction {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x58e330c1, 0x7b48, 0x11d2,
            [0x98, 0xb9, 0x00, 0x80, 0x5f, 0x29, 0x7d, 0x89])
    }
}

unsafe impl RefCounted for nsITransaction {
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
pub trait nsITransactionCoerce {
    fn coerce_from(v: &nsITransaction) -> &Self;
}

impl nsITransactionCoerce for nsITransaction {
    #[inline]
    fn coerce_from(v: &nsITransaction) -> &Self {
        v
    }
}

impl nsITransaction {
    #[inline]
    pub fn coerce<T: nsITransactionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITransaction {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITransactionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransaction) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITransactionVTable {
    pub __base: nsISupportsVTable,

    /* void doTransaction (); */
    pub doTransaction: unsafe extern "C" fn (this: *const nsITransaction) -> nsresult,

    /* void undoTransaction (); */
    pub undoTransaction: unsafe extern "C" fn (this: *const nsITransaction) -> nsresult,

    /* void redoTransaction (); */
    pub redoTransaction: unsafe extern "C" fn (this: *const nsITransaction) -> nsresult,

    /* readonly attribute boolean isTransient; */
    pub get_isTransient: unsafe extern "C" fn (this: *const nsITransaction, aIsTransient: *mut bool) -> nsresult,

    /* boolean merge (in nsITransaction aTransaction); */
    pub merge: unsafe extern "C" fn (this: *const nsITransaction, aTransaction: *const nsITransaction, _retval: *mut bool) -> nsresult,

}


impl nsITransaction {
    /* void doTransaction (); */
    #[inline]
    pub unsafe fn doTransaction(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).doTransaction)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void undoTransaction (); */
    #[inline]
    pub unsafe fn undoTransaction(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).undoTransaction)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void redoTransaction (); */
    #[inline]
    pub unsafe fn redoTransaction(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).redoTransaction)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean isTransient; */
    #[inline]
    pub unsafe fn get_isTransient(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isTransient)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean merge (in nsITransaction aTransaction); */
    #[inline]
    pub unsafe fn merge(&self, aTransaction: Option<&nsITransaction>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).merge)(self as *const _, aTransaction.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


