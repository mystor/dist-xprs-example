//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITextServicesFilter.idl
//


#[repr(C)]
pub struct nsITextServicesFilter {
    vtable: *const nsITextServicesFilterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITextServicesFilter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5bec321f, 0x59ac, 0x413a,
            [0xa4, 0xad, 0x8a, 0x8d, 0x7c, 0x50, 0xa0, 0xd0])
    }
}

unsafe impl RefCounted for nsITextServicesFilter {
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
pub trait nsITextServicesFilterCoerce {
    fn coerce_from(v: &nsITextServicesFilter) -> &Self;
}

impl nsITextServicesFilterCoerce for nsITextServicesFilter {
    #[inline]
    fn coerce_from(v: &nsITextServicesFilter) -> &Self {
        v
    }
}

impl nsITextServicesFilter {
    #[inline]
    pub fn coerce<T: nsITextServicesFilterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITextServicesFilter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITextServicesFilterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITextServicesFilter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITextServicesFilterVTable {
    pub __base: nsISupportsVTable,

    /* boolean skip (in nsIDOMNode aNode); */
    pub skip: unsafe extern "C" fn (this: *const nsITextServicesFilter, aNode: *const nsIDOMNode, _retval: *mut bool) -> nsresult,

}


impl nsITextServicesFilter {
    /* boolean skip (in nsIDOMNode aNode); */
    #[inline]
    pub unsafe fn skip(&self, aNode: Option<&nsIDOMNode>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).skip)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


