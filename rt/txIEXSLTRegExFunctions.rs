//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/txIEXSLTRegExFunctions.idl
//


#[repr(C)]
pub struct txIEXSLTRegExFunctions {
    vtable: *const txIEXSLTRegExFunctionsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for txIEXSLTRegExFunctions {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc180e993, 0xaced, 0x4839,
            [0x95, 0xa0, 0xec, 0xd5, 0xff, 0x13, 0x8b, 0xe9])
    }
}

unsafe impl RefCounted for txIEXSLTRegExFunctions {
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
pub trait txIEXSLTRegExFunctionsCoerce {
    fn coerce_from(v: &txIEXSLTRegExFunctions) -> &Self;
}

impl txIEXSLTRegExFunctionsCoerce for txIEXSLTRegExFunctions {
    #[inline]
    fn coerce_from(v: &txIEXSLTRegExFunctions) -> &Self {
        v
    }
}

impl txIEXSLTRegExFunctions {
    #[inline]
    pub fn coerce<T: txIEXSLTRegExFunctionsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for txIEXSLTRegExFunctions {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> txIEXSLTRegExFunctionsCoerce for T {
    #[inline]
    fn coerce_from(v: &txIEXSLTRegExFunctions) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct txIEXSLTRegExFunctionsVTable {
    pub __base: nsISupportsVTable,

    /* txINodeSet match (in txIFunctionEvaluationContext aContext, in DOMString aString, in DOMString aRegEx, in DOMString aFlags); */
    pub match_: unsafe extern "C" fn (this: *const txIEXSLTRegExFunctions, aContext: *const txIFunctionEvaluationContext, aString: *const nsAString, aRegEx: *const nsAString, aFlags: *const nsAString, _retval: *mut *const txINodeSet) -> nsresult,

    /* DOMString replace (in DOMString aString, in DOMString aRegEx, in DOMString aFlags, in DOMString aReplace); */
    pub replace: unsafe extern "C" fn (this: *const txIEXSLTRegExFunctions, aString: *const nsAString, aRegEx: *const nsAString, aFlags: *const nsAString, aReplace: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* boolean test (in DOMString aString, in DOMString aRegEx, in DOMString aFlags); */
    pub test: unsafe extern "C" fn (this: *const txIEXSLTRegExFunctions, aString: *const nsAString, aRegEx: *const nsAString, aFlags: *const nsAString, _retval: *mut bool) -> nsresult,

}


impl txIEXSLTRegExFunctions {
    /* txINodeSet match (in txIFunctionEvaluationContext aContext, in DOMString aString, in DOMString aRegEx, in DOMString aFlags); */
    #[inline]
    pub unsafe fn match_(&self, aContext: Option<&txIFunctionEvaluationContext>, aString: &[u16], aRegEx: &[u16], aFlags: &[u16]) -> Result<Option<RefPtr<txINodeSet>>, nsresult> {
        let aString = nsString::from(aString);
        let aRegEx = nsString::from(aRegEx);
        let aFlags = nsString::from(aFlags);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).match_)(self as *const _, aContext.map_or(::std::ptr::null(), |x| x as *const _), &*aString, &*aRegEx, &*aFlags, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* DOMString replace (in DOMString aString, in DOMString aRegEx, in DOMString aFlags, in DOMString aReplace); */
    #[inline]
    pub unsafe fn replace(&self, aString: &[u16], aRegEx: &[u16], aFlags: &[u16], aReplace: &[u16]) -> Result<nsString, nsresult> {
        let aString = nsString::from(aString);
        let aRegEx = nsString::from(aRegEx);
        let aFlags = nsString::from(aFlags);
        let aReplace = nsString::from(aReplace);
        let mut _retval = nsString::new();
        match ((*self.vtable).replace)(self as *const _, &*aString, &*aRegEx, &*aFlags, &*aReplace, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean test (in DOMString aString, in DOMString aRegEx, in DOMString aFlags); */
    #[inline]
    pub unsafe fn test(&self, aString: &[u16], aRegEx: &[u16], aFlags: &[u16]) -> Result<bool, nsresult> {
        let aString = nsString::from(aString);
        let aRegEx = nsString::from(aRegEx);
        let aFlags = nsString::from(aFlags);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).test)(self as *const _, &*aString, &*aRegEx, &*aFlags, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


