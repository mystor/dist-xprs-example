//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFObserver.idl
//


#[repr(C)]
pub struct nsIRDFObserver {
    vtable: *const nsIRDFObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3cc75360, 0x484a, 0x11d2,
            [0xbc, 0x16, 0x00, 0x80, 0x5f, 0x91, 0x2f, 0xe7])
    }
}

unsafe impl RefCounted for nsIRDFObserver {
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
pub trait nsIRDFObserverCoerce {
    fn coerce_from(v: &nsIRDFObserver) -> &Self;
}

impl nsIRDFObserverCoerce for nsIRDFObserver {
    #[inline]
    fn coerce_from(v: &nsIRDFObserver) -> &Self {
        v
    }
}

impl nsIRDFObserver {
    #[inline]
    pub fn coerce<T: nsIRDFObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onAssert (in nsIRDFDataSource aDataSource, in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget); */
    pub onAssert: unsafe extern "C" fn (this: *const nsIRDFObserver, aDataSource: *const nsIRDFDataSource, aSource: *const nsIRDFResource, aProperty: *const nsIRDFResource, aTarget: *const nsIRDFNode) -> nsresult,

    /* void onUnassert (in nsIRDFDataSource aDataSource, in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget); */
    pub onUnassert: unsafe extern "C" fn (this: *const nsIRDFObserver, aDataSource: *const nsIRDFDataSource, aSource: *const nsIRDFResource, aProperty: *const nsIRDFResource, aTarget: *const nsIRDFNode) -> nsresult,

    /* void onChange (in nsIRDFDataSource aDataSource, in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aOldTarget, in nsIRDFNode aNewTarget); */
    pub onChange: unsafe extern "C" fn (this: *const nsIRDFObserver, aDataSource: *const nsIRDFDataSource, aSource: *const nsIRDFResource, aProperty: *const nsIRDFResource, aOldTarget: *const nsIRDFNode, aNewTarget: *const nsIRDFNode) -> nsresult,

    /* void onMove (in nsIRDFDataSource aDataSource, in nsIRDFResource aOldSource, in nsIRDFResource aNewSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget); */
    pub onMove: unsafe extern "C" fn (this: *const nsIRDFObserver, aDataSource: *const nsIRDFDataSource, aOldSource: *const nsIRDFResource, aNewSource: *const nsIRDFResource, aProperty: *const nsIRDFResource, aTarget: *const nsIRDFNode) -> nsresult,

    /* void onBeginUpdateBatch (in nsIRDFDataSource aDataSource); */
    pub onBeginUpdateBatch: unsafe extern "C" fn (this: *const nsIRDFObserver, aDataSource: *const nsIRDFDataSource) -> nsresult,

    /* void onEndUpdateBatch (in nsIRDFDataSource aDataSource); */
    pub onEndUpdateBatch: unsafe extern "C" fn (this: *const nsIRDFObserver, aDataSource: *const nsIRDFDataSource) -> nsresult,

}


impl nsIRDFObserver {
    /* void onAssert (in nsIRDFDataSource aDataSource, in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget); */
    #[inline]
    pub unsafe fn onAssert(&self, aDataSource: Option<&nsIRDFDataSource>, aSource: Option<&nsIRDFResource>, aProperty: Option<&nsIRDFResource>, aTarget: Option<&nsIRDFNode>) -> Result<(), nsresult> {

        match ((*self.vtable).onAssert)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _), aSource.map_or(::std::ptr::null(), |x| x as *const _), aProperty.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onUnassert (in nsIRDFDataSource aDataSource, in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget); */
    #[inline]
    pub unsafe fn onUnassert(&self, aDataSource: Option<&nsIRDFDataSource>, aSource: Option<&nsIRDFResource>, aProperty: Option<&nsIRDFResource>, aTarget: Option<&nsIRDFNode>) -> Result<(), nsresult> {

        match ((*self.vtable).onUnassert)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _), aSource.map_or(::std::ptr::null(), |x| x as *const _), aProperty.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onChange (in nsIRDFDataSource aDataSource, in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aOldTarget, in nsIRDFNode aNewTarget); */
    #[inline]
    pub unsafe fn onChange(&self, aDataSource: Option<&nsIRDFDataSource>, aSource: Option<&nsIRDFResource>, aProperty: Option<&nsIRDFResource>, aOldTarget: Option<&nsIRDFNode>, aNewTarget: Option<&nsIRDFNode>) -> Result<(), nsresult> {

        match ((*self.vtable).onChange)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _), aSource.map_or(::std::ptr::null(), |x| x as *const _), aProperty.map_or(::std::ptr::null(), |x| x as *const _), aOldTarget.map_or(::std::ptr::null(), |x| x as *const _), aNewTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onMove (in nsIRDFDataSource aDataSource, in nsIRDFResource aOldSource, in nsIRDFResource aNewSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget); */
    #[inline]
    pub unsafe fn onMove(&self, aDataSource: Option<&nsIRDFDataSource>, aOldSource: Option<&nsIRDFResource>, aNewSource: Option<&nsIRDFResource>, aProperty: Option<&nsIRDFResource>, aTarget: Option<&nsIRDFNode>) -> Result<(), nsresult> {

        match ((*self.vtable).onMove)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _), aOldSource.map_or(::std::ptr::null(), |x| x as *const _), aNewSource.map_or(::std::ptr::null(), |x| x as *const _), aProperty.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onBeginUpdateBatch (in nsIRDFDataSource aDataSource); */
    #[inline]
    pub unsafe fn onBeginUpdateBatch(&self, aDataSource: Option<&nsIRDFDataSource>) -> Result<(), nsresult> {

        match ((*self.vtable).onBeginUpdateBatch)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onEndUpdateBatch (in nsIRDFDataSource aDataSource); */
    #[inline]
    pub unsafe fn onEndUpdateBatch(&self, aDataSource: Option<&nsIRDFDataSource>) -> Result<(), nsresult> {

        match ((*self.vtable).onEndUpdateBatch)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


