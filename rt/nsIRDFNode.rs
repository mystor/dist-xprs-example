//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFNode.idl
//


#[repr(C)]
pub struct nsIRDFNode {
    vtable: *const nsIRDFNodeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFNode {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0f78da50, 0x8321, 0x11d2,
            [0x8e, 0xac, 0x00, 0x80, 0x5f, 0x29, 0xf3, 0x70])
    }
}

unsafe impl RefCounted for nsIRDFNode {
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
pub trait nsIRDFNodeCoerce {
    fn coerce_from(v: &nsIRDFNode) -> &Self;
}

impl nsIRDFNodeCoerce for nsIRDFNode {
    #[inline]
    fn coerce_from(v: &nsIRDFNode) -> &Self {
        v
    }
}

impl nsIRDFNode {
    #[inline]
    pub fn coerce<T: nsIRDFNodeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFNode {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFNodeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFNode) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFNodeVTable {
    pub __base: nsISupportsVTable,

    /* boolean EqualsNode (in nsIRDFNode aNode); */
    pub EqualsNode: unsafe extern "C" fn (this: *const nsIRDFNode, aNode: *const nsIRDFNode, _retval: *mut bool) -> nsresult,

}


impl nsIRDFNode {
    /* boolean EqualsNode (in nsIRDFNode aNode); */
    #[inline]
    pub unsafe fn EqualsNode(&self, aNode: Option<&nsIRDFNode>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).EqualsNode)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


