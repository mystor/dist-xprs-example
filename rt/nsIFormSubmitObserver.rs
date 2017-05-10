//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFormSubmitObserver.idl
//


#[repr(C)]
pub struct nsIFormSubmitObserver {
    vtable: *const nsIFormSubmitObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFormSubmitObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x867cb7e7, 0x835d, 0x408b,
            [0x97, 0x88, 0xd2, 0x83, 0x4d, 0x28, 0x4e, 0x03])
    }
}

unsafe impl RefCounted for nsIFormSubmitObserver {
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
pub trait nsIFormSubmitObserverCoerce {
    fn coerce_from(v: &nsIFormSubmitObserver) -> &Self;
}

impl nsIFormSubmitObserverCoerce for nsIFormSubmitObserver {
    #[inline]
    fn coerce_from(v: &nsIFormSubmitObserver) -> &Self {
        v
    }
}

impl nsIFormSubmitObserver {
    #[inline]
    pub fn coerce<T: nsIFormSubmitObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFormSubmitObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFormSubmitObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFormSubmitObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFormSubmitObserverVTable {
    pub __base: nsISupportsVTable,

    /* void notify (in nsIDOMHTMLFormElement formNode, in mozIDOMWindow window, in nsIURI actionURL, out boolean cancelSubmit); */
    pub notify: unsafe extern "C" fn (this: *const nsIFormSubmitObserver, formNode: *const nsIDOMHTMLFormElement, window: *const mozIDOMWindow, actionURL: *const nsIURI, cancelSubmit: *mut bool) -> nsresult,

    /* void notifyInvalidSubmit (in nsIDOMHTMLFormElement formNode, in nsIArray invalidElements); */
    pub notifyInvalidSubmit: unsafe extern "C" fn (this: *const nsIFormSubmitObserver, formNode: *const nsIDOMHTMLFormElement, invalidElements: *const nsIArray) -> nsresult,

}


impl nsIFormSubmitObserver {
    /* void notify (in nsIDOMHTMLFormElement formNode, in mozIDOMWindow window, in nsIURI actionURL, out boolean cancelSubmit); */
    #[inline]
    pub unsafe fn notify(&self, formNode: Option<&nsIDOMHTMLFormElement>, window: Option<&mozIDOMWindow>, actionURL: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut cancelSubmit: bool = ::std::mem::zeroed();
        match ((*self.vtable).notify)(self as *const _, formNode.map_or(::std::ptr::null(), |x| x as *const _), window.map_or(::std::ptr::null(), |x| x as *const _), actionURL.map_or(::std::ptr::null(), |x| x as *const _), &mut cancelSubmit as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(cancelSubmit)
    }

    /* void notifyInvalidSubmit (in nsIDOMHTMLFormElement formNode, in nsIArray invalidElements); */
    #[inline]
    pub unsafe fn notifyInvalidSubmit(&self, formNode: Option<&nsIDOMHTMLFormElement>, invalidElements: Option<&nsIArray>) -> Result<(), nsresult> {

        match ((*self.vtable).notifyInvalidSubmit)(self as *const _, formNode.map_or(::std::ptr::null(), |x| x as *const _), invalidElements.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


