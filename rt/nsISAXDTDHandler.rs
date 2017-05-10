//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXDTDHandler.idl
//


#[repr(C)]
pub struct nsISAXDTDHandler {
    vtable: *const nsISAXDTDHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISAXDTDHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4d01f225, 0x6cc5, 0x11da,
            [0xbe, 0x43, 0x00, 0x14, 0x22, 0x10, 0x69, 0x90])
    }
}

unsafe impl RefCounted for nsISAXDTDHandler {
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
pub trait nsISAXDTDHandlerCoerce {
    fn coerce_from(v: &nsISAXDTDHandler) -> &Self;
}

impl nsISAXDTDHandlerCoerce for nsISAXDTDHandler {
    #[inline]
    fn coerce_from(v: &nsISAXDTDHandler) -> &Self {
        v
    }
}

impl nsISAXDTDHandler {
    #[inline]
    pub fn coerce<T: nsISAXDTDHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISAXDTDHandler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISAXDTDHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISAXDTDHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISAXDTDHandlerVTable {
    pub __base: nsISupportsVTable,

    /* void notationDecl (in AString name, in AString publicId, in AString systemId); */
    pub notationDecl: unsafe extern "C" fn (this: *const nsISAXDTDHandler, name: *const nsAString, publicId: *const nsAString, systemId: *const nsAString) -> nsresult,

    /* void unparsedEntityDecl (in AString name, in AString publicId, in AString systemId, in AString notationName); */
    pub unparsedEntityDecl: unsafe extern "C" fn (this: *const nsISAXDTDHandler, name: *const nsAString, publicId: *const nsAString, systemId: *const nsAString, notationName: *const nsAString) -> nsresult,

}


impl nsISAXDTDHandler {
    /* void notationDecl (in AString name, in AString publicId, in AString systemId); */
    #[inline]
    pub unsafe fn notationDecl(&self, name: &[u16], publicId: &[u16], systemId: &[u16]) -> Result<(), nsresult> {
        let name = nsString::from(name);
        let publicId = nsString::from(publicId);
        let systemId = nsString::from(systemId);
        match ((*self.vtable).notationDecl)(self as *const _, &*name, &*publicId, &*systemId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unparsedEntityDecl (in AString name, in AString publicId, in AString systemId, in AString notationName); */
    #[inline]
    pub unsafe fn unparsedEntityDecl(&self, name: &[u16], publicId: &[u16], systemId: &[u16], notationName: &[u16]) -> Result<(), nsresult> {
        let name = nsString::from(name);
        let publicId = nsString::from(publicId);
        let systemId = nsString::from(systemId);
        let notationName = nsString::from(notationName);
        match ((*self.vtable).unparsedEntityDecl)(self as *const _, &*name, &*publicId, &*systemId, &*notationName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


