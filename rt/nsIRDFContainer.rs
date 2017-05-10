//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFContainer.idl
//


#[repr(C)]
pub struct nsIRDFContainer {
    vtable: *const nsIRDFContainerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFContainer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd4214e90, 0xfb94, 0x11d2,
            [0xbd, 0xd8, 0x00, 0x10, 0x4b, 0xde, 0x60, 0x48])
    }
}

unsafe impl RefCounted for nsIRDFContainer {
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
pub trait nsIRDFContainerCoerce {
    fn coerce_from(v: &nsIRDFContainer) -> &Self;
}

impl nsIRDFContainerCoerce for nsIRDFContainer {
    #[inline]
    fn coerce_from(v: &nsIRDFContainer) -> &Self {
        v
    }
}

impl nsIRDFContainer {
    #[inline]
    pub fn coerce<T: nsIRDFContainerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFContainer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFContainerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFContainer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFContainerVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIRDFDataSource DataSource; */
    pub get_DataSource: unsafe extern "C" fn (this: *const nsIRDFContainer, aDataSource: *mut *const nsIRDFDataSource) -> nsresult,

    /* readonly attribute nsIRDFResource Resource; */
    pub get_Resource: unsafe extern "C" fn (this: *const nsIRDFContainer, aResource: *mut *const nsIRDFResource) -> nsresult,

    /* void Init (in nsIRDFDataSource aDataSource, in nsIRDFResource aContainer); */
    pub Init: unsafe extern "C" fn (this: *const nsIRDFContainer, aDataSource: *const nsIRDFDataSource, aContainer: *const nsIRDFResource) -> nsresult,

    /* long GetCount (); */
    pub GetCount: unsafe extern "C" fn (this: *const nsIRDFContainer, _retval: *mut libc::int32_t) -> nsresult,

    /* nsISimpleEnumerator GetElements (); */
    pub GetElements: unsafe extern "C" fn (this: *const nsIRDFContainer, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* void AppendElement (in nsIRDFNode aElement); */
    pub AppendElement: unsafe extern "C" fn (this: *const nsIRDFContainer, aElement: *const nsIRDFNode) -> nsresult,

    /* void RemoveElement (in nsIRDFNode aElement, in boolean aRenumber); */
    pub RemoveElement: unsafe extern "C" fn (this: *const nsIRDFContainer, aElement: *const nsIRDFNode, aRenumber: bool) -> nsresult,

    /* void InsertElementAt (in nsIRDFNode aElement, in long aIndex, in boolean aRenumber); */
    pub InsertElementAt: unsafe extern "C" fn (this: *const nsIRDFContainer, aElement: *const nsIRDFNode, aIndex: libc::int32_t, aRenumber: bool) -> nsresult,

    /* nsIRDFNode RemoveElementAt (in long aIndex, in boolean aRenumber); */
    pub RemoveElementAt: unsafe extern "C" fn (this: *const nsIRDFContainer, aIndex: libc::int32_t, aRenumber: bool, _retval: *mut *const nsIRDFNode) -> nsresult,

    /* long IndexOf (in nsIRDFNode aElement); */
    pub IndexOf: unsafe extern "C" fn (this: *const nsIRDFContainer, aElement: *const nsIRDFNode, _retval: *mut libc::int32_t) -> nsresult,

}


impl nsIRDFContainer {
    /* readonly attribute nsIRDFDataSource DataSource; */
    #[inline]
    pub unsafe fn get_DataSource(&self, ) -> Result<Option<RefPtr<nsIRDFDataSource>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_DataSource)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIRDFResource Resource; */
    #[inline]
    pub unsafe fn get_Resource(&self, ) -> Result<Option<RefPtr<nsIRDFResource>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_Resource)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void Init (in nsIRDFDataSource aDataSource, in nsIRDFResource aContainer); */
    #[inline]
    pub unsafe fn Init(&self, aDataSource: Option<&nsIRDFDataSource>, aContainer: Option<&nsIRDFResource>) -> Result<(), nsresult> {

        match ((*self.vtable).Init)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _), aContainer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* long GetCount (); */
    #[inline]
    pub unsafe fn GetCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).GetCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISimpleEnumerator GetElements (); */
    #[inline]
    pub unsafe fn GetElements(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetElements)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void AppendElement (in nsIRDFNode aElement); */
    #[inline]
    pub unsafe fn AppendElement(&self, aElement: Option<&nsIRDFNode>) -> Result<(), nsresult> {

        match ((*self.vtable).AppendElement)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void RemoveElement (in nsIRDFNode aElement, in boolean aRenumber); */
    #[inline]
    pub unsafe fn RemoveElement(&self, aElement: Option<&nsIRDFNode>, aRenumber: bool) -> Result<(), nsresult> {

        match ((*self.vtable).RemoveElement)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), aRenumber) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void InsertElementAt (in nsIRDFNode aElement, in long aIndex, in boolean aRenumber); */
    #[inline]
    pub unsafe fn InsertElementAt(&self, aElement: Option<&nsIRDFNode>, aIndex: libc::int32_t, aRenumber: bool) -> Result<(), nsresult> {

        match ((*self.vtable).InsertElementAt)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), aIndex, aRenumber) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIRDFNode RemoveElementAt (in long aIndex, in boolean aRenumber); */
    #[inline]
    pub unsafe fn RemoveElementAt(&self, aIndex: libc::int32_t, aRenumber: bool) -> Result<Option<RefPtr<nsIRDFNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).RemoveElementAt)(self as *const _, aIndex, aRenumber, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* long IndexOf (in nsIRDFNode aElement); */
    #[inline]
    pub unsafe fn IndexOf(&self, aElement: Option<&nsIRDFNode>) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).IndexOf)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


