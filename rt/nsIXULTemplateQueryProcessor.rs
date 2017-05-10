//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULTemplateQueryProcessor.idl
//


#[repr(C)]
pub struct nsIXULTemplateQueryProcessor {
    vtable: *const nsIXULTemplateQueryProcessorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXULTemplateQueryProcessor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc257573f, 0x444f, 0x468a,
            [0xba, 0x27, 0xde, 0x97, 0x9d, 0xc5, 0x5f, 0xe4])
    }
}

unsafe impl RefCounted for nsIXULTemplateQueryProcessor {
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
pub trait nsIXULTemplateQueryProcessorCoerce {
    fn coerce_from(v: &nsIXULTemplateQueryProcessor) -> &Self;
}

impl nsIXULTemplateQueryProcessorCoerce for nsIXULTemplateQueryProcessor {
    #[inline]
    fn coerce_from(v: &nsIXULTemplateQueryProcessor) -> &Self {
        v
    }
}

impl nsIXULTemplateQueryProcessor {
    #[inline]
    pub fn coerce<T: nsIXULTemplateQueryProcessorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXULTemplateQueryProcessor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXULTemplateQueryProcessorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULTemplateQueryProcessor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXULTemplateQueryProcessorVTable {
    pub __base: nsISupportsVTable,

    /* nsISupports getDatasource (in nsIArray aDataSources, in nsIDOMNode aRootNode, in boolean aIsTrusted, in nsIXULTemplateBuilder aBuilder, out boolean aShouldDelayBuilding); */
    pub getDatasource: unsafe extern "C" fn (this: *const nsIXULTemplateQueryProcessor, aDataSources: *const nsIArray, aRootNode: *const nsIDOMNode, aIsTrusted: bool, aBuilder: *const nsIXULTemplateBuilder, aShouldDelayBuilding: *mut bool, _retval: *mut *const nsISupports) -> nsresult,

    /* void initializeForBuilding (in nsISupports aDatasource, in nsIXULTemplateBuilder aBuilder, in nsIDOMNode aRootNode); */
    pub initializeForBuilding: unsafe extern "C" fn (this: *const nsIXULTemplateQueryProcessor, aDatasource: *const nsISupports, aBuilder: *const nsIXULTemplateBuilder, aRootNode: *const nsIDOMNode) -> nsresult,

    /* void done (); */
    pub done: unsafe extern "C" fn (this: *const nsIXULTemplateQueryProcessor) -> nsresult,

    /* nsISupports compileQuery (in nsIXULTemplateBuilder aBuilder, in nsIDOMNode aQuery, in nsIAtom aRefVariable, in nsIAtom aMemberVariable); */
    pub compileQuery: unsafe extern "C" fn (this: *const nsIXULTemplateQueryProcessor, aBuilder: *const nsIXULTemplateBuilder, aQuery: *const nsIDOMNode, aRefVariable: *const nsIAtom, aMemberVariable: *const nsIAtom, _retval: *mut *const nsISupports) -> nsresult,

    /* nsISimpleEnumerator generateResults (in nsISupports aDatasource, in nsIXULTemplateResult aRef, in nsISupports aQuery); */
    pub generateResults: unsafe extern "C" fn (this: *const nsIXULTemplateQueryProcessor, aDatasource: *const nsISupports, aRef: *const nsIXULTemplateResult, aQuery: *const nsISupports, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* void addBinding (in nsIDOMNode aRuleNode, in nsIAtom aVar, in nsIAtom aRef, in AString aExpr); */
    pub addBinding: unsafe extern "C" fn (this: *const nsIXULTemplateQueryProcessor, aRuleNode: *const nsIDOMNode, aVar: *const nsIAtom, aRef: *const nsIAtom, aExpr: *const nsAString) -> nsresult,

    /* nsIXULTemplateResult translateRef (in nsISupports aDatasource, in AString aRefString); */
    pub translateRef: unsafe extern "C" fn (this: *const nsIXULTemplateQueryProcessor, aDatasource: *const nsISupports, aRefString: *const nsAString, _retval: *mut *const nsIXULTemplateResult) -> nsresult,

    /* int32_t compareResults (in nsIXULTemplateResult aLeft, in nsIXULTemplateResult aRight, in nsIAtom aVar, in unsigned long aSortHints); */
    pub compareResults: unsafe extern "C" fn (this: *const nsIXULTemplateQueryProcessor, aLeft: *const nsIXULTemplateResult, aRight: *const nsIXULTemplateResult, aVar: *const nsIAtom, aSortHints: libc::uint32_t, _retval: *mut int32_t) -> nsresult,

}


impl nsIXULTemplateQueryProcessor {
    /* nsISupports getDatasource (in nsIArray aDataSources, in nsIDOMNode aRootNode, in boolean aIsTrusted, in nsIXULTemplateBuilder aBuilder, out boolean aShouldDelayBuilding); */
    #[inline]
    pub unsafe fn getDatasource(&self, aDataSources: Option<&nsIArray>, aRootNode: Option<&nsIDOMNode>, aIsTrusted: bool, aBuilder: Option<&nsIXULTemplateBuilder>) -> Result<(bool, Option<RefPtr<nsISupports>>), nsresult> {
        let mut aShouldDelayBuilding: bool = ::std::mem::zeroed();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getDatasource)(self as *const _, aDataSources.map_or(::std::ptr::null(), |x| x as *const _), aRootNode.map_or(::std::ptr::null(), |x| x as *const _), aIsTrusted, aBuilder.map_or(::std::ptr::null(), |x| x as *const _), &mut aShouldDelayBuilding as *mut _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aShouldDelayBuilding, _retval.refptr()))
    }

    /* void initializeForBuilding (in nsISupports aDatasource, in nsIXULTemplateBuilder aBuilder, in nsIDOMNode aRootNode); */
    #[inline]
    pub unsafe fn initializeForBuilding(&self, aDatasource: Option<&nsISupports>, aBuilder: Option<&nsIXULTemplateBuilder>, aRootNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).initializeForBuilding)(self as *const _, aDatasource.map_or(::std::ptr::null(), |x| x as *const _), aBuilder.map_or(::std::ptr::null(), |x| x as *const _), aRootNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void done (); */
    #[inline]
    pub unsafe fn done(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).done)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISupports compileQuery (in nsIXULTemplateBuilder aBuilder, in nsIDOMNode aQuery, in nsIAtom aRefVariable, in nsIAtom aMemberVariable); */
    #[inline]
    pub unsafe fn compileQuery(&self, aBuilder: Option<&nsIXULTemplateBuilder>, aQuery: Option<&nsIDOMNode>, aRefVariable: Option<&nsIAtom>, aMemberVariable: Option<&nsIAtom>) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).compileQuery)(self as *const _, aBuilder.map_or(::std::ptr::null(), |x| x as *const _), aQuery.map_or(::std::ptr::null(), |x| x as *const _), aRefVariable.map_or(::std::ptr::null(), |x| x as *const _), aMemberVariable.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISimpleEnumerator generateResults (in nsISupports aDatasource, in nsIXULTemplateResult aRef, in nsISupports aQuery); */
    #[inline]
    pub unsafe fn generateResults(&self, aDatasource: Option<&nsISupports>, aRef: Option<&nsIXULTemplateResult>, aQuery: Option<&nsISupports>) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).generateResults)(self as *const _, aDatasource.map_or(::std::ptr::null(), |x| x as *const _), aRef.map_or(::std::ptr::null(), |x| x as *const _), aQuery.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void addBinding (in nsIDOMNode aRuleNode, in nsIAtom aVar, in nsIAtom aRef, in AString aExpr); */
    #[inline]
    pub unsafe fn addBinding(&self, aRuleNode: Option<&nsIDOMNode>, aVar: Option<&nsIAtom>, aRef: Option<&nsIAtom>, aExpr: &[u16]) -> Result<(), nsresult> {
        let aExpr = nsString::from(aExpr);
        match ((*self.vtable).addBinding)(self as *const _, aRuleNode.map_or(::std::ptr::null(), |x| x as *const _), aVar.map_or(::std::ptr::null(), |x| x as *const _), aRef.map_or(::std::ptr::null(), |x| x as *const _), &*aExpr) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIXULTemplateResult translateRef (in nsISupports aDatasource, in AString aRefString); */
    #[inline]
    pub unsafe fn translateRef(&self, aDatasource: Option<&nsISupports>, aRefString: &[u16]) -> Result<Option<RefPtr<nsIXULTemplateResult>>, nsresult> {
        let aRefString = nsString::from(aRefString);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).translateRef)(self as *const _, aDatasource.map_or(::std::ptr::null(), |x| x as *const _), &*aRefString, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* int32_t compareResults (in nsIXULTemplateResult aLeft, in nsIXULTemplateResult aRight, in nsIAtom aVar, in unsigned long aSortHints); */
    #[inline]
    pub unsafe fn compareResults(&self, aLeft: Option<&nsIXULTemplateResult>, aRight: Option<&nsIXULTemplateResult>, aVar: Option<&nsIAtom>, aSortHints: libc::uint32_t) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).compareResults)(self as *const _, aLeft.map_or(::std::ptr::null(), |x| x as *const _), aRight.map_or(::std::ptr::null(), |x| x as *const _), aVar.map_or(::std::ptr::null(), |x| x as *const _), aSortHints, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


