//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIInputListAutoComplete.idl
//


#[repr(C)]
pub struct nsIInputListAutoComplete {
    vtable: *const nsIInputListAutoCompleteVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIInputListAutoComplete {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0e33de3e, 0x4faf, 0x4a1a,
            [0xb9, 0x6e, 0x24, 0x11, 0x5b, 0x8b, 0xfd, 0x45])
    }
}

unsafe impl RefCounted for nsIInputListAutoComplete {
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
pub trait nsIInputListAutoCompleteCoerce {
    fn coerce_from(v: &nsIInputListAutoComplete) -> &Self;
}

impl nsIInputListAutoCompleteCoerce for nsIInputListAutoComplete {
    #[inline]
    fn coerce_from(v: &nsIInputListAutoComplete) -> &Self {
        v
    }
}

impl nsIInputListAutoComplete {
    #[inline]
    pub fn coerce<T: nsIInputListAutoCompleteCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIInputListAutoComplete {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIInputListAutoCompleteCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputListAutoComplete) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIInputListAutoCompleteVTable {
    pub __base: nsISupportsVTable,

    /* nsIAutoCompleteResult autoCompleteSearch (in AString aSearchString, in nsIDOMHTMLInputElement aField); */
    pub autoCompleteSearch: unsafe extern "C" fn (this: *const nsIInputListAutoComplete, aSearchString: *const nsAString, aField: *const nsIDOMHTMLInputElement, _retval: *mut *const nsIAutoCompleteResult) -> nsresult,

}


impl nsIInputListAutoComplete {
    /* nsIAutoCompleteResult autoCompleteSearch (in AString aSearchString, in nsIDOMHTMLInputElement aField); */
    #[inline]
    pub unsafe fn autoCompleteSearch(&self, aSearchString: &[u16], aField: Option<&nsIDOMHTMLInputElement>) -> Result<Option<RefPtr<nsIAutoCompleteResult>>, nsresult> {
        let aSearchString = nsString::from(aSearchString);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).autoCompleteSearch)(self as *const _, &*aSearchString, aField.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


