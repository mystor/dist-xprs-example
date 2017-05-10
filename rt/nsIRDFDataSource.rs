//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFDataSource.idl
//


#[repr(C)]
pub struct nsIRDFDataSource {
    vtable: *const nsIRDFDataSourceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFDataSource {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0f78da58, 0x8321, 0x11d2,
            [0x8e, 0xac, 0x00, 0x80, 0x5f, 0x29, 0xf3, 0x70])
    }
}

unsafe impl RefCounted for nsIRDFDataSource {
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
pub trait nsIRDFDataSourceCoerce {
    fn coerce_from(v: &nsIRDFDataSource) -> &Self;
}

impl nsIRDFDataSourceCoerce for nsIRDFDataSource {
    #[inline]
    fn coerce_from(v: &nsIRDFDataSource) -> &Self {
        v
    }
}

impl nsIRDFDataSource {
    #[inline]
    pub fn coerce<T: nsIRDFDataSourceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFDataSource {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFDataSourceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFDataSource) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFDataSourceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute string URI; */
    pub get_URI: unsafe extern "C" fn (this: *const nsIRDFDataSource, aURI: *mut *const libc::c_char) -> nsresult,

    /* nsIRDFResource GetSource (in nsIRDFResource aProperty, in nsIRDFNode aTarget, in boolean aTruthValue); */
    pub GetSource: unsafe extern "C" fn (this: *const nsIRDFDataSource, aProperty: *const nsIRDFResource, aTarget: *const nsIRDFNode, aTruthValue: bool, _retval: *mut *const nsIRDFResource) -> nsresult,

    /* nsISimpleEnumerator GetSources (in nsIRDFResource aProperty, in nsIRDFNode aTarget, in boolean aTruthValue); */
    pub GetSources: unsafe extern "C" fn (this: *const nsIRDFDataSource, aProperty: *const nsIRDFResource, aTarget: *const nsIRDFNode, aTruthValue: bool, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* nsIRDFNode GetTarget (in nsIRDFResource aSource, in nsIRDFResource aProperty, in boolean aTruthValue); */
    pub GetTarget: unsafe extern "C" fn (this: *const nsIRDFDataSource, aSource: *const nsIRDFResource, aProperty: *const nsIRDFResource, aTruthValue: bool, _retval: *mut *const nsIRDFNode) -> nsresult,

    /* nsISimpleEnumerator GetTargets (in nsIRDFResource aSource, in nsIRDFResource aProperty, in boolean aTruthValue); */
    pub GetTargets: unsafe extern "C" fn (this: *const nsIRDFDataSource, aSource: *const nsIRDFResource, aProperty: *const nsIRDFResource, aTruthValue: bool, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* void Assert (in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget, in boolean aTruthValue); */
    pub Assert: unsafe extern "C" fn (this: *const nsIRDFDataSource, aSource: *const nsIRDFResource, aProperty: *const nsIRDFResource, aTarget: *const nsIRDFNode, aTruthValue: bool) -> nsresult,

    /* void Unassert (in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget); */
    pub Unassert: unsafe extern "C" fn (this: *const nsIRDFDataSource, aSource: *const nsIRDFResource, aProperty: *const nsIRDFResource, aTarget: *const nsIRDFNode) -> nsresult,

    /* void Change (in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aOldTarget, in nsIRDFNode aNewTarget); */
    pub Change: unsafe extern "C" fn (this: *const nsIRDFDataSource, aSource: *const nsIRDFResource, aProperty: *const nsIRDFResource, aOldTarget: *const nsIRDFNode, aNewTarget: *const nsIRDFNode) -> nsresult,

    /* void Move (in nsIRDFResource aOldSource, in nsIRDFResource aNewSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget); */
    pub Move: unsafe extern "C" fn (this: *const nsIRDFDataSource, aOldSource: *const nsIRDFResource, aNewSource: *const nsIRDFResource, aProperty: *const nsIRDFResource, aTarget: *const nsIRDFNode) -> nsresult,

    /* boolean HasAssertion (in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget, in boolean aTruthValue); */
    pub HasAssertion: unsafe extern "C" fn (this: *const nsIRDFDataSource, aSource: *const nsIRDFResource, aProperty: *const nsIRDFResource, aTarget: *const nsIRDFNode, aTruthValue: bool, _retval: *mut bool) -> nsresult,

    /* void AddObserver (in nsIRDFObserver aObserver); */
    pub AddObserver: unsafe extern "C" fn (this: *const nsIRDFDataSource, aObserver: *const nsIRDFObserver) -> nsresult,

    /* void RemoveObserver (in nsIRDFObserver aObserver); */
    pub RemoveObserver: unsafe extern "C" fn (this: *const nsIRDFDataSource, aObserver: *const nsIRDFObserver) -> nsresult,

    /* nsISimpleEnumerator ArcLabelsIn (in nsIRDFNode aNode); */
    pub ArcLabelsIn: unsafe extern "C" fn (this: *const nsIRDFDataSource, aNode: *const nsIRDFNode, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* nsISimpleEnumerator ArcLabelsOut (in nsIRDFResource aSource); */
    pub ArcLabelsOut: unsafe extern "C" fn (this: *const nsIRDFDataSource, aSource: *const nsIRDFResource, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* nsISimpleEnumerator GetAllResources (); */
    pub GetAllResources: unsafe extern "C" fn (this: *const nsIRDFDataSource, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* boolean IsCommandEnabled (in nsISupports aSources, in nsIRDFResource aCommand, in nsISupports aArguments); */
    pub IsCommandEnabled: unsafe extern "C" fn (this: *const nsIRDFDataSource, aSources: *const nsISupports, aCommand: *const nsIRDFResource, aArguments: *const nsISupports, _retval: *mut bool) -> nsresult,

    /* void DoCommand (in nsISupports aSources, in nsIRDFResource aCommand, in nsISupports aArguments); */
    pub DoCommand: unsafe extern "C" fn (this: *const nsIRDFDataSource, aSources: *const nsISupports, aCommand: *const nsIRDFResource, aArguments: *const nsISupports) -> nsresult,

    /* nsISimpleEnumerator GetAllCmds (in nsIRDFResource aSource); */
    pub GetAllCmds: unsafe extern "C" fn (this: *const nsIRDFDataSource, aSource: *const nsIRDFResource, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* boolean hasArcIn (in nsIRDFNode aNode, in nsIRDFResource aArc); */
    pub hasArcIn: unsafe extern "C" fn (this: *const nsIRDFDataSource, aNode: *const nsIRDFNode, aArc: *const nsIRDFResource, _retval: *mut bool) -> nsresult,

    /* boolean hasArcOut (in nsIRDFResource aSource, in nsIRDFResource aArc); */
    pub hasArcOut: unsafe extern "C" fn (this: *const nsIRDFDataSource, aSource: *const nsIRDFResource, aArc: *const nsIRDFResource, _retval: *mut bool) -> nsresult,

    /* void beginUpdateBatch (); */
    pub beginUpdateBatch: unsafe extern "C" fn (this: *const nsIRDFDataSource) -> nsresult,

    /* void endUpdateBatch (); */
    pub endUpdateBatch: unsafe extern "C" fn (this: *const nsIRDFDataSource) -> nsresult,

}


impl nsIRDFDataSource {
    /* readonly attribute string URI; */
    #[inline]
    pub unsafe fn get_URI(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_URI)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIRDFResource GetSource (in nsIRDFResource aProperty, in nsIRDFNode aTarget, in boolean aTruthValue); */
    #[inline]
    pub unsafe fn GetSource(&self, aProperty: Option<&nsIRDFResource>, aTarget: Option<&nsIRDFNode>, aTruthValue: bool) -> Result<Option<RefPtr<nsIRDFResource>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetSource)(self as *const _, aProperty.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _), aTruthValue, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISimpleEnumerator GetSources (in nsIRDFResource aProperty, in nsIRDFNode aTarget, in boolean aTruthValue); */
    #[inline]
    pub unsafe fn GetSources(&self, aProperty: Option<&nsIRDFResource>, aTarget: Option<&nsIRDFNode>, aTruthValue: bool) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetSources)(self as *const _, aProperty.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _), aTruthValue, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIRDFNode GetTarget (in nsIRDFResource aSource, in nsIRDFResource aProperty, in boolean aTruthValue); */
    #[inline]
    pub unsafe fn GetTarget(&self, aSource: Option<&nsIRDFResource>, aProperty: Option<&nsIRDFResource>, aTruthValue: bool) -> Result<Option<RefPtr<nsIRDFNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetTarget)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _), aProperty.map_or(::std::ptr::null(), |x| x as *const _), aTruthValue, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISimpleEnumerator GetTargets (in nsIRDFResource aSource, in nsIRDFResource aProperty, in boolean aTruthValue); */
    #[inline]
    pub unsafe fn GetTargets(&self, aSource: Option<&nsIRDFResource>, aProperty: Option<&nsIRDFResource>, aTruthValue: bool) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetTargets)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _), aProperty.map_or(::std::ptr::null(), |x| x as *const _), aTruthValue, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void Assert (in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget, in boolean aTruthValue); */
    #[inline]
    pub unsafe fn Assert(&self, aSource: Option<&nsIRDFResource>, aProperty: Option<&nsIRDFResource>, aTarget: Option<&nsIRDFNode>, aTruthValue: bool) -> Result<(), nsresult> {

        match ((*self.vtable).Assert)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _), aProperty.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _), aTruthValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void Unassert (in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget); */
    #[inline]
    pub unsafe fn Unassert(&self, aSource: Option<&nsIRDFResource>, aProperty: Option<&nsIRDFResource>, aTarget: Option<&nsIRDFNode>) -> Result<(), nsresult> {

        match ((*self.vtable).Unassert)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _), aProperty.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void Change (in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aOldTarget, in nsIRDFNode aNewTarget); */
    #[inline]
    pub unsafe fn Change(&self, aSource: Option<&nsIRDFResource>, aProperty: Option<&nsIRDFResource>, aOldTarget: Option<&nsIRDFNode>, aNewTarget: Option<&nsIRDFNode>) -> Result<(), nsresult> {

        match ((*self.vtable).Change)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _), aProperty.map_or(::std::ptr::null(), |x| x as *const _), aOldTarget.map_or(::std::ptr::null(), |x| x as *const _), aNewTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void Move (in nsIRDFResource aOldSource, in nsIRDFResource aNewSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget); */
    #[inline]
    pub unsafe fn Move(&self, aOldSource: Option<&nsIRDFResource>, aNewSource: Option<&nsIRDFResource>, aProperty: Option<&nsIRDFResource>, aTarget: Option<&nsIRDFNode>) -> Result<(), nsresult> {

        match ((*self.vtable).Move)(self as *const _, aOldSource.map_or(::std::ptr::null(), |x| x as *const _), aNewSource.map_or(::std::ptr::null(), |x| x as *const _), aProperty.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean HasAssertion (in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget, in boolean aTruthValue); */
    #[inline]
    pub unsafe fn HasAssertion(&self, aSource: Option<&nsIRDFResource>, aProperty: Option<&nsIRDFResource>, aTarget: Option<&nsIRDFNode>, aTruthValue: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).HasAssertion)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _), aProperty.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _), aTruthValue, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void AddObserver (in nsIRDFObserver aObserver); */
    #[inline]
    pub unsafe fn AddObserver(&self, aObserver: Option<&nsIRDFObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).AddObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void RemoveObserver (in nsIRDFObserver aObserver); */
    #[inline]
    pub unsafe fn RemoveObserver(&self, aObserver: Option<&nsIRDFObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).RemoveObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISimpleEnumerator ArcLabelsIn (in nsIRDFNode aNode); */
    #[inline]
    pub unsafe fn ArcLabelsIn(&self, aNode: Option<&nsIRDFNode>) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).ArcLabelsIn)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISimpleEnumerator ArcLabelsOut (in nsIRDFResource aSource); */
    #[inline]
    pub unsafe fn ArcLabelsOut(&self, aSource: Option<&nsIRDFResource>) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).ArcLabelsOut)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISimpleEnumerator GetAllResources (); */
    #[inline]
    pub unsafe fn GetAllResources(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetAllResources)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean IsCommandEnabled (in nsISupports aSources, in nsIRDFResource aCommand, in nsISupports aArguments); */
    #[inline]
    pub unsafe fn IsCommandEnabled(&self, aSources: Option<&nsISupports>, aCommand: Option<&nsIRDFResource>, aArguments: Option<&nsISupports>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).IsCommandEnabled)(self as *const _, aSources.map_or(::std::ptr::null(), |x| x as *const _), aCommand.map_or(::std::ptr::null(), |x| x as *const _), aArguments.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void DoCommand (in nsISupports aSources, in nsIRDFResource aCommand, in nsISupports aArguments); */
    #[inline]
    pub unsafe fn DoCommand(&self, aSources: Option<&nsISupports>, aCommand: Option<&nsIRDFResource>, aArguments: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).DoCommand)(self as *const _, aSources.map_or(::std::ptr::null(), |x| x as *const _), aCommand.map_or(::std::ptr::null(), |x| x as *const _), aArguments.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISimpleEnumerator GetAllCmds (in nsIRDFResource aSource); */
    #[inline]
    pub unsafe fn GetAllCmds(&self, aSource: Option<&nsIRDFResource>) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetAllCmds)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean hasArcIn (in nsIRDFNode aNode, in nsIRDFResource aArc); */
    #[inline]
    pub unsafe fn hasArcIn(&self, aNode: Option<&nsIRDFNode>, aArc: Option<&nsIRDFResource>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasArcIn)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), aArc.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean hasArcOut (in nsIRDFResource aSource, in nsIRDFResource aArc); */
    #[inline]
    pub unsafe fn hasArcOut(&self, aSource: Option<&nsIRDFResource>, aArc: Option<&nsIRDFResource>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasArcOut)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _), aArc.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void beginUpdateBatch (); */
    #[inline]
    pub unsafe fn beginUpdateBatch(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).beginUpdateBatch)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endUpdateBatch (); */
    #[inline]
    pub unsafe fn endUpdateBatch(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).endUpdateBatch)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


