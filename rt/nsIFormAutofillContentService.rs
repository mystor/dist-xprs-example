//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFormAutofillContentService.idl
//


#[repr(C)]
pub struct nsIFormAutofillContentService {
    vtable: *const nsIFormAutofillContentServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFormAutofillContentService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1db29340, 0x99df, 0x4845,
            [0x91, 0x02, 0x0c, 0x5d, 0x28, 0x1b, 0x2f, 0xe8])
    }
}

unsafe impl RefCounted for nsIFormAutofillContentService {
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
pub trait nsIFormAutofillContentServiceCoerce {
    fn coerce_from(v: &nsIFormAutofillContentService) -> &Self;
}

impl nsIFormAutofillContentServiceCoerce for nsIFormAutofillContentService {
    #[inline]
    fn coerce_from(v: &nsIFormAutofillContentService) -> &Self {
        v
    }
}

impl nsIFormAutofillContentService {
    #[inline]
    pub fn coerce<T: nsIFormAutofillContentServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFormAutofillContentService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFormAutofillContentServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFormAutofillContentService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFormAutofillContentServiceVTable {
    pub __base: nsISupportsVTable,

    /* void requestAutocomplete (in nsIDOMHTMLFormElement aForm, in nsIDOMWindow aWindow); */
    pub requestAutocomplete: unsafe extern "C" fn (this: *const nsIFormAutofillContentService, aForm: *const nsIDOMHTMLFormElement, aWindow: *const nsIDOMWindow) -> nsresult,

}


impl nsIFormAutofillContentService {
    /* void requestAutocomplete (in nsIDOMHTMLFormElement aForm, in nsIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn requestAutocomplete(&self, aForm: Option<&nsIDOMHTMLFormElement>, aWindow: Option<&nsIDOMWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).requestAutocomplete)(self as *const _, aForm.map_or(::std::ptr::null(), |x| x as *const _), aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


