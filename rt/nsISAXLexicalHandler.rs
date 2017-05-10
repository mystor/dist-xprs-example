//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXLexicalHandler.idl
//


#[repr(C)]
pub struct nsISAXLexicalHandler {
    vtable: *const nsISAXLexicalHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISAXLexicalHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x23c26a56, 0xadff, 0x440c,
            [0x8c, 0xaf, 0x95, 0xc2, 0xdc, 0x2e, 0x39, 0x9b])
    }
}

unsafe impl RefCounted for nsISAXLexicalHandler {
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
pub trait nsISAXLexicalHandlerCoerce {
    fn coerce_from(v: &nsISAXLexicalHandler) -> &Self;
}

impl nsISAXLexicalHandlerCoerce for nsISAXLexicalHandler {
    #[inline]
    fn coerce_from(v: &nsISAXLexicalHandler) -> &Self {
        v
    }
}

impl nsISAXLexicalHandler {
    #[inline]
    pub fn coerce<T: nsISAXLexicalHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISAXLexicalHandler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISAXLexicalHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISAXLexicalHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISAXLexicalHandlerVTable {
    pub __base: nsISupportsVTable,

    /* void comment (in AString chars); */
    pub comment: unsafe extern "C" fn (this: *const nsISAXLexicalHandler, chars: *const nsAString) -> nsresult,

    /* void startDTD (in AString name, in AString publicId, in AString systemId); */
    pub startDTD: unsafe extern "C" fn (this: *const nsISAXLexicalHandler, name: *const nsAString, publicId: *const nsAString, systemId: *const nsAString) -> nsresult,

    /* void endDTD (); */
    pub endDTD: unsafe extern "C" fn (this: *const nsISAXLexicalHandler) -> nsresult,

    /* void startCDATA (); */
    pub startCDATA: unsafe extern "C" fn (this: *const nsISAXLexicalHandler) -> nsresult,

    /* void endCDATA (); */
    pub endCDATA: unsafe extern "C" fn (this: *const nsISAXLexicalHandler) -> nsresult,

    /* void startEntity (in AString name); */
    pub startEntity: unsafe extern "C" fn (this: *const nsISAXLexicalHandler, name: *const nsAString) -> nsresult,

    /* void endEntity (in AString name); */
    pub endEntity: unsafe extern "C" fn (this: *const nsISAXLexicalHandler, name: *const nsAString) -> nsresult,

}


impl nsISAXLexicalHandler {
    /* void comment (in AString chars); */
    #[inline]
    pub unsafe fn comment(&self, chars: &[u16]) -> Result<(), nsresult> {
        let chars = nsString::from(chars);
        match ((*self.vtable).comment)(self as *const _, &*chars) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void startDTD (in AString name, in AString publicId, in AString systemId); */
    #[inline]
    pub unsafe fn startDTD(&self, name: &[u16], publicId: &[u16], systemId: &[u16]) -> Result<(), nsresult> {
        let name = nsString::from(name);
        let publicId = nsString::from(publicId);
        let systemId = nsString::from(systemId);
        match ((*self.vtable).startDTD)(self as *const _, &*name, &*publicId, &*systemId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endDTD (); */
    #[inline]
    pub unsafe fn endDTD(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).endDTD)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void startCDATA (); */
    #[inline]
    pub unsafe fn startCDATA(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).startCDATA)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endCDATA (); */
    #[inline]
    pub unsafe fn endCDATA(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).endCDATA)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void startEntity (in AString name); */
    #[inline]
    pub unsafe fn startEntity(&self, name: &[u16]) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).startEntity)(self as *const _, &*name) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endEntity (in AString name); */
    #[inline]
    pub unsafe fn endEntity(&self, name: &[u16]) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).endEntity)(self as *const _, &*name) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


