//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleTableChangeEvent.idl
//


#[repr(C)]
pub struct nsIAccessibleTableChangeEvent {
    vtable: *const nsIAccessibleTableChangeEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleTableChangeEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9fb3a8a4, 0xd254, 0x43d3,
            [0x80, 0xa5, 0x20, 0xe1, 0x71, 0xd5, 0x2b, 0x21])
    }
}

unsafe impl RefCounted for nsIAccessibleTableChangeEvent {
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
pub trait nsIAccessibleTableChangeEventCoerce {
    fn coerce_from(v: &nsIAccessibleTableChangeEvent) -> &Self;
}

impl nsIAccessibleTableChangeEventCoerce for nsIAccessibleTableChangeEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTableChangeEvent) -> &Self {
        v
    }
}

impl nsIAccessibleTableChangeEvent {
    #[inline]
    pub fn coerce<T: nsIAccessibleTableChangeEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleTableChangeEvent {
    type Target = nsIAccessibleEvent;
    #[inline]
    fn deref(&self) -> &nsIAccessibleEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIAccessibleEventCoerce> nsIAccessibleTableChangeEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTableChangeEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleTableChangeEventVTable {
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute long rowOrColIndex; */
    pub get_rowOrColIndex: unsafe extern "C" fn (this: *const nsIAccessibleTableChangeEvent, aRowOrColIndex: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long RowsOrColsCount; */
    pub get_RowsOrColsCount: unsafe extern "C" fn (this: *const nsIAccessibleTableChangeEvent, aRowsOrColsCount: *mut libc::int32_t) -> nsresult,

}


impl nsIAccessibleTableChangeEvent {
    /* readonly attribute long rowOrColIndex; */
    #[inline]
    pub unsafe fn get_rowOrColIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_rowOrColIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long RowsOrColsCount; */
    #[inline]
    pub unsafe fn get_RowsOrColsCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_RowsOrColsCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


