//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDragEvent.idl
//


#[repr(C)]
pub struct nsIDOMDragEvent {
    vtable: *const nsIDOMDragEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMDragEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd5c0d4c2, 0xc646, 0x4b4b,
            [0x83, 0x6e, 0x48, 0x40, 0x8b, 0x9c, 0x7b, 0x80])
    }
}

unsafe impl RefCounted for nsIDOMDragEvent {
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
pub trait nsIDOMDragEventCoerce {
    fn coerce_from(v: &nsIDOMDragEvent) -> &Self;
}

impl nsIDOMDragEventCoerce for nsIDOMDragEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMDragEvent) -> &Self {
        v
    }
}

impl nsIDOMDragEvent {
    #[inline]
    pub fn coerce<T: nsIDOMDragEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMDragEvent {
    type Target = nsIDOMMouseEvent;
    #[inline]
    fn deref(&self) -> &nsIDOMMouseEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMMouseEventCoerce> nsIDOMDragEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMDragEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMDragEventVTable {
    pub __base: nsIDOMMouseEventVTable,

    /* readonly attribute nsIDOMDataTransfer dataTransfer; */
    pub get_dataTransfer: unsafe extern "C" fn (this: *const nsIDOMDragEvent, aDataTransfer: *mut *const nsIDOMDataTransfer) -> nsresult,

}


impl nsIDOMDragEvent {
    /* readonly attribute nsIDOMDataTransfer dataTransfer; */
    #[inline]
    pub unsafe fn get_dataTransfer(&self, ) -> Result<Option<RefPtr<nsIDOMDataTransfer>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_dataTransfer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


