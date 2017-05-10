//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleTextChangeEvent.idl
//


#[repr(C)]
pub struct nsIAccessibleTextChangeEvent {
    vtable: *const nsIAccessibleTextChangeEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleTextChangeEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1fcc0dfa, 0x93e6, 0x48f4,
            [0xbb, 0xd4, 0xf8, 0x0e, 0xb1, 0xd9, 0xf2, 0xe6])
    }
}

unsafe impl RefCounted for nsIAccessibleTextChangeEvent {
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
pub trait nsIAccessibleTextChangeEventCoerce {
    fn coerce_from(v: &nsIAccessibleTextChangeEvent) -> &Self;
}

impl nsIAccessibleTextChangeEventCoerce for nsIAccessibleTextChangeEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTextChangeEvent) -> &Self {
        v
    }
}

impl nsIAccessibleTextChangeEvent {
    #[inline]
    pub fn coerce<T: nsIAccessibleTextChangeEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleTextChangeEvent {
    type Target = nsIAccessibleEvent;
    #[inline]
    fn deref(&self) -> &nsIAccessibleEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIAccessibleEventCoerce> nsIAccessibleTextChangeEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTextChangeEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleTextChangeEventVTable {
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute long start; */
    pub get_start: unsafe extern "C" fn (this: *const nsIAccessibleTextChangeEvent, aStart: *mut libc::int32_t) -> nsresult,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIAccessibleTextChangeEvent, aLength: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute boolean isInserted; */
    pub get_isInserted: unsafe extern "C" fn (this: *const nsIAccessibleTextChangeEvent, aIsInserted: *mut bool) -> nsresult,

    /* readonly attribute DOMString modifiedText; */
    pub get_modifiedText: unsafe extern "C" fn (this: *const nsIAccessibleTextChangeEvent, aModifiedText: *mut nsAString) -> nsresult,

}


impl nsIAccessibleTextChangeEvent {
    /* readonly attribute long start; */
    #[inline]
    pub unsafe fn get_start(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_start)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isInserted; */
    #[inline]
    pub unsafe fn get_isInserted(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isInserted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString modifiedText; */
    #[inline]
    pub unsafe fn get_modifiedText(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_modifiedText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


