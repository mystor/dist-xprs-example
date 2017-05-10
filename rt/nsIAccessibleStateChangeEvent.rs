//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleStateChangeEvent.idl
//


#[repr(C)]
pub struct nsIAccessibleStateChangeEvent {
    vtable: *const nsIAccessibleStateChangeEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleStateChangeEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x58b74954, 0x1835, 0x46ed,
            [0x9c, 0xcd, 0xc9, 0x06, 0x49, 0x01, 0x06, 0xf6])
    }
}

unsafe impl RefCounted for nsIAccessibleStateChangeEvent {
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
pub trait nsIAccessibleStateChangeEventCoerce {
    fn coerce_from(v: &nsIAccessibleStateChangeEvent) -> &Self;
}

impl nsIAccessibleStateChangeEventCoerce for nsIAccessibleStateChangeEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleStateChangeEvent) -> &Self {
        v
    }
}

impl nsIAccessibleStateChangeEvent {
    #[inline]
    pub fn coerce<T: nsIAccessibleStateChangeEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleStateChangeEvent {
    type Target = nsIAccessibleEvent;
    #[inline]
    fn deref(&self) -> &nsIAccessibleEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIAccessibleEventCoerce> nsIAccessibleStateChangeEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleStateChangeEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleStateChangeEventVTable {
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute unsigned long state; */
    pub get_state: unsafe extern "C" fn (this: *const nsIAccessibleStateChangeEvent, aState: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute boolean isExtraState; */
    pub get_isExtraState: unsafe extern "C" fn (this: *const nsIAccessibleStateChangeEvent, aIsExtraState: *mut bool) -> nsresult,

    /* readonly attribute boolean isEnabled; */
    pub get_isEnabled: unsafe extern "C" fn (this: *const nsIAccessibleStateChangeEvent, aIsEnabled: *mut bool) -> nsresult,

}


impl nsIAccessibleStateChangeEvent {
    /* readonly attribute unsigned long state; */
    #[inline]
    pub unsafe fn get_state(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_state)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isExtraState; */
    #[inline]
    pub unsafe fn get_isExtraState(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isExtraState)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isEnabled; */
    #[inline]
    pub unsafe fn get_isEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


