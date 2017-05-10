//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFormFillController.idl
//


#[repr(C)]
pub struct nsIFormFillController {
    vtable: *const nsIFormFillControllerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFormFillController {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x07f0a0dc, 0xf6e9, 0x4cdd,
            [0xa5, 0x5f, 0x56, 0xd7, 0x70, 0x52, 0x3a, 0x4c])
    }
}

unsafe impl RefCounted for nsIFormFillController {
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
pub trait nsIFormFillControllerCoerce {
    fn coerce_from(v: &nsIFormFillController) -> &Self;
}

impl nsIFormFillControllerCoerce for nsIFormFillController {
    #[inline]
    fn coerce_from(v: &nsIFormFillController) -> &Self {
        v
    }
}

impl nsIFormFillController {
    #[inline]
    pub fn coerce<T: nsIFormFillControllerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFormFillController {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFormFillControllerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFormFillController) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFormFillControllerVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMHTMLInputElement focusedInput; */
    pub get_focusedInput: unsafe extern "C" fn (this: *const nsIFormFillController, aFocusedInput: *mut *const nsIDOMHTMLInputElement) -> nsresult,

    /* void attachToBrowser (in nsIDocShell docShell, in nsIAutoCompletePopup popup); */
    pub attachToBrowser: unsafe extern "C" fn (this: *const nsIFormFillController, docShell: *const nsIDocShell, popup: *const nsIAutoCompletePopup) -> nsresult,

    /* void detachFromBrowser (in nsIDocShell docShell); */
    pub detachFromBrowser: unsafe extern "C" fn (this: *const nsIFormFillController, docShell: *const nsIDocShell) -> nsresult,

    /* void markAsLoginManagerField (in nsIDOMHTMLInputElement aInput); */
    pub markAsLoginManagerField: unsafe extern "C" fn (this: *const nsIFormFillController, aInput: *const nsIDOMHTMLInputElement) -> nsresult,

    /* void markAsAutofillField (in nsIDOMHTMLInputElement aInput); */
    pub markAsAutofillField: unsafe extern "C" fn (this: *const nsIFormFillController, aInput: *const nsIDOMHTMLInputElement) -> nsresult,

    /* void showPopup (); */
    pub showPopup: unsafe extern "C" fn (this: *const nsIFormFillController) -> nsresult,

}


impl nsIFormFillController {
    /* readonly attribute nsIDOMHTMLInputElement focusedInput; */
    #[inline]
    pub unsafe fn get_focusedInput(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLInputElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_focusedInput)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void attachToBrowser (in nsIDocShell docShell, in nsIAutoCompletePopup popup); */
    #[inline]
    pub unsafe fn attachToBrowser(&self, docShell: Option<&nsIDocShell>, popup: Option<&nsIAutoCompletePopup>) -> Result<(), nsresult> {

        match ((*self.vtable).attachToBrowser)(self as *const _, docShell.map_or(::std::ptr::null(), |x| x as *const _), popup.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void detachFromBrowser (in nsIDocShell docShell); */
    #[inline]
    pub unsafe fn detachFromBrowser(&self, docShell: Option<&nsIDocShell>) -> Result<(), nsresult> {

        match ((*self.vtable).detachFromBrowser)(self as *const _, docShell.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void markAsLoginManagerField (in nsIDOMHTMLInputElement aInput); */
    #[inline]
    pub unsafe fn markAsLoginManagerField(&self, aInput: Option<&nsIDOMHTMLInputElement>) -> Result<(), nsresult> {

        match ((*self.vtable).markAsLoginManagerField)(self as *const _, aInput.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void markAsAutofillField (in nsIDOMHTMLInputElement aInput); */
    #[inline]
    pub unsafe fn markAsAutofillField(&self, aInput: Option<&nsIDOMHTMLInputElement>) -> Result<(), nsresult> {

        match ((*self.vtable).markAsAutofillField)(self as *const _, aInput.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showPopup (); */
    #[inline]
    pub unsafe fn showPopup(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).showPopup)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


