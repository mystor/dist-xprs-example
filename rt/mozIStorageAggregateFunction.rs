//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageAggregateFunction.idl
//


#[repr(C)]
pub struct mozIStorageAggregateFunction {
    vtable: *const mozIStorageAggregateFunctionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageAggregateFunction {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x763217b7, 0x3123, 0x11da,
            [0x91, 0x8d, 0x00, 0x03, 0x47, 0x41, 0x2e, 0x16])
    }
}

unsafe impl RefCounted for mozIStorageAggregateFunction {
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
pub trait mozIStorageAggregateFunctionCoerce {
    fn coerce_from(v: &mozIStorageAggregateFunction) -> &Self;
}

impl mozIStorageAggregateFunctionCoerce for mozIStorageAggregateFunction {
    #[inline]
    fn coerce_from(v: &mozIStorageAggregateFunction) -> &Self {
        v
    }
}

impl mozIStorageAggregateFunction {
    #[inline]
    pub fn coerce<T: mozIStorageAggregateFunctionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageAggregateFunction {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStorageAggregateFunctionCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageAggregateFunction) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageAggregateFunctionVTable {
    pub __base: nsISupportsVTable,

    /* void onStep (in mozIStorageValueArray aFunctionArguments); */
    pub onStep: unsafe extern "C" fn (this: *const mozIStorageAggregateFunction, aFunctionArguments: *const mozIStorageValueArray) -> nsresult,

    /* nsIVariant onFinal (); */
    pub onFinal: unsafe extern "C" fn (this: *const mozIStorageAggregateFunction, _retval: *mut *const nsIVariant) -> nsresult,

}


impl mozIStorageAggregateFunction {
    /* void onStep (in mozIStorageValueArray aFunctionArguments); */
    #[inline]
    pub unsafe fn onStep(&self, aFunctionArguments: Option<&mozIStorageValueArray>) -> Result<(), nsresult> {

        match ((*self.vtable).onStep)(self as *const _, aFunctionArguments.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIVariant onFinal (); */
    #[inline]
    pub unsafe fn onFinal(&self, ) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).onFinal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


