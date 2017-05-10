//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFPurgeableDataSource.idl
//


#[repr(C)]
pub struct nsIRDFPurgeableDataSource {
    vtable: *const nsIRDFPurgeableDataSourceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFPurgeableDataSource {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x951700f0, 0xfed0, 0x11d2,
            [0xbd, 0xd9, 0x00, 0x10, 0x4b, 0xde, 0x60, 0x48])
    }
}

unsafe impl RefCounted for nsIRDFPurgeableDataSource {
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
pub trait nsIRDFPurgeableDataSourceCoerce {
    fn coerce_from(v: &nsIRDFPurgeableDataSource) -> &Self;
}

impl nsIRDFPurgeableDataSourceCoerce for nsIRDFPurgeableDataSource {
    #[inline]
    fn coerce_from(v: &nsIRDFPurgeableDataSource) -> &Self {
        v
    }
}

impl nsIRDFPurgeableDataSource {
    #[inline]
    pub fn coerce<T: nsIRDFPurgeableDataSourceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFPurgeableDataSource {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFPurgeableDataSourceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFPurgeableDataSource) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFPurgeableDataSourceVTable {
    pub __base: nsISupportsVTable,

    /* boolean Mark (in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget, in boolean aTruthValue); */
    pub Mark: unsafe extern "C" fn (this: *const nsIRDFPurgeableDataSource, aSource: *const nsIRDFResource, aProperty: *const nsIRDFResource, aTarget: *const nsIRDFNode, aTruthValue: bool, _retval: *mut bool) -> nsresult,

    /* void Sweep (); */
    pub Sweep: unsafe extern "C" fn (this: *const nsIRDFPurgeableDataSource) -> nsresult,

}


impl nsIRDFPurgeableDataSource {
    /* boolean Mark (in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget, in boolean aTruthValue); */
    #[inline]
    pub unsafe fn Mark(&self, aSource: Option<&nsIRDFResource>, aProperty: Option<&nsIRDFResource>, aTarget: Option<&nsIRDFNode>, aTruthValue: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).Mark)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _), aProperty.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _), aTruthValue, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void Sweep (); */
    #[inline]
    pub unsafe fn Sweep(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).Sweep)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


