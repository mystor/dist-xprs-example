//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMozSAXXMLDeclarationHandler.idl
//


#[repr(C)]
pub struct nsIMozSAXXMLDeclarationHandler {
    vtable: *const nsIMozSAXXMLDeclarationHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMozSAXXMLDeclarationHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc0e461cb, 0x0e5e, 0x284c,
            [0xb9, 0x7d, 0xcf, 0xfe, 0xec, 0x46, 0x7e, 0xba])
    }
}

unsafe impl RefCounted for nsIMozSAXXMLDeclarationHandler {
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
pub trait nsIMozSAXXMLDeclarationHandlerCoerce {
    fn coerce_from(v: &nsIMozSAXXMLDeclarationHandler) -> &Self;
}

impl nsIMozSAXXMLDeclarationHandlerCoerce for nsIMozSAXXMLDeclarationHandler {
    #[inline]
    fn coerce_from(v: &nsIMozSAXXMLDeclarationHandler) -> &Self {
        v
    }
}

impl nsIMozSAXXMLDeclarationHandler {
    #[inline]
    pub fn coerce<T: nsIMozSAXXMLDeclarationHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMozSAXXMLDeclarationHandler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMozSAXXMLDeclarationHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMozSAXXMLDeclarationHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMozSAXXMLDeclarationHandlerVTable {
    pub __base: nsISupportsVTable,

    /* void handleXMLDeclaration (in AString version, in AString encoding, in boolean standalone); */
    pub handleXMLDeclaration: unsafe extern "C" fn (this: *const nsIMozSAXXMLDeclarationHandler, version: *const nsAString, encoding: *const nsAString, standalone: bool) -> nsresult,

}


impl nsIMozSAXXMLDeclarationHandler {
    /* void handleXMLDeclaration (in AString version, in AString encoding, in boolean standalone); */
    #[inline]
    pub unsafe fn handleXMLDeclaration(&self, version: &[u16], encoding: &[u16], standalone: bool) -> Result<(), nsresult> {
        let version = nsString::from(version);
        let encoding = nsString::from(encoding);
        match ((*self.vtable).handleXMLDeclaration)(self as *const _, &*version, &*encoding, standalone) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


