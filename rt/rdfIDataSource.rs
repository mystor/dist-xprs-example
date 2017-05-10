//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/rdfIDataSource.idl
//


#[repr(C)]
pub struct rdfIDataSource {
    vtable: *const rdfIDataSourceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for rdfIDataSource {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xebce86bd, 0x1568, 0x4a34,
            [0xa8, 0x08, 0x9c, 0xcf, 0x9c, 0xde, 0x80, 0x87])
    }
}

unsafe impl RefCounted for rdfIDataSource {
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
pub trait rdfIDataSourceCoerce {
    fn coerce_from(v: &rdfIDataSource) -> &Self;
}

impl rdfIDataSourceCoerce for rdfIDataSource {
    #[inline]
    fn coerce_from(v: &rdfIDataSource) -> &Self {
        v
    }
}

impl rdfIDataSource {
    #[inline]
    pub fn coerce<T: rdfIDataSourceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for rdfIDataSource {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> rdfIDataSourceCoerce for T {
    #[inline]
    fn coerce_from(v: &rdfIDataSource) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct rdfIDataSourceVTable {
    pub __base: nsISupportsVTable,

    /* void visitAllSubjects (in rdfITripleVisitor aVisitor); */
    pub visitAllSubjects: unsafe extern "C" fn (this: *const rdfIDataSource, aVisitor: *const rdfITripleVisitor) -> nsresult,

    /* void visitAllTriples (in rdfITripleVisitor aVisitor); */
    pub visitAllTriples: unsafe extern "C" fn (this: *const rdfIDataSource, aVisitor: *const rdfITripleVisitor) -> nsresult,

}


impl rdfIDataSource {
    /* void visitAllSubjects (in rdfITripleVisitor aVisitor); */
    #[inline]
    pub unsafe fn visitAllSubjects(&self, aVisitor: Option<&rdfITripleVisitor>) -> Result<(), nsresult> {

        match ((*self.vtable).visitAllSubjects)(self as *const _, aVisitor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void visitAllTriples (in rdfITripleVisitor aVisitor); */
    #[inline]
    pub unsafe fn visitAllTriples(&self, aVisitor: Option<&rdfITripleVisitor>) -> Result<(), nsresult> {

        match ((*self.vtable).visitAllTriples)(self as *const _, aVisitor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


