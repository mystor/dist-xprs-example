//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptLoaderObserver.idl
//


#[repr(C)]
pub struct nsIScriptLoaderObserver {
    vtable: *const nsIScriptLoaderObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIScriptLoaderObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7b787204, 0x76fb, 0x4764,
            [0x96, 0xf1, 0xfb, 0x7a, 0x66, 0x6d, 0xb4, 0xf4])
    }
}

unsafe impl RefCounted for nsIScriptLoaderObserver {
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
pub trait nsIScriptLoaderObserverCoerce {
    fn coerce_from(v: &nsIScriptLoaderObserver) -> &Self;
}

impl nsIScriptLoaderObserverCoerce for nsIScriptLoaderObserver {
    #[inline]
    fn coerce_from(v: &nsIScriptLoaderObserver) -> &Self {
        v
    }
}

impl nsIScriptLoaderObserver {
    #[inline]
    pub fn coerce<T: nsIScriptLoaderObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIScriptLoaderObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIScriptLoaderObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptLoaderObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIScriptLoaderObserverVTable {
    pub __base: nsISupportsVTable,

    /* void scriptAvailable (in nsresult aResult, in nsIScriptElement aElement, in boolean aIsInline, in nsIURI aURI, in int32_t aLineNo); */
    pub scriptAvailable: unsafe extern "C" fn (this: *const nsIScriptLoaderObserver, aResult: nsresult, aElement: *const nsIScriptElement, aIsInline: bool, aURI: *const nsIURI, aLineNo: int32_t) -> nsresult,

    /* void scriptEvaluated (in nsresult aResult, in nsIScriptElement aElement, in boolean aIsInline); */
    pub scriptEvaluated: unsafe extern "C" fn (this: *const nsIScriptLoaderObserver, aResult: nsresult, aElement: *const nsIScriptElement, aIsInline: bool) -> nsresult,

}


impl nsIScriptLoaderObserver {
    /* void scriptAvailable (in nsresult aResult, in nsIScriptElement aElement, in boolean aIsInline, in nsIURI aURI, in int32_t aLineNo); */
    #[inline]
    pub unsafe fn scriptAvailable(&self, aResult: nsresult, aElement: Option<&nsIScriptElement>, aIsInline: bool, aURI: Option<&nsIURI>, aLineNo: int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).scriptAvailable)(self as *const _, aResult, aElement.map_or(::std::ptr::null(), |x| x as *const _), aIsInline, aURI.map_or(::std::ptr::null(), |x| x as *const _), aLineNo) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scriptEvaluated (in nsresult aResult, in nsIScriptElement aElement, in boolean aIsInline); */
    #[inline]
    pub unsafe fn scriptEvaluated(&self, aResult: nsresult, aElement: Option<&nsIScriptElement>, aIsInline: bool) -> Result<(), nsresult> {

        match ((*self.vtable).scriptEvaluated)(self as *const _, aResult, aElement.map_or(::std::ptr::null(), |x| x as *const _), aIsInline) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


