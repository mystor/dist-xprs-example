//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleCaretMoveEvent.idl
//


#[repr(C)]
pub struct nsIAccessibleCaretMoveEvent {
    vtable: *const nsIAccessibleCaretMoveEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleCaretMoveEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xed1982e4, 0x57d7, 0x41a8,
            [0x8c, 0xd8, 0x90, 0x23, 0xf8, 0x09, 0x38, 0x3e])
    }
}

unsafe impl RefCounted for nsIAccessibleCaretMoveEvent {
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
pub trait nsIAccessibleCaretMoveEventCoerce {
    fn coerce_from(v: &nsIAccessibleCaretMoveEvent) -> &Self;
}

impl nsIAccessibleCaretMoveEventCoerce for nsIAccessibleCaretMoveEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleCaretMoveEvent) -> &Self {
        v
    }
}

impl nsIAccessibleCaretMoveEvent {
    #[inline]
    pub fn coerce<T: nsIAccessibleCaretMoveEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleCaretMoveEvent {
    type Target = nsIAccessibleEvent;
    #[inline]
    fn deref(&self) -> &nsIAccessibleEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIAccessibleEventCoerce> nsIAccessibleCaretMoveEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleCaretMoveEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleCaretMoveEventVTable {
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute long caretOffset; */
    pub get_caretOffset: unsafe extern "C" fn (this: *const nsIAccessibleCaretMoveEvent, aCaretOffset: *mut libc::int32_t) -> nsresult,

}


impl nsIAccessibleCaretMoveEvent {
    /* readonly attribute long caretOffset; */
    #[inline]
    pub unsafe fn get_caretOffset(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_caretOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


