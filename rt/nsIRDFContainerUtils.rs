//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFContainerUtils.idl
//


#[repr(C)]
pub struct nsIRDFContainerUtils {
    vtable: *const nsIRDFContainerUtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFContainerUtils {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd4214e91, 0xfb94, 0x11d2,
            [0xbd, 0xd8, 0x00, 0x10, 0x4b, 0xde, 0x60, 0x48])
    }
}

unsafe impl RefCounted for nsIRDFContainerUtils {
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
pub trait nsIRDFContainerUtilsCoerce {
    fn coerce_from(v: &nsIRDFContainerUtils) -> &Self;
}

impl nsIRDFContainerUtilsCoerce for nsIRDFContainerUtils {
    #[inline]
    fn coerce_from(v: &nsIRDFContainerUtils) -> &Self {
        v
    }
}

impl nsIRDFContainerUtils {
    #[inline]
    pub fn coerce<T: nsIRDFContainerUtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFContainerUtils {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFContainerUtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFContainerUtils) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFContainerUtilsVTable {
    pub __base: nsISupportsVTable,

    /* boolean IsOrdinalProperty (in nsIRDFResource aProperty); */
    pub IsOrdinalProperty: unsafe extern "C" fn (this: *const nsIRDFContainerUtils, aProperty: *const nsIRDFResource, _retval: *mut bool) -> nsresult,

    /* nsIRDFResource IndexToOrdinalResource (in long aIndex); */
    pub IndexToOrdinalResource: unsafe extern "C" fn (this: *const nsIRDFContainerUtils, aIndex: libc::int32_t, _retval: *mut *const nsIRDFResource) -> nsresult,

    /* long OrdinalResourceToIndex (in nsIRDFResource aOrdinal); */
    pub OrdinalResourceToIndex: unsafe extern "C" fn (this: *const nsIRDFContainerUtils, aOrdinal: *const nsIRDFResource, _retval: *mut libc::int32_t) -> nsresult,

    /* boolean IsContainer (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    pub IsContainer: unsafe extern "C" fn (this: *const nsIRDFContainerUtils, aDataSource: *const nsIRDFDataSource, aResource: *const nsIRDFResource, _retval: *mut bool) -> nsresult,

    /* boolean IsEmpty (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    pub IsEmpty: unsafe extern "C" fn (this: *const nsIRDFContainerUtils, aDataSource: *const nsIRDFDataSource, aResource: *const nsIRDFResource, _retval: *mut bool) -> nsresult,

    /* boolean IsBag (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    pub IsBag: unsafe extern "C" fn (this: *const nsIRDFContainerUtils, aDataSource: *const nsIRDFDataSource, aResource: *const nsIRDFResource, _retval: *mut bool) -> nsresult,

    /* boolean IsSeq (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    pub IsSeq: unsafe extern "C" fn (this: *const nsIRDFContainerUtils, aDataSource: *const nsIRDFDataSource, aResource: *const nsIRDFResource, _retval: *mut bool) -> nsresult,

    /* boolean IsAlt (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    pub IsAlt: unsafe extern "C" fn (this: *const nsIRDFContainerUtils, aDataSource: *const nsIRDFDataSource, aResource: *const nsIRDFResource, _retval: *mut bool) -> nsresult,

    /* nsIRDFContainer MakeBag (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    pub MakeBag: unsafe extern "C" fn (this: *const nsIRDFContainerUtils, aDataSource: *const nsIRDFDataSource, aResource: *const nsIRDFResource, _retval: *mut *const nsIRDFContainer) -> nsresult,

    /* nsIRDFContainer MakeSeq (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    pub MakeSeq: unsafe extern "C" fn (this: *const nsIRDFContainerUtils, aDataSource: *const nsIRDFDataSource, aResource: *const nsIRDFResource, _retval: *mut *const nsIRDFContainer) -> nsresult,

    /* nsIRDFContainer MakeAlt (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    pub MakeAlt: unsafe extern "C" fn (this: *const nsIRDFContainerUtils, aDataSource: *const nsIRDFDataSource, aResource: *const nsIRDFResource, _retval: *mut *const nsIRDFContainer) -> nsresult,

    /* long indexOf (in nsIRDFDataSource aDataSource, in nsIRDFResource aContainer, in nsIRDFNode aElement); */
    pub indexOf: unsafe extern "C" fn (this: *const nsIRDFContainerUtils, aDataSource: *const nsIRDFDataSource, aContainer: *const nsIRDFResource, aElement: *const nsIRDFNode, _retval: *mut libc::int32_t) -> nsresult,

}


impl nsIRDFContainerUtils {
    /* boolean IsOrdinalProperty (in nsIRDFResource aProperty); */
    #[inline]
    pub unsafe fn IsOrdinalProperty(&self, aProperty: Option<&nsIRDFResource>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).IsOrdinalProperty)(self as *const _, aProperty.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIRDFResource IndexToOrdinalResource (in long aIndex); */
    #[inline]
    pub unsafe fn IndexToOrdinalResource(&self, aIndex: libc::int32_t) -> Result<Option<RefPtr<nsIRDFResource>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).IndexToOrdinalResource)(self as *const _, aIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* long OrdinalResourceToIndex (in nsIRDFResource aOrdinal); */
    #[inline]
    pub unsafe fn OrdinalResourceToIndex(&self, aOrdinal: Option<&nsIRDFResource>) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).OrdinalResourceToIndex)(self as *const _, aOrdinal.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean IsContainer (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    #[inline]
    pub unsafe fn IsContainer(&self, aDataSource: Option<&nsIRDFDataSource>, aResource: Option<&nsIRDFResource>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).IsContainer)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _), aResource.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean IsEmpty (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    #[inline]
    pub unsafe fn IsEmpty(&self, aDataSource: Option<&nsIRDFDataSource>, aResource: Option<&nsIRDFResource>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).IsEmpty)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _), aResource.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean IsBag (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    #[inline]
    pub unsafe fn IsBag(&self, aDataSource: Option<&nsIRDFDataSource>, aResource: Option<&nsIRDFResource>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).IsBag)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _), aResource.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean IsSeq (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    #[inline]
    pub unsafe fn IsSeq(&self, aDataSource: Option<&nsIRDFDataSource>, aResource: Option<&nsIRDFResource>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).IsSeq)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _), aResource.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean IsAlt (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    #[inline]
    pub unsafe fn IsAlt(&self, aDataSource: Option<&nsIRDFDataSource>, aResource: Option<&nsIRDFResource>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).IsAlt)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _), aResource.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIRDFContainer MakeBag (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    #[inline]
    pub unsafe fn MakeBag(&self, aDataSource: Option<&nsIRDFDataSource>, aResource: Option<&nsIRDFResource>) -> Result<Option<RefPtr<nsIRDFContainer>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).MakeBag)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _), aResource.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIRDFContainer MakeSeq (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    #[inline]
    pub unsafe fn MakeSeq(&self, aDataSource: Option<&nsIRDFDataSource>, aResource: Option<&nsIRDFResource>) -> Result<Option<RefPtr<nsIRDFContainer>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).MakeSeq)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _), aResource.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIRDFContainer MakeAlt (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
    #[inline]
    pub unsafe fn MakeAlt(&self, aDataSource: Option<&nsIRDFDataSource>, aResource: Option<&nsIRDFResource>) -> Result<Option<RefPtr<nsIRDFContainer>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).MakeAlt)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _), aResource.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* long indexOf (in nsIRDFDataSource aDataSource, in nsIRDFResource aContainer, in nsIRDFNode aElement); */
    #[inline]
    pub unsafe fn indexOf(&self, aDataSource: Option<&nsIRDFDataSource>, aContainer: Option<&nsIRDFResource>, aElement: Option<&nsIRDFNode>) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).indexOf)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _), aContainer.map_or(::std::ptr::null(), |x| x as *const _), aElement.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


