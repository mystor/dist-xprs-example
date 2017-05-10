//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserChromeFocus.idl
//


#[repr(C)]
pub struct nsIWebBrowserChromeFocus {
    vtable: *const nsIWebBrowserChromeFocusVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserChromeFocus {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x947b2ee6, 0x51ed, 0x4c2b,
            [0x9f, 0x45, 0x42, 0x6c, 0x27, 0xca, 0x84, 0xc6])
    }
}

unsafe impl RefCounted for nsIWebBrowserChromeFocus {
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
pub trait nsIWebBrowserChromeFocusCoerce {
    fn coerce_from(v: &nsIWebBrowserChromeFocus) -> &Self;
}

impl nsIWebBrowserChromeFocusCoerce for nsIWebBrowserChromeFocus {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserChromeFocus) -> &Self {
        v
    }
}

impl nsIWebBrowserChromeFocus {
    #[inline]
    pub fn coerce<T: nsIWebBrowserChromeFocusCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserChromeFocus {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebBrowserChromeFocusCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserChromeFocus) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserChromeFocusVTable {
    pub __base: nsISupportsVTable,

    /* void focusNextElement (in bool aForDocumentNavigation); */
    pub focusNextElement: unsafe extern "C" fn (this: *const nsIWebBrowserChromeFocus, aForDocumentNavigation: bool) -> nsresult,

    /* void focusPrevElement (in bool aForDocumentNavigation); */
    pub focusPrevElement: unsafe extern "C" fn (this: *const nsIWebBrowserChromeFocus, aForDocumentNavigation: bool) -> nsresult,

}


impl nsIWebBrowserChromeFocus {
    /* void focusNextElement (in bool aForDocumentNavigation); */
    #[inline]
    pub unsafe fn focusNextElement(&self, aForDocumentNavigation: bool) -> Result<(), nsresult> {

        match ((*self.vtable).focusNextElement)(self as *const _, aForDocumentNavigation) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void focusPrevElement (in bool aForDocumentNavigation); */
    #[inline]
    pub unsafe fn focusPrevElement(&self, aForDocumentNavigation: bool) -> Result<(), nsresult> {

        match ((*self.vtable).focusPrevElement)(self as *const _, aForDocumentNavigation) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


