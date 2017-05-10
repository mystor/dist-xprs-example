//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHTMLInlineTableEditor.idl
//


#[repr(C)]
pub struct nsIHTMLInlineTableEditor {
    vtable: *const nsIHTMLInlineTableEditorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHTMLInlineTableEditor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xeda2e65c, 0xa758, 0x451f,
            [0x9b, 0x05, 0x77, 0xcb, 0x8d, 0xe7, 0x4e, 0xd2])
    }
}

unsafe impl RefCounted for nsIHTMLInlineTableEditor {
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
pub trait nsIHTMLInlineTableEditorCoerce {
    fn coerce_from(v: &nsIHTMLInlineTableEditor) -> &Self;
}

impl nsIHTMLInlineTableEditorCoerce for nsIHTMLInlineTableEditor {
    #[inline]
    fn coerce_from(v: &nsIHTMLInlineTableEditor) -> &Self {
        v
    }
}

impl nsIHTMLInlineTableEditor {
    #[inline]
    pub fn coerce<T: nsIHTMLInlineTableEditorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHTMLInlineTableEditor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHTMLInlineTableEditorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHTMLInlineTableEditor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHTMLInlineTableEditorVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean inlineTableEditingEnabled; */
    pub get_inlineTableEditingEnabled: unsafe extern "C" fn (this: *const nsIHTMLInlineTableEditor, aInlineTableEditingEnabled: *mut bool) -> nsresult,
    pub set_inlineTableEditingEnabled: unsafe extern "C" fn (this: *const nsIHTMLInlineTableEditor, aInlineTableEditingEnabled: bool) -> nsresult,

    /* void showInlineTableEditingUI (in nsIDOMElement aCell); */
    pub showInlineTableEditingUI: unsafe extern "C" fn (this: *const nsIHTMLInlineTableEditor, aCell: *const nsIDOMElement) -> nsresult,

    /* void hideInlineTableEditingUI (); */
    pub hideInlineTableEditingUI: unsafe extern "C" fn (this: *const nsIHTMLInlineTableEditor) -> nsresult,

    /* void doInlineTableEditingAction (in nsIDOMElement aUIAnonymousElement); */
    pub doInlineTableEditingAction: unsafe extern "C" fn (this: *const nsIHTMLInlineTableEditor, aUIAnonymousElement: *const nsIDOMElement) -> nsresult,

    /* void refreshInlineTableEditingUI (); */
    pub refreshInlineTableEditingUI: unsafe extern "C" fn (this: *const nsIHTMLInlineTableEditor) -> nsresult,

}


impl nsIHTMLInlineTableEditor {
    /* attribute boolean inlineTableEditingEnabled; */
    #[inline]
    pub unsafe fn get_inlineTableEditingEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_inlineTableEditingEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_inlineTableEditingEnabled(&self, aInlineTableEditingEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_inlineTableEditingEnabled)(self as *const _, aInlineTableEditingEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showInlineTableEditingUI (in nsIDOMElement aCell); */
    #[inline]
    pub unsafe fn showInlineTableEditingUI(&self, aCell: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).showInlineTableEditingUI)(self as *const _, aCell.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void hideInlineTableEditingUI (); */
    #[inline]
    pub unsafe fn hideInlineTableEditingUI(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).hideInlineTableEditingUI)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void doInlineTableEditingAction (in nsIDOMElement aUIAnonymousElement); */
    #[inline]
    pub unsafe fn doInlineTableEditingAction(&self, aUIAnonymousElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).doInlineTableEditingAction)(self as *const _, aUIAnonymousElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void refreshInlineTableEditingUI (); */
    #[inline]
    pub unsafe fn refreshInlineTableEditingUI(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).refreshInlineTableEditingUI)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


