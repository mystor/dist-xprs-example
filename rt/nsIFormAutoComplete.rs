//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFormAutoComplete.idl
//


#[repr(C)]
pub struct nsIFormAutoComplete {
    vtable: *const nsIFormAutoCompleteVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFormAutoComplete {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbfd9b82b, 0x0ab3, 0x4b6b,
            [0x9e, 0x54, 0xaa, 0x96, 0x1f, 0xf4, 0xb7, 0x32])
    }
}

unsafe impl RefCounted for nsIFormAutoComplete {
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
pub trait nsIFormAutoCompleteCoerce {
    fn coerce_from(v: &nsIFormAutoComplete) -> &Self;
}

impl nsIFormAutoCompleteCoerce for nsIFormAutoComplete {
    #[inline]
    fn coerce_from(v: &nsIFormAutoComplete) -> &Self {
        v
    }
}

impl nsIFormAutoComplete {
    #[inline]
    pub fn coerce<T: nsIFormAutoCompleteCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFormAutoComplete {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFormAutoCompleteCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFormAutoComplete) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFormAutoCompleteVTable {
    pub __base: nsISupportsVTable,

    /* void autoCompleteSearchAsync (in AString aInputName, in AString aSearchString, in nsIDOMHTMLInputElement aField, in nsIAutoCompleteResult aPreviousResult, in nsIAutoCompleteResult aDatalistResult, in nsIFormAutoCompleteObserver aListener); */
    pub autoCompleteSearchAsync: unsafe extern "C" fn (this: *const nsIFormAutoComplete, aInputName: *const nsAString, aSearchString: *const nsAString, aField: *const nsIDOMHTMLInputElement, aPreviousResult: *const nsIAutoCompleteResult, aDatalistResult: *const nsIAutoCompleteResult, aListener: *const nsIFormAutoCompleteObserver) -> nsresult,

    /* void stopAutoCompleteSearch (); */
    pub stopAutoCompleteSearch: unsafe extern "C" fn (this: *const nsIFormAutoComplete) -> nsresult,

    /* void stopControllingInput (in nsIDOMHTMLInputElement aField); */
    pub stopControllingInput: unsafe extern "C" fn (this: *const nsIFormAutoComplete, aField: *const nsIDOMHTMLInputElement) -> nsresult,

}


impl nsIFormAutoComplete {
    /* void autoCompleteSearchAsync (in AString aInputName, in AString aSearchString, in nsIDOMHTMLInputElement aField, in nsIAutoCompleteResult aPreviousResult, in nsIAutoCompleteResult aDatalistResult, in nsIFormAutoCompleteObserver aListener); */
    #[inline]
    pub unsafe fn autoCompleteSearchAsync(&self, aInputName: &[u16], aSearchString: &[u16], aField: Option<&nsIDOMHTMLInputElement>, aPreviousResult: Option<&nsIAutoCompleteResult>, aDatalistResult: Option<&nsIAutoCompleteResult>, aListener: Option<&nsIFormAutoCompleteObserver>) -> Result<(), nsresult> {
        let aInputName = nsString::from(aInputName);
        let aSearchString = nsString::from(aSearchString);
        match ((*self.vtable).autoCompleteSearchAsync)(self as *const _, &*aInputName, &*aSearchString, aField.map_or(::std::ptr::null(), |x| x as *const _), aPreviousResult.map_or(::std::ptr::null(), |x| x as *const _), aDatalistResult.map_or(::std::ptr::null(), |x| x as *const _), aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stopAutoCompleteSearch (); */
    #[inline]
    pub unsafe fn stopAutoCompleteSearch(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stopAutoCompleteSearch)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stopControllingInput (in nsIDOMHTMLInputElement aField); */
    #[inline]
    pub unsafe fn stopControllingInput(&self, aField: Option<&nsIDOMHTMLInputElement>) -> Result<(), nsresult> {

        match ((*self.vtable).stopControllingInput)(self as *const _, aField.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIFormAutoCompleteObserver {
    vtable: *const nsIFormAutoCompleteObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFormAutoCompleteObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x604419ab, 0x55a0, 0x4831,
            [0x9e, 0xca, 0x1b, 0x9e, 0x67, 0xcc, 0x47, 0x51])
    }
}

unsafe impl RefCounted for nsIFormAutoCompleteObserver {
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
pub trait nsIFormAutoCompleteObserverCoerce {
    fn coerce_from(v: &nsIFormAutoCompleteObserver) -> &Self;
}

impl nsIFormAutoCompleteObserverCoerce for nsIFormAutoCompleteObserver {
    #[inline]
    fn coerce_from(v: &nsIFormAutoCompleteObserver) -> &Self {
        v
    }
}

impl nsIFormAutoCompleteObserver {
    #[inline]
    pub fn coerce<T: nsIFormAutoCompleteObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFormAutoCompleteObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFormAutoCompleteObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFormAutoCompleteObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFormAutoCompleteObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onSearchCompletion (in nsIAutoCompleteResult result); */
    pub onSearchCompletion: unsafe extern "C" fn (this: *const nsIFormAutoCompleteObserver, result: *const nsIAutoCompleteResult) -> nsresult,

}


impl nsIFormAutoCompleteObserver {
    /* void onSearchCompletion (in nsIAutoCompleteResult result); */
    #[inline]
    pub unsafe fn onSearchCompletion(&self, result: Option<&nsIAutoCompleteResult>) -> Result<(), nsresult> {

        match ((*self.vtable).onSearchCompletion)(self as *const _, result.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


