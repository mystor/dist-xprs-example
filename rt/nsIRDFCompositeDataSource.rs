//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFCompositeDataSource.idl
//


#[repr(C)]
pub struct nsIRDFCompositeDataSource {
    vtable: *const nsIRDFCompositeDataSourceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFCompositeDataSource {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x96343820, 0x307c, 0x11d2,
            [0xbc, 0x15, 0x00, 0x80, 0x5f, 0x91, 0x2f, 0xe7])
    }
}

unsafe impl RefCounted for nsIRDFCompositeDataSource {
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
pub trait nsIRDFCompositeDataSourceCoerce {
    fn coerce_from(v: &nsIRDFCompositeDataSource) -> &Self;
}

impl nsIRDFCompositeDataSourceCoerce for nsIRDFCompositeDataSource {
    #[inline]
    fn coerce_from(v: &nsIRDFCompositeDataSource) -> &Self {
        v
    }
}

impl nsIRDFCompositeDataSource {
    #[inline]
    pub fn coerce<T: nsIRDFCompositeDataSourceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFCompositeDataSource {
    type Target = nsIRDFDataSource;
    #[inline]
    fn deref(&self) -> &nsIRDFDataSource {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRDFDataSourceCoerce> nsIRDFCompositeDataSourceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFCompositeDataSource) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFCompositeDataSourceVTable {
    pub __base: nsIRDFDataSourceVTable,

    /* attribute boolean allowNegativeAssertions; */
    pub get_allowNegativeAssertions: unsafe extern "C" fn (this: *const nsIRDFCompositeDataSource, aAllowNegativeAssertions: *mut bool) -> nsresult,
    pub set_allowNegativeAssertions: unsafe extern "C" fn (this: *const nsIRDFCompositeDataSource, aAllowNegativeAssertions: bool) -> nsresult,

    /* attribute boolean coalesceDuplicateArcs; */
    pub get_coalesceDuplicateArcs: unsafe extern "C" fn (this: *const nsIRDFCompositeDataSource, aCoalesceDuplicateArcs: *mut bool) -> nsresult,
    pub set_coalesceDuplicateArcs: unsafe extern "C" fn (this: *const nsIRDFCompositeDataSource, aCoalesceDuplicateArcs: bool) -> nsresult,

    /* void AddDataSource (in nsIRDFDataSource aDataSource); */
    pub AddDataSource: unsafe extern "C" fn (this: *const nsIRDFCompositeDataSource, aDataSource: *const nsIRDFDataSource) -> nsresult,

    /* void RemoveDataSource (in nsIRDFDataSource aDataSource); */
    pub RemoveDataSource: unsafe extern "C" fn (this: *const nsIRDFCompositeDataSource, aDataSource: *const nsIRDFDataSource) -> nsresult,

    /* nsISimpleEnumerator GetDataSources (); */
    pub GetDataSources: unsafe extern "C" fn (this: *const nsIRDFCompositeDataSource, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

}


impl nsIRDFCompositeDataSource {
    /* attribute boolean allowNegativeAssertions; */
    #[inline]
    pub unsafe fn get_allowNegativeAssertions(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowNegativeAssertions)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowNegativeAssertions(&self, aAllowNegativeAssertions: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowNegativeAssertions)(self as *const _, aAllowNegativeAssertions) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean coalesceDuplicateArcs; */
    #[inline]
    pub unsafe fn get_coalesceDuplicateArcs(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_coalesceDuplicateArcs)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_coalesceDuplicateArcs(&self, aCoalesceDuplicateArcs: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_coalesceDuplicateArcs)(self as *const _, aCoalesceDuplicateArcs) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void AddDataSource (in nsIRDFDataSource aDataSource); */
    #[inline]
    pub unsafe fn AddDataSource(&self, aDataSource: Option<&nsIRDFDataSource>) -> Result<(), nsresult> {

        match ((*self.vtable).AddDataSource)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void RemoveDataSource (in nsIRDFDataSource aDataSource); */
    #[inline]
    pub unsafe fn RemoveDataSource(&self, aDataSource: Option<&nsIRDFDataSource>) -> Result<(), nsresult> {

        match ((*self.vtable).RemoveDataSource)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISimpleEnumerator GetDataSources (); */
    #[inline]
    pub unsafe fn GetDataSources(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetDataSources)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


