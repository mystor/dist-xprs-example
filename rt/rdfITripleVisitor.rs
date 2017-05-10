//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/rdfITripleVisitor.idl
//


#[repr(C)]
pub struct rdfITripleVisitor {
    vtable: *const rdfITripleVisitorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for rdfITripleVisitor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xaafea151, 0xc271, 0x4505,
            [0x99, 0x78, 0xa1, 0x00, 0xd2, 0x92, 0x80, 0x0c])
    }
}

unsafe impl RefCounted for rdfITripleVisitor {
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
pub trait rdfITripleVisitorCoerce {
    fn coerce_from(v: &rdfITripleVisitor) -> &Self;
}

impl rdfITripleVisitorCoerce for rdfITripleVisitor {
    #[inline]
    fn coerce_from(v: &rdfITripleVisitor) -> &Self {
        v
    }
}

impl rdfITripleVisitor {
    #[inline]
    pub fn coerce<T: rdfITripleVisitorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for rdfITripleVisitor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> rdfITripleVisitorCoerce for T {
    #[inline]
    fn coerce_from(v: &rdfITripleVisitor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct rdfITripleVisitorVTable {
    pub __base: nsISupportsVTable,

    /* void visit (in nsIRDFNode aSubject, in nsIRDFResource aPredicate, in nsIRDFNode aObject, in boolean aTruthValue); */
    pub visit: unsafe extern "C" fn (this: *const rdfITripleVisitor, aSubject: *const nsIRDFNode, aPredicate: *const nsIRDFResource, aObject: *const nsIRDFNode, aTruthValue: bool) -> nsresult,

}


impl rdfITripleVisitor {
    /* void visit (in nsIRDFNode aSubject, in nsIRDFResource aPredicate, in nsIRDFNode aObject, in boolean aTruthValue); */
    #[inline]
    pub unsafe fn visit(&self, aSubject: Option<&nsIRDFNode>, aPredicate: Option<&nsIRDFResource>, aObject: Option<&nsIRDFNode>, aTruthValue: bool) -> Result<(), nsresult> {

        match ((*self.vtable).visit)(self as *const _, aSubject.map_or(::std::ptr::null(), |x| x as *const _), aPredicate.map_or(::std::ptr::null(), |x| x as *const _), aObject.map_or(::std::ptr::null(), |x| x as *const _), aTruthValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


