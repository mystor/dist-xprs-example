//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMFocusEvent.idl
//


#[repr(C)]
pub struct nsIDOMFocusEvent {
    vtable: *const nsIDOMFocusEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMFocusEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xceab9fcd, 0x2cae, 0x42cb,
            [0xb6, 0x92, 0xef, 0xfa, 0x7e, 0xc4, 0x88, 0x48])
    }
}

unsafe impl RefCounted for nsIDOMFocusEvent {
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
pub trait nsIDOMFocusEventCoerce {
    fn coerce_from(v: &nsIDOMFocusEvent) -> &Self;
}

impl nsIDOMFocusEventCoerce for nsIDOMFocusEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMFocusEvent) -> &Self {
        v
    }
}

impl nsIDOMFocusEvent {
    #[inline]
    pub fn coerce<T: nsIDOMFocusEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMFocusEvent {
    type Target = nsIDOMUIEvent;
    #[inline]
    fn deref(&self) -> &nsIDOMUIEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMUIEventCoerce> nsIDOMFocusEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMFocusEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMFocusEventVTable {
    pub __base: nsIDOMUIEventVTable,

    /* readonly attribute nsIDOMEventTarget relatedTarget; */
    pub get_relatedTarget: unsafe extern "C" fn (this: *const nsIDOMFocusEvent, aRelatedTarget: *mut *const nsIDOMEventTarget) -> nsresult,

}


impl nsIDOMFocusEvent {
    /* readonly attribute nsIDOMEventTarget relatedTarget; */
    #[inline]
    pub unsafe fn get_relatedTarget(&self, ) -> Result<Option<RefPtr<nsIDOMEventTarget>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_relatedTarget)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


