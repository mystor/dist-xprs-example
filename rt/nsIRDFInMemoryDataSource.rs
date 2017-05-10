//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFInMemoryDataSource.idl
//


#[repr(C)]
pub struct nsIRDFInMemoryDataSource {
    vtable: *const nsIRDFInMemoryDataSourceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFInMemoryDataSource {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x17c4e0aa, 0x1dd2, 0x11b2,
            [0x80, 0x29, 0xbf, 0x6f, 0x66, 0x8d, 0xe5, 0x00])
    }
}

unsafe impl RefCounted for nsIRDFInMemoryDataSource {
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
pub trait nsIRDFInMemoryDataSourceCoerce {
    fn coerce_from(v: &nsIRDFInMemoryDataSource) -> &Self;
}

impl nsIRDFInMemoryDataSourceCoerce for nsIRDFInMemoryDataSource {
    #[inline]
    fn coerce_from(v: &nsIRDFInMemoryDataSource) -> &Self {
        v
    }
}

impl nsIRDFInMemoryDataSource {
    #[inline]
    pub fn coerce<T: nsIRDFInMemoryDataSourceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFInMemoryDataSource {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFInMemoryDataSourceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFInMemoryDataSource) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFInMemoryDataSourceVTable {
    pub __base: nsISupportsVTable,

    /* void EnsureFastContainment (in nsIRDFResource aSource); */
    pub EnsureFastContainment: unsafe extern "C" fn (this: *const nsIRDFInMemoryDataSource, aSource: *const nsIRDFResource) -> nsresult,

}


impl nsIRDFInMemoryDataSource {
    /* void EnsureFastContainment (in nsIRDFResource aSource); */
    #[inline]
    pub unsafe fn EnsureFastContainment(&self, aSource: Option<&nsIRDFResource>) -> Result<(), nsresult> {

        match ((*self.vtable).EnsureFastContainment)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


