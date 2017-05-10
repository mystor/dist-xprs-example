//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIPlacesPendingOperation.idl
//


#[repr(C)]
pub struct mozIPlacesPendingOperation {
    vtable: *const mozIPlacesPendingOperationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIPlacesPendingOperation {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xebd31374, 0x3808, 0x40e4,
            [0x9e, 0x73, 0x30, 0x3b, 0xf7, 0x04, 0x67, 0xc3])
    }
}

unsafe impl RefCounted for mozIPlacesPendingOperation {
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
pub trait mozIPlacesPendingOperationCoerce {
    fn coerce_from(v: &mozIPlacesPendingOperation) -> &Self;
}

impl mozIPlacesPendingOperationCoerce for mozIPlacesPendingOperation {
    #[inline]
    fn coerce_from(v: &mozIPlacesPendingOperation) -> &Self {
        v
    }
}

impl mozIPlacesPendingOperation {
    #[inline]
    pub fn coerce<T: mozIPlacesPendingOperationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIPlacesPendingOperation {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIPlacesPendingOperationCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIPlacesPendingOperation) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIPlacesPendingOperationVTable {
    pub __base: nsISupportsVTable,

    /* void cancel (); */
    pub cancel: unsafe extern "C" fn (this: *const mozIPlacesPendingOperation) -> nsresult,

}


impl mozIPlacesPendingOperation {
    /* void cancel (); */
    #[inline]
    pub unsafe fn cancel(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cancel)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


