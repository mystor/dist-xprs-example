//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULRelatedElement.idl
//


#[repr(C)]
pub struct nsIDOMXULRelatedElement {
    vtable: *const nsIDOMXULRelatedElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULRelatedElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9fbac05a, 0xfb27, 0x470d,
            [0x8e, 0x5f, 0x02, 0x8b, 0x2d, 0xc5, 0x4a, 0xd0])
    }
}

unsafe impl RefCounted for nsIDOMXULRelatedElement {
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
pub trait nsIDOMXULRelatedElementCoerce {
    fn coerce_from(v: &nsIDOMXULRelatedElement) -> &Self;
}

impl nsIDOMXULRelatedElementCoerce for nsIDOMXULRelatedElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULRelatedElement) -> &Self {
        v
    }
}

impl nsIDOMXULRelatedElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULRelatedElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULRelatedElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMXULRelatedElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULRelatedElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULRelatedElementVTable {
    pub __base: nsISupportsVTable,

    /* nsIDOMNode getRelatedElement (in nsIDOMNode aElement); */
    pub getRelatedElement: unsafe extern "C" fn (this: *const nsIDOMXULRelatedElement, aElement: *const nsIDOMNode, _retval: *mut *const nsIDOMNode) -> nsresult,

}


impl nsIDOMXULRelatedElement {
    /* nsIDOMNode getRelatedElement (in nsIDOMNode aElement); */
    #[inline]
    pub unsafe fn getRelatedElement(&self, aElement: Option<&nsIDOMNode>) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRelatedElement)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


