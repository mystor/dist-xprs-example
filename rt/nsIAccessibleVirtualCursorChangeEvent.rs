//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleVirtualCursorChangeEvent.idl
//


#[repr(C)]
pub struct nsIAccessibleVirtualCursorChangeEvent {
    vtable: *const nsIAccessibleVirtualCursorChangeEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleVirtualCursorChangeEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa58693b1, 0x009e, 0x4cc9,
            [0xae, 0x93, 0x9c, 0x7d, 0x8f, 0x85, 0xcf, 0xdf])
    }
}

unsafe impl RefCounted for nsIAccessibleVirtualCursorChangeEvent {
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
pub trait nsIAccessibleVirtualCursorChangeEventCoerce {
    fn coerce_from(v: &nsIAccessibleVirtualCursorChangeEvent) -> &Self;
}

impl nsIAccessibleVirtualCursorChangeEventCoerce for nsIAccessibleVirtualCursorChangeEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleVirtualCursorChangeEvent) -> &Self {
        v
    }
}

impl nsIAccessibleVirtualCursorChangeEvent {
    #[inline]
    pub fn coerce<T: nsIAccessibleVirtualCursorChangeEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleVirtualCursorChangeEvent {
    type Target = nsIAccessibleEvent;
    #[inline]
    fn deref(&self) -> &nsIAccessibleEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIAccessibleEventCoerce> nsIAccessibleVirtualCursorChangeEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleVirtualCursorChangeEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleVirtualCursorChangeEventVTable {
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute nsIAccessible oldAccessible; */
    pub get_oldAccessible: unsafe extern "C" fn (this: *const nsIAccessibleVirtualCursorChangeEvent, aOldAccessible: *mut *const nsIAccessible) -> nsresult,

    /* readonly attribute long oldStartOffset; */
    pub get_oldStartOffset: unsafe extern "C" fn (this: *const nsIAccessibleVirtualCursorChangeEvent, aOldStartOffset: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long oldEndOffset; */
    pub get_oldEndOffset: unsafe extern "C" fn (this: *const nsIAccessibleVirtualCursorChangeEvent, aOldEndOffset: *mut libc::int32_t) -> nsresult,

    /* readonly attribute short reason; */
    pub get_reason: unsafe extern "C" fn (this: *const nsIAccessibleVirtualCursorChangeEvent, aReason: *mut libc::int16_t) -> nsresult,

}


impl nsIAccessibleVirtualCursorChangeEvent {
    /* readonly attribute nsIAccessible oldAccessible; */
    #[inline]
    pub unsafe fn get_oldAccessible(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_oldAccessible)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long oldStartOffset; */
    #[inline]
    pub unsafe fn get_oldStartOffset(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_oldStartOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long oldEndOffset; */
    #[inline]
    pub unsafe fn get_oldEndOffset(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_oldEndOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute short reason; */
    #[inline]
    pub unsafe fn get_reason(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_reason)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


