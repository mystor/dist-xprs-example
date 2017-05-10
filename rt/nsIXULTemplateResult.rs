//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULTemplateResult.idl
//


#[repr(C)]
pub struct nsIXULTemplateResult {
    vtable: *const nsIXULTemplateResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXULTemplateResult {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xebea0230, 0x36fa, 0x41b7,
            [0x8e, 0x31, 0x76, 0x08, 0x06, 0x05, 0x79, 0x65])
    }
}

unsafe impl RefCounted for nsIXULTemplateResult {
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
pub trait nsIXULTemplateResultCoerce {
    fn coerce_from(v: &nsIXULTemplateResult) -> &Self;
}

impl nsIXULTemplateResultCoerce for nsIXULTemplateResult {
    #[inline]
    fn coerce_from(v: &nsIXULTemplateResult) -> &Self {
        v
    }
}

impl nsIXULTemplateResult {
    #[inline]
    pub fn coerce<T: nsIXULTemplateResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXULTemplateResult {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXULTemplateResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULTemplateResult) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXULTemplateResultVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isContainer; */
    pub get_isContainer: unsafe extern "C" fn (this: *const nsIXULTemplateResult, aIsContainer: *mut bool) -> nsresult,

    /* readonly attribute boolean isEmpty; */
    pub get_isEmpty: unsafe extern "C" fn (this: *const nsIXULTemplateResult, aIsEmpty: *mut bool) -> nsresult,

    /* readonly attribute boolean mayProcessChildren; */
    pub get_mayProcessChildren: unsafe extern "C" fn (this: *const nsIXULTemplateResult, aMayProcessChildren: *mut bool) -> nsresult,

    /* readonly attribute AString id; */
    pub get_id: unsafe extern "C" fn (this: *const nsIXULTemplateResult, aId: *mut nsAString) -> nsresult,

    /* readonly attribute nsIRDFResource resource; */
    pub get_resource: unsafe extern "C" fn (this: *const nsIXULTemplateResult, aResource: *mut *const nsIRDFResource) -> nsresult,

    /* readonly attribute AString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIXULTemplateResult, aType: *mut nsAString) -> nsresult,

    /* AString getBindingFor (in nsIAtom aVar); */
    pub getBindingFor: unsafe extern "C" fn (this: *const nsIXULTemplateResult, aVar: *const nsIAtom, _retval: *mut nsAString) -> nsresult,

    /* nsISupports getBindingObjectFor (in nsIAtom aVar); */
    pub getBindingObjectFor: unsafe extern "C" fn (this: *const nsIXULTemplateResult, aVar: *const nsIAtom, _retval: *mut *const nsISupports) -> nsresult,

    /* void ruleMatched (in nsISupports aQuery, in nsIDOMNode aRuleNode); */
    pub ruleMatched: unsafe extern "C" fn (this: *const nsIXULTemplateResult, aQuery: *const nsISupports, aRuleNode: *const nsIDOMNode) -> nsresult,

    /* void hasBeenRemoved (); */
    pub hasBeenRemoved: unsafe extern "C" fn (this: *const nsIXULTemplateResult) -> nsresult,

}


impl nsIXULTemplateResult {
    /* readonly attribute boolean isContainer; */
    #[inline]
    pub unsafe fn get_isContainer(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isContainer)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isEmpty; */
    #[inline]
    pub unsafe fn get_isEmpty(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isEmpty)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean mayProcessChildren; */
    #[inline]
    pub unsafe fn get_mayProcessChildren(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mayProcessChildren)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString id; */
    #[inline]
    pub unsafe fn get_id(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_id)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIRDFResource resource; */
    #[inline]
    pub unsafe fn get_resource(&self, ) -> Result<Option<RefPtr<nsIRDFResource>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_resource)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getBindingFor (in nsIAtom aVar); */
    #[inline]
    pub unsafe fn getBindingFor(&self, aVar: Option<&nsIAtom>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getBindingFor)(self as *const _, aVar.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISupports getBindingObjectFor (in nsIAtom aVar); */
    #[inline]
    pub unsafe fn getBindingObjectFor(&self, aVar: Option<&nsIAtom>) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getBindingObjectFor)(self as *const _, aVar.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void ruleMatched (in nsISupports aQuery, in nsIDOMNode aRuleNode); */
    #[inline]
    pub unsafe fn ruleMatched(&self, aQuery: Option<&nsISupports>, aRuleNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).ruleMatched)(self as *const _, aQuery.map_or(::std::ptr::null(), |x| x as *const _), aRuleNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void hasBeenRemoved (); */
    #[inline]
    pub unsafe fn hasBeenRemoved(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).hasBeenRemoved)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


