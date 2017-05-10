//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITaskbarProgress.idl
//


pub type nsTaskbarProgressState = libc::int32_t;


pub mod nsITaskbarProgress_consts {
    pub const STATE_NO_PROGRESS: i64 = 0;
    pub const STATE_INDETERMINATE: i64 = 1;
    pub const STATE_NORMAL: i64 = 2;
    pub const STATE_ERROR: i64 = 3;
    pub const STATE_PAUSED: i64 = 4;
}


#[repr(C)]
pub struct nsITaskbarProgress {
    vtable: *const nsITaskbarProgressVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITaskbarProgress {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x23ac257d, 0xef3c, 0x4033,
            [0xb4, 0x24, 0xbe, 0x7f, 0xef, 0x91, 0xa8, 0x6c])
    }
}

unsafe impl RefCounted for nsITaskbarProgress {
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
pub trait nsITaskbarProgressCoerce {
    fn coerce_from(v: &nsITaskbarProgress) -> &Self;
}

impl nsITaskbarProgressCoerce for nsITaskbarProgress {
    #[inline]
    fn coerce_from(v: &nsITaskbarProgress) -> &Self {
        v
    }
}

impl nsITaskbarProgress {
    #[inline]
    pub fn coerce<T: nsITaskbarProgressCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITaskbarProgress {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITaskbarProgressCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITaskbarProgress) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITaskbarProgressVTable {
    pub __base: nsISupportsVTable,

    /* void setProgressState (in nsTaskbarProgressState state, [optional] in unsigned long long currentValue, [optional] in unsigned long long maxValue); */
    pub setProgressState: unsafe extern "C" fn (this: *const nsITaskbarProgress, state: nsTaskbarProgressState, currentValue: libc::uint64_t, maxValue: libc::uint64_t) -> nsresult,

}


impl nsITaskbarProgress {
    /* void setProgressState (in nsTaskbarProgressState state, [optional] in unsigned long long currentValue, [optional] in unsigned long long maxValue); */
    #[inline]
    pub unsafe fn setProgressState(&self, state: nsTaskbarProgressState, currentValue: libc::uint64_t, maxValue: libc::uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).setProgressState)(self as *const _, state, currentValue, maxValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


