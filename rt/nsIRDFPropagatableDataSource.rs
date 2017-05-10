//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFPropagatableDataSource.idl
//


#[repr(C)]
pub struct nsIRDFPropagatableDataSource {
    vtable: *const nsIRDFPropagatableDataSourceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFPropagatableDataSource {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5a9b4770, 0x9fcb, 0x4307,
            [0xa1, 0x2e, 0x4b, 0x67, 0x08, 0xe7, 0x8b, 0x97])
    }
}

unsafe impl RefCounted for nsIRDFPropagatableDataSource {
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
pub trait nsIRDFPropagatableDataSourceCoerce {
    fn coerce_from(v: &nsIRDFPropagatableDataSource) -> &Self;
}

impl nsIRDFPropagatableDataSourceCoerce for nsIRDFPropagatableDataSource {
    #[inline]
    fn coerce_from(v: &nsIRDFPropagatableDataSource) -> &Self {
        v
    }
}

impl nsIRDFPropagatableDataSource {
    #[inline]
    pub fn coerce<T: nsIRDFPropagatableDataSourceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFPropagatableDataSource {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFPropagatableDataSourceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFPropagatableDataSource) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFPropagatableDataSourceVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean propagateChanges; */
    pub get_propagateChanges: unsafe extern "C" fn (this: *const nsIRDFPropagatableDataSource, aPropagateChanges: *mut bool) -> nsresult,
    pub set_propagateChanges: unsafe extern "C" fn (this: *const nsIRDFPropagatableDataSource, aPropagateChanges: bool) -> nsresult,

}


impl nsIRDFPropagatableDataSource {
    /* attribute boolean propagateChanges; */
    #[inline]
    pub unsafe fn get_propagateChanges(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_propagateChanges)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_propagateChanges(&self, aPropagateChanges: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_propagateChanges)(self as *const _, aPropagateChanges) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


