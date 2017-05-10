//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEditorStyleSheets.idl
//


#[repr(C)]
pub struct nsIEditorStyleSheets {
    vtable: *const nsIEditorStyleSheetsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEditorStyleSheets {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4805e682, 0x49b9, 0x11d3,
            [0x9c, 0xe4, 0xed, 0x60, 0xbd, 0x6c, 0xb5, 0xbc])
    }
}

unsafe impl RefCounted for nsIEditorStyleSheets {
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
pub trait nsIEditorStyleSheetsCoerce {
    fn coerce_from(v: &nsIEditorStyleSheets) -> &Self;
}

impl nsIEditorStyleSheetsCoerce for nsIEditorStyleSheets {
    #[inline]
    fn coerce_from(v: &nsIEditorStyleSheets) -> &Self {
        v
    }
}

impl nsIEditorStyleSheets {
    #[inline]
    pub fn coerce<T: nsIEditorStyleSheetsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEditorStyleSheets {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEditorStyleSheetsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditorStyleSheets) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEditorStyleSheetsVTable {
    pub __base: nsISupportsVTable,

    /* void replaceStyleSheet (in AString aURL); */
    pub replaceStyleSheet: unsafe extern "C" fn (this: *const nsIEditorStyleSheets, aURL: *const nsAString) -> nsresult,

    /* void addStyleSheet (in AString aURL); */
    pub addStyleSheet: unsafe extern "C" fn (this: *const nsIEditorStyleSheets, aURL: *const nsAString) -> nsresult,

    /* void replaceOverrideStyleSheet (in AString aURL); */
    pub replaceOverrideStyleSheet: unsafe extern "C" fn (this: *const nsIEditorStyleSheets, aURL: *const nsAString) -> nsresult,

    /* void addOverrideStyleSheet (in AString aURL); */
    pub addOverrideStyleSheet: unsafe extern "C" fn (this: *const nsIEditorStyleSheets, aURL: *const nsAString) -> nsresult,

    /* void removeStyleSheet (in AString aURL); */
    pub removeStyleSheet: unsafe extern "C" fn (this: *const nsIEditorStyleSheets, aURL: *const nsAString) -> nsresult,

    /* void removeOverrideStyleSheet (in AString aURL); */
    pub removeOverrideStyleSheet: unsafe extern "C" fn (this: *const nsIEditorStyleSheets, aURL: *const nsAString) -> nsresult,

    /* void enableStyleSheet (in AString aURL, in boolean aEnable); */
    pub enableStyleSheet: unsafe extern "C" fn (this: *const nsIEditorStyleSheets, aURL: *const nsAString, aEnable: bool) -> nsresult,

}


impl nsIEditorStyleSheets {
    /* void replaceStyleSheet (in AString aURL); */
    #[inline]
    pub unsafe fn replaceStyleSheet(&self, aURL: &[u16]) -> Result<(), nsresult> {
        let aURL = nsString::from(aURL);
        match ((*self.vtable).replaceStyleSheet)(self as *const _, &*aURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addStyleSheet (in AString aURL); */
    #[inline]
    pub unsafe fn addStyleSheet(&self, aURL: &[u16]) -> Result<(), nsresult> {
        let aURL = nsString::from(aURL);
        match ((*self.vtable).addStyleSheet)(self as *const _, &*aURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void replaceOverrideStyleSheet (in AString aURL); */
    #[inline]
    pub unsafe fn replaceOverrideStyleSheet(&self, aURL: &[u16]) -> Result<(), nsresult> {
        let aURL = nsString::from(aURL);
        match ((*self.vtable).replaceOverrideStyleSheet)(self as *const _, &*aURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addOverrideStyleSheet (in AString aURL); */
    #[inline]
    pub unsafe fn addOverrideStyleSheet(&self, aURL: &[u16]) -> Result<(), nsresult> {
        let aURL = nsString::from(aURL);
        match ((*self.vtable).addOverrideStyleSheet)(self as *const _, &*aURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeStyleSheet (in AString aURL); */
    #[inline]
    pub unsafe fn removeStyleSheet(&self, aURL: &[u16]) -> Result<(), nsresult> {
        let aURL = nsString::from(aURL);
        match ((*self.vtable).removeStyleSheet)(self as *const _, &*aURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeOverrideStyleSheet (in AString aURL); */
    #[inline]
    pub unsafe fn removeOverrideStyleSheet(&self, aURL: &[u16]) -> Result<(), nsresult> {
        let aURL = nsString::from(aURL);
        match ((*self.vtable).removeOverrideStyleSheet)(self as *const _, &*aURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void enableStyleSheet (in AString aURL, in boolean aEnable); */
    #[inline]
    pub unsafe fn enableStyleSheet(&self, aURL: &[u16], aEnable: bool) -> Result<(), nsresult> {
        let aURL = nsString::from(aURL);
        match ((*self.vtable).enableStyleSheet)(self as *const _, &*aURL, aEnable) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


