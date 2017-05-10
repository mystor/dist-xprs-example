//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageFunction.idl
//


#[repr(C)]
pub struct mozIStorageFunction {
    vtable: *const mozIStorageFunctionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageFunction {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9ff02465, 0x21cb, 0x49f3,
            [0xb9, 0x75, 0x7d, 0x5b, 0x38, 0xce, 0xec, 0x73])
    }
}

unsafe impl RefCounted for mozIStorageFunction {
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
pub trait mozIStorageFunctionCoerce {
    fn coerce_from(v: &mozIStorageFunction) -> &Self;
}

impl mozIStorageFunctionCoerce for mozIStorageFunction {
    #[inline]
    fn coerce_from(v: &mozIStorageFunction) -> &Self {
        v
    }
}

impl mozIStorageFunction {
    #[inline]
    pub fn coerce<T: mozIStorageFunctionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageFunction {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStorageFunctionCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageFunction) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageFunctionVTable {
    pub __base: nsISupportsVTable,

    /* nsIVariant onFunctionCall (in mozIStorageValueArray aFunctionArguments); */
    pub onFunctionCall: unsafe extern "C" fn (this: *const mozIStorageFunction, aFunctionArguments: *const mozIStorageValueArray, _retval: *mut *const nsIVariant) -> nsresult,

}


impl mozIStorageFunction {
    /* nsIVariant onFunctionCall (in mozIStorageValueArray aFunctionArguments); */
    #[inline]
    pub unsafe fn onFunctionCall(&self, aFunctionArguments: Option<&mozIStorageValueArray>) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).onFunctionCall)(self as *const _, aFunctionArguments.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


