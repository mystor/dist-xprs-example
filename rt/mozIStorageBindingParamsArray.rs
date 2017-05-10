//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageBindingParamsArray.idl
//


#[repr(C)]
pub struct mozIStorageBindingParamsArray {
    vtable: *const mozIStorageBindingParamsArrayVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageBindingParamsArray {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x67eea5c3, 0x4881, 0x41ff,
            [0xb0, 0xfe, 0x09, 0xf2, 0x35, 0x6a, 0xea, 0xdb])
    }
}

unsafe impl RefCounted for mozIStorageBindingParamsArray {
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
pub trait mozIStorageBindingParamsArrayCoerce {
    fn coerce_from(v: &mozIStorageBindingParamsArray) -> &Self;
}

impl mozIStorageBindingParamsArrayCoerce for mozIStorageBindingParamsArray {
    #[inline]
    fn coerce_from(v: &mozIStorageBindingParamsArray) -> &Self {
        v
    }
}

impl mozIStorageBindingParamsArray {
    #[inline]
    pub fn coerce<T: mozIStorageBindingParamsArrayCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageBindingParamsArray {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStorageBindingParamsArrayCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageBindingParamsArray) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageBindingParamsArrayVTable {
    pub __base: nsISupportsVTable,

    /* mozIStorageBindingParams newBindingParams (); */
    pub newBindingParams: unsafe extern "C" fn (this: *const mozIStorageBindingParamsArray, _retval: *mut *const mozIStorageBindingParams) -> nsresult,

    /* void addParams (in mozIStorageBindingParams aParameters); */
    pub addParams: unsafe extern "C" fn (this: *const mozIStorageBindingParamsArray, aParameters: *const mozIStorageBindingParams) -> nsresult,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const mozIStorageBindingParamsArray, aLength: *mut libc::uint32_t) -> nsresult,

}


impl mozIStorageBindingParamsArray {
    /* mozIStorageBindingParams newBindingParams (); */
    #[inline]
    pub unsafe fn newBindingParams(&self, ) -> Result<Option<RefPtr<mozIStorageBindingParams>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newBindingParams)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void addParams (in mozIStorageBindingParams aParameters); */
    #[inline]
    pub unsafe fn addParams(&self, aParameters: Option<&mozIStorageBindingParams>) -> Result<(), nsresult> {

        match ((*self.vtable).addParams)(self as *const _, aParameters.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


