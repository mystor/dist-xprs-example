//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXPathEvaluator.idl
//


#[repr(C)]
pub struct nsIDOMXPathEvaluator {
    vtable: *const nsIDOMXPathEvaluatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXPathEvaluator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x92584002, 0xd0e2, 0x4b88,
            [0x9a, 0xf9, 0xfa, 0x6f, 0xf5, 0x9e, 0xe0, 0x02])
    }
}

unsafe impl RefCounted for nsIDOMXPathEvaluator {
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
pub trait nsIDOMXPathEvaluatorCoerce {
    fn coerce_from(v: &nsIDOMXPathEvaluator) -> &Self;
}

impl nsIDOMXPathEvaluatorCoerce for nsIDOMXPathEvaluator {
    #[inline]
    fn coerce_from(v: &nsIDOMXPathEvaluator) -> &Self {
        v
    }
}

impl nsIDOMXPathEvaluator {
    #[inline]
    pub fn coerce<T: nsIDOMXPathEvaluatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXPathEvaluator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMXPathEvaluatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXPathEvaluator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXPathEvaluatorVTable {
    pub __base: nsISupportsVTable,

    /* nsISupports evaluate (in DOMString expression, in nsIDOMNode contextNode, in nsIDOMNode resolver, in unsigned short type, in nsISupports result) raises (XPathException,DOMException); */
    pub evaluate: unsafe extern "C" fn (this: *const nsIDOMXPathEvaluator, expression: *const nsAString, contextNode: *const nsIDOMNode, resolver: *const nsIDOMNode, type_: libc::uint16_t, result: *const nsISupports, _retval: *mut *const nsISupports) -> nsresult,

}


impl nsIDOMXPathEvaluator {
    /* nsISupports evaluate (in DOMString expression, in nsIDOMNode contextNode, in nsIDOMNode resolver, in unsigned short type, in nsISupports result) raises (XPathException,DOMException); */
    #[inline]
    pub unsafe fn evaluate(&self, expression: &[u16], contextNode: Option<&nsIDOMNode>, resolver: Option<&nsIDOMNode>, type_: libc::uint16_t, result: Option<&nsISupports>) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let expression = nsString::from(expression);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).evaluate)(self as *const _, &*expression, contextNode.map_or(::std::ptr::null(), |x| x as *const _), resolver.map_or(::std::ptr::null(), |x| x as *const _), type_, result.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


