//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/txIFunctionEvaluationContext.idl
//


#[repr(C)]
pub struct txIFunctionEvaluationContext {
    vtable: *const txIFunctionEvaluationContextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for txIFunctionEvaluationContext {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0ecbb00c, 0x6a78, 0x11d9,
            [0x97, 0x91, 0x00, 0x0a, 0x95, 0xdc, 0x23, 0x4c])
    }
}

unsafe impl RefCounted for txIFunctionEvaluationContext {
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
pub trait txIFunctionEvaluationContextCoerce {
    fn coerce_from(v: &txIFunctionEvaluationContext) -> &Self;
}

impl txIFunctionEvaluationContextCoerce for txIFunctionEvaluationContext {
    #[inline]
    fn coerce_from(v: &txIFunctionEvaluationContext) -> &Self {
        v
    }
}

impl txIFunctionEvaluationContext {
    #[inline]
    pub fn coerce<T: txIFunctionEvaluationContextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for txIFunctionEvaluationContext {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> txIFunctionEvaluationContextCoerce for T {
    #[inline]
    fn coerce_from(v: &txIFunctionEvaluationContext) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct txIFunctionEvaluationContextVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute uint32_t position; */
    pub get_position: unsafe extern "C" fn (this: *const txIFunctionEvaluationContext, aPosition: *mut uint32_t) -> nsresult,

    /* readonly attribute uint32_t size; */
    pub get_size: unsafe extern "C" fn (this: *const txIFunctionEvaluationContext, aSize: *mut uint32_t) -> nsresult,

    /* readonly attribute nsIDOMNode contextNode; */
    pub get_contextNode: unsafe extern "C" fn (this: *const txIFunctionEvaluationContext, aContextNode: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute nsISupports state; */
    pub get_state: unsafe extern "C" fn (this: *const txIFunctionEvaluationContext, aState: *mut *const nsISupports) -> nsresult,

}


impl txIFunctionEvaluationContext {
    /* readonly attribute uint32_t position; */
    #[inline]
    pub unsafe fn get_position(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_position)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t size; */
    #[inline]
    pub unsafe fn get_size(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_size)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMNode contextNode; */
    #[inline]
    pub unsafe fn get_contextNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_contextNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsISupports state; */
    #[inline]
    pub unsafe fn get_state(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_state)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


