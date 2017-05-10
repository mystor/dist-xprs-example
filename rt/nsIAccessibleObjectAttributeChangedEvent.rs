//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleObjectAttributeChangedEvent.idl
//


#[repr(C)]
pub struct nsIAccessibleObjectAttributeChangedEvent {
    vtable: *const nsIAccessibleObjectAttributeChangedEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleObjectAttributeChangedEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xce41add2, 0x096e, 0x4606,
            [0xb1, 0xca, 0x74, 0x08, 0xc6, 0xd5, 0xb4, 0xc3])
    }
}

unsafe impl RefCounted for nsIAccessibleObjectAttributeChangedEvent {
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
pub trait nsIAccessibleObjectAttributeChangedEventCoerce {
    fn coerce_from(v: &nsIAccessibleObjectAttributeChangedEvent) -> &Self;
}

impl nsIAccessibleObjectAttributeChangedEventCoerce for nsIAccessibleObjectAttributeChangedEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleObjectAttributeChangedEvent) -> &Self {
        v
    }
}

impl nsIAccessibleObjectAttributeChangedEvent {
    #[inline]
    pub fn coerce<T: nsIAccessibleObjectAttributeChangedEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleObjectAttributeChangedEvent {
    type Target = nsIAccessibleEvent;
    #[inline]
    fn deref(&self) -> &nsIAccessibleEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIAccessibleEventCoerce> nsIAccessibleObjectAttributeChangedEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleObjectAttributeChangedEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleObjectAttributeChangedEventVTable {
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute nsIAtom changedAttribute; */
    pub get_changedAttribute: unsafe extern "C" fn (this: *const nsIAccessibleObjectAttributeChangedEvent, aChangedAttribute: *mut *const nsIAtom) -> nsresult,

}


impl nsIAccessibleObjectAttributeChangedEvent {
    /* readonly attribute nsIAtom changedAttribute; */
    #[inline]
    pub unsafe fn get_changedAttribute(&self, ) -> Result<Option<RefPtr<nsIAtom>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_changedAttribute)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


