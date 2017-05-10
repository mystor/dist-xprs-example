//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleHideEvent.idl
//


#[repr(C)]
pub struct nsIAccessibleHideEvent {
    vtable: *const nsIAccessibleHideEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleHideEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2051709a, 0x4e0d, 0x4be5,
            [0x87, 0x3d, 0xb4, 0x9d, 0x1d, 0xee, 0x35, 0xfa])
    }
}

unsafe impl RefCounted for nsIAccessibleHideEvent {
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
pub trait nsIAccessibleHideEventCoerce {
    fn coerce_from(v: &nsIAccessibleHideEvent) -> &Self;
}

impl nsIAccessibleHideEventCoerce for nsIAccessibleHideEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleHideEvent) -> &Self {
        v
    }
}

impl nsIAccessibleHideEvent {
    #[inline]
    pub fn coerce<T: nsIAccessibleHideEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleHideEvent {
    type Target = nsIAccessibleEvent;
    #[inline]
    fn deref(&self) -> &nsIAccessibleEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIAccessibleEventCoerce> nsIAccessibleHideEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleHideEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleHideEventVTable {
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute nsIAccessible targetParent; */
    pub get_targetParent: unsafe extern "C" fn (this: *const nsIAccessibleHideEvent, aTargetParent: *mut *const nsIAccessible) -> nsresult,

    /* readonly attribute nsIAccessible targetNextSibling; */
    pub get_targetNextSibling: unsafe extern "C" fn (this: *const nsIAccessibleHideEvent, aTargetNextSibling: *mut *const nsIAccessible) -> nsresult,

    /* readonly attribute nsIAccessible targetPrevSibling; */
    pub get_targetPrevSibling: unsafe extern "C" fn (this: *const nsIAccessibleHideEvent, aTargetPrevSibling: *mut *const nsIAccessible) -> nsresult,

}


impl nsIAccessibleHideEvent {
    /* readonly attribute nsIAccessible targetParent; */
    #[inline]
    pub unsafe fn get_targetParent(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_targetParent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAccessible targetNextSibling; */
    #[inline]
    pub unsafe fn get_targetNextSibling(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_targetNextSibling)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAccessible targetPrevSibling; */
    #[inline]
    pub unsafe fn get_targetPrevSibling(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_targetPrevSibling)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


