//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFXMLSource.idl
//


#[repr(C)]
pub struct nsIRDFXMLSource {
    vtable: *const nsIRDFXMLSourceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFXMLSource {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4da56f10, 0x99fe, 0x11d2,
            [0x8e, 0xbb, 0x00, 0x80, 0x5f, 0x29, 0xf3, 0x70])
    }
}

unsafe impl RefCounted for nsIRDFXMLSource {
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
pub trait nsIRDFXMLSourceCoerce {
    fn coerce_from(v: &nsIRDFXMLSource) -> &Self;
}

impl nsIRDFXMLSourceCoerce for nsIRDFXMLSource {
    #[inline]
    fn coerce_from(v: &nsIRDFXMLSource) -> &Self {
        v
    }
}

impl nsIRDFXMLSource {
    #[inline]
    pub fn coerce<T: nsIRDFXMLSourceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFXMLSource {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFXMLSourceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFXMLSource) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFXMLSourceVTable {
    pub __base: nsISupportsVTable,

    /* void Serialize (in nsIOutputStream aStream); */
    pub Serialize: unsafe extern "C" fn (this: *const nsIRDFXMLSource, aStream: *const nsIOutputStream) -> nsresult,

}


impl nsIRDFXMLSource {
    /* void Serialize (in nsIOutputStream aStream); */
    #[inline]
    pub unsafe fn Serialize(&self, aStream: Option<&nsIOutputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).Serialize)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


