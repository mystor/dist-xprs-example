//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageCompletionCallback.idl
//


#[repr(C)]
pub struct mozIStorageCompletionCallback {
    vtable: *const mozIStorageCompletionCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageCompletionCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8cbf2dc2, 0x91e0, 0x44bc,
            [0x98, 0x4f, 0x55, 0x36, 0x38, 0x41, 0x20, 0x71])
    }
}

unsafe impl RefCounted for mozIStorageCompletionCallback {
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
pub trait mozIStorageCompletionCallbackCoerce {
    fn coerce_from(v: &mozIStorageCompletionCallback) -> &Self;
}

impl mozIStorageCompletionCallbackCoerce for mozIStorageCompletionCallback {
    #[inline]
    fn coerce_from(v: &mozIStorageCompletionCallback) -> &Self {
        v
    }
}

impl mozIStorageCompletionCallback {
    #[inline]
    pub fn coerce<T: mozIStorageCompletionCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageCompletionCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStorageCompletionCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageCompletionCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageCompletionCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void complete (in nsresult status, [optional] in nsISupports value); */
    pub complete: unsafe extern "C" fn (this: *const mozIStorageCompletionCallback, status: nsresult, value: *const nsISupports) -> nsresult,

}


impl mozIStorageCompletionCallback {
    /* void complete (in nsresult status, [optional] in nsISupports value); */
    #[inline]
    pub unsafe fn complete(&self, status: nsresult, value: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).complete)(self as *const _, status, value.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


