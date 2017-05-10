//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProfileUnlocker.idl
//


pub mod nsIProfileUnlocker_consts {
    pub const ATTEMPT_QUIT: i64 = 0;
    pub const FORCE_QUIT: i64 = 1;
}


#[repr(C)]
pub struct nsIProfileUnlocker {
    vtable: *const nsIProfileUnlockerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProfileUnlocker {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x08923af1, 0xe7a3, 0x4fae,
            [0xba, 0x02, 0x12, 0x85, 0x02, 0x19, 0x39, 0x94])
    }
}

unsafe impl RefCounted for nsIProfileUnlocker {
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
pub trait nsIProfileUnlockerCoerce {
    fn coerce_from(v: &nsIProfileUnlocker) -> &Self;
}

impl nsIProfileUnlockerCoerce for nsIProfileUnlocker {
    #[inline]
    fn coerce_from(v: &nsIProfileUnlocker) -> &Self {
        v
    }
}

impl nsIProfileUnlocker {
    #[inline]
    pub fn coerce<T: nsIProfileUnlockerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProfileUnlocker {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProfileUnlockerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProfileUnlocker) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProfileUnlockerVTable {
    pub __base: nsISupportsVTable,

    /* void unlock (in unsigned long aSeverity); */
    pub unlock: unsafe extern "C" fn (this: *const nsIProfileUnlocker, aSeverity: libc::uint32_t) -> nsresult,

}


impl nsIProfileUnlocker {
    /* void unlock (in unsigned long aSeverity); */
    #[inline]
    pub unsafe fn unlock(&self, aSeverity: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).unlock)(self as *const _, aSeverity) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


