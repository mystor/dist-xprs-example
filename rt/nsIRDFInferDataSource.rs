//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFInferDataSource.idl
//


#[repr(C)]
pub struct nsIRDFInferDataSource {
    vtable: *const nsIRDFInferDataSourceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFInferDataSource {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2b04860f, 0x4017, 0x40f6,
            [0x8a, 0x57, 0x78, 0x4a, 0x1e, 0x35, 0x07, 0x7a])
    }
}

unsafe impl RefCounted for nsIRDFInferDataSource {
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
pub trait nsIRDFInferDataSourceCoerce {
    fn coerce_from(v: &nsIRDFInferDataSource) -> &Self;
}

impl nsIRDFInferDataSourceCoerce for nsIRDFInferDataSource {
    #[inline]
    fn coerce_from(v: &nsIRDFInferDataSource) -> &Self {
        v
    }
}

impl nsIRDFInferDataSource {
    #[inline]
    pub fn coerce<T: nsIRDFInferDataSourceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFInferDataSource {
    type Target = nsIRDFDataSource;
    #[inline]
    fn deref(&self) -> &nsIRDFDataSource {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRDFDataSourceCoerce> nsIRDFInferDataSourceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFInferDataSource) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFInferDataSourceVTable {
    pub __base: nsIRDFDataSourceVTable,

    /* attribute nsIRDFDataSource baseDataSource; */
    pub get_baseDataSource: unsafe extern "C" fn (this: *const nsIRDFInferDataSource, aBaseDataSource: *mut *const nsIRDFDataSource) -> nsresult,
    pub set_baseDataSource: unsafe extern "C" fn (this: *const nsIRDFInferDataSource, aBaseDataSource: *const nsIRDFDataSource) -> nsresult,

}


impl nsIRDFInferDataSource {
    /* attribute nsIRDFDataSource baseDataSource; */
    #[inline]
    pub unsafe fn get_baseDataSource(&self, ) -> Result<Option<RefPtr<nsIRDFDataSource>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_baseDataSource)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_baseDataSource(&self, aBaseDataSource: Option<&nsIRDFDataSource>) -> Result<(), nsresult> {

        match ((*self.vtable).set_baseDataSource)(self as *const _, aBaseDataSource.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


