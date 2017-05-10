//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgINotificationObserver.idl
//


pub mod imgINotificationObserver_consts {
    pub const SIZE_AVAILABLE: i64 = 1;
    pub const FRAME_UPDATE: i64 = 2;
    pub const FRAME_COMPLETE: i64 = 3;
    pub const LOAD_COMPLETE: i64 = 4;
    pub const DECODE_COMPLETE: i64 = 5;
    pub const DISCARD: i64 = 6;
    pub const UNLOCKED_DRAW: i64 = 7;
    pub const IS_ANIMATED: i64 = 8;
    pub const HAS_TRANSPARENCY: i64 = 9;
}


#[repr(C)]
pub struct imgINotificationObserver {
    vtable: *const imgINotificationObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for imgINotificationObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x03da5641, 0xa333, 0x454a,
            [0xa8, 0x59, 0x03, 0x6d, 0x0b, 0xb6, 0x83, 0xb7])
    }
}

unsafe impl RefCounted for imgINotificationObserver {
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
pub trait imgINotificationObserverCoerce {
    fn coerce_from(v: &imgINotificationObserver) -> &Self;
}

impl imgINotificationObserverCoerce for imgINotificationObserver {
    #[inline]
    fn coerce_from(v: &imgINotificationObserver) -> &Self {
        v
    }
}

impl imgINotificationObserver {
    #[inline]
    pub fn coerce<T: imgINotificationObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for imgINotificationObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> imgINotificationObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &imgINotificationObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct imgINotificationObserverVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] void notify (in imgIRequest aProxy, in long aType, [const] in nsIntRect aRect); */
    /// Unable to call function as its signature contains a non-rust type
    pub notify: *const ::libc::c_void,

}


impl imgINotificationObserver {
    /* [noscript] void notify (in imgIRequest aProxy, in long aType, [const] in nsIntRect aRect); */


}


