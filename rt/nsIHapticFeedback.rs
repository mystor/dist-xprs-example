//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHapticFeedback.idl
//


pub mod nsIHapticFeedback_consts {
    pub const ShortPress: i64 = 0;
    pub const LongPress: i64 = 1;
}


#[repr(C)]
pub struct nsIHapticFeedback {
    vtable: *const nsIHapticFeedbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHapticFeedback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x91917c98, 0xa8f3, 0x4c98,
            [0x8f, 0x10, 0x4a, 0xfb, 0x87, 0x2f, 0x54, 0xc7])
    }
}

unsafe impl RefCounted for nsIHapticFeedback {
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
pub trait nsIHapticFeedbackCoerce {
    fn coerce_from(v: &nsIHapticFeedback) -> &Self;
}

impl nsIHapticFeedbackCoerce for nsIHapticFeedback {
    #[inline]
    fn coerce_from(v: &nsIHapticFeedback) -> &Self {
        v
    }
}

impl nsIHapticFeedback {
    #[inline]
    pub fn coerce<T: nsIHapticFeedbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHapticFeedback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHapticFeedbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHapticFeedback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHapticFeedbackVTable {
    pub __base: nsISupportsVTable,

    /* void performSimpleAction (in long isLongPress); */
    pub performSimpleAction: unsafe extern "C" fn (this: *const nsIHapticFeedback, isLongPress: libc::int32_t) -> nsresult,

}


impl nsIHapticFeedback {
    /* void performSimpleAction (in long isLongPress); */
    #[inline]
    pub unsafe fn performSimpleAction(&self, isLongPress: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).performSimpleAction)(self as *const _, isLongPress) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


