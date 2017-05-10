//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageStatementCallback.idl
//


pub mod mozIStorageStatementCallback_consts {
    pub const REASON_FINISHED: i64 = 0;
    pub const REASON_CANCELED: i64 = 1;
    pub const REASON_ERROR: i64 = 2;
}


#[repr(C)]
pub struct mozIStorageStatementCallback {
    vtable: *const mozIStorageStatementCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageStatementCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x29383d00, 0xd8c4, 0x4ddd,
            [0x9f, 0x8b, 0xc2, 0xfe, 0xb0, 0xf2, 0xfc, 0xfa])
    }
}

unsafe impl RefCounted for mozIStorageStatementCallback {
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
pub trait mozIStorageStatementCallbackCoerce {
    fn coerce_from(v: &mozIStorageStatementCallback) -> &Self;
}

impl mozIStorageStatementCallbackCoerce for mozIStorageStatementCallback {
    #[inline]
    fn coerce_from(v: &mozIStorageStatementCallback) -> &Self {
        v
    }
}

impl mozIStorageStatementCallback {
    #[inline]
    pub fn coerce<T: mozIStorageStatementCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageStatementCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStorageStatementCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageStatementCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageStatementCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void handleResult (in mozIStorageResultSet aResultSet); */
    pub handleResult: unsafe extern "C" fn (this: *const mozIStorageStatementCallback, aResultSet: *const mozIStorageResultSet) -> nsresult,

    /* void handleError (in mozIStorageError aError); */
    pub handleError: unsafe extern "C" fn (this: *const mozIStorageStatementCallback, aError: *const mozIStorageError) -> nsresult,

    /* void handleCompletion (in unsigned short aReason); */
    pub handleCompletion: unsafe extern "C" fn (this: *const mozIStorageStatementCallback, aReason: libc::uint16_t) -> nsresult,

}


impl mozIStorageStatementCallback {
    /* void handleResult (in mozIStorageResultSet aResultSet); */
    #[inline]
    pub unsafe fn handleResult(&self, aResultSet: Option<&mozIStorageResultSet>) -> Result<(), nsresult> {

        match ((*self.vtable).handleResult)(self as *const _, aResultSet.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleError (in mozIStorageError aError); */
    #[inline]
    pub unsafe fn handleError(&self, aError: Option<&mozIStorageError>) -> Result<(), nsresult> {

        match ((*self.vtable).handleError)(self as *const _, aError.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleCompletion (in unsigned short aReason); */
    #[inline]
    pub unsafe fn handleCompletion(&self, aReason: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).handleCompletion)(self as *const _, aReason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


