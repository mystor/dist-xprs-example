//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFXMLSerializer.idl
//


#[repr(C)]
pub struct nsIRDFXMLSerializer {
    vtable: *const nsIRDFXMLSerializerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFXMLSerializer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8ae1fbf8, 0x1dd2, 0x11b2,
            [0xbd, 0x21, 0xd7, 0x28, 0x06, 0x9c, 0xca, 0x92])
    }
}

unsafe impl RefCounted for nsIRDFXMLSerializer {
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
pub trait nsIRDFXMLSerializerCoerce {
    fn coerce_from(v: &nsIRDFXMLSerializer) -> &Self;
}

impl nsIRDFXMLSerializerCoerce for nsIRDFXMLSerializer {
    #[inline]
    fn coerce_from(v: &nsIRDFXMLSerializer) -> &Self {
        v
    }
}

impl nsIRDFXMLSerializer {
    #[inline]
    pub fn coerce<T: nsIRDFXMLSerializerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFXMLSerializer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFXMLSerializerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFXMLSerializer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFXMLSerializerVTable {
    pub __base: nsISupportsVTable,

    /* void init (in nsIRDFDataSource aDataSource); */
    pub init: unsafe extern "C" fn (this: *const nsIRDFXMLSerializer, aDataSource: *const nsIRDFDataSource) -> nsresult,

    /* void addNameSpace (in nsIAtom aPrefix, in DOMString aURI); */
    pub addNameSpace: unsafe extern "C" fn (this: *const nsIRDFXMLSerializer, aPrefix: *const nsIAtom, aURI: *const nsAString) -> nsresult,

}


impl nsIRDFXMLSerializer {
    /* void init (in nsIRDFDataSource aDataSource); */
    #[inline]
    pub unsafe fn init(&self, aDataSource: Option<&nsIRDFDataSource>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addNameSpace (in nsIAtom aPrefix, in DOMString aURI); */
    #[inline]
    pub unsafe fn addNameSpace(&self, aPrefix: Option<&nsIAtom>, aURI: &[u16]) -> Result<(), nsresult> {
        let aURI = nsString::from(aURI);
        match ((*self.vtable).addNameSpace)(self as *const _, aPrefix.map_or(::std::ptr::null(), |x| x as *const _), &*aURI) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


