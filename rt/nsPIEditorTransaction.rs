//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsPIEditorTransaction.idl
//


#[repr(C)]
pub struct nsPIEditorTransaction {
    vtable: *const nsPIEditorTransactionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsPIEditorTransaction {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4f18ada2, 0x0ddc, 0x11d5,
            [0x9d, 0x3a, 0x00, 0x60, 0xb0, 0xf8, 0xba, 0xff])
    }
}

unsafe impl RefCounted for nsPIEditorTransaction {
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
pub trait nsPIEditorTransactionCoerce {
    fn coerce_from(v: &nsPIEditorTransaction) -> &Self;
}

impl nsPIEditorTransactionCoerce for nsPIEditorTransaction {
    #[inline]
    fn coerce_from(v: &nsPIEditorTransaction) -> &Self {
        v
    }
}

impl nsPIEditorTransaction {
    #[inline]
    pub fn coerce<T: nsPIEditorTransactionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsPIEditorTransaction {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsPIEditorTransactionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsPIEditorTransaction) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsPIEditorTransactionVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString txnDescription; */
    pub get_txnDescription: unsafe extern "C" fn (this: *const nsPIEditorTransaction, aTxnDescription: *mut nsAString) -> nsresult,

}


impl nsPIEditorTransaction {
    /* readonly attribute DOMString txnDescription; */
    #[inline]
    pub unsafe fn get_txnDescription(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_txnDescription)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


