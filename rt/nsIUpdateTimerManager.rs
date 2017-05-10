//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUpdateTimerManager.idl
//


#[repr(C)]
pub struct nsIUpdateTimerManager {
    vtable: *const nsIUpdateTimerManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUpdateTimerManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0765c92c, 0x6145, 0x4253,
            [0x9d, 0xb4, 0x59, 0x4d, 0x80, 0x23, 0x08, 0x7e])
    }
}

unsafe impl RefCounted for nsIUpdateTimerManager {
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
pub trait nsIUpdateTimerManagerCoerce {
    fn coerce_from(v: &nsIUpdateTimerManager) -> &Self;
}

impl nsIUpdateTimerManagerCoerce for nsIUpdateTimerManager {
    #[inline]
    fn coerce_from(v: &nsIUpdateTimerManager) -> &Self {
        v
    }
}

impl nsIUpdateTimerManager {
    #[inline]
    pub fn coerce<T: nsIUpdateTimerManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUpdateTimerManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUpdateTimerManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdateTimerManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUpdateTimerManagerVTable {
    pub __base: nsISupportsVTable,

    /* void registerTimer (in AString id, in nsITimerCallback callback, in unsigned long interval); */
    pub registerTimer: unsafe extern "C" fn (this: *const nsIUpdateTimerManager, id: *const nsAString, callback: *const nsITimerCallback, interval: libc::uint32_t) -> nsresult,

    /* void unregisterTimer (in AString id); */
    pub unregisterTimer: unsafe extern "C" fn (this: *const nsIUpdateTimerManager, id: *const nsAString) -> nsresult,

}


impl nsIUpdateTimerManager {
    /* void registerTimer (in AString id, in nsITimerCallback callback, in unsigned long interval); */
    #[inline]
    pub unsafe fn registerTimer(&self, id: &[u16], callback: Option<&nsITimerCallback>, interval: libc::uint32_t) -> Result<(), nsresult> {
        let id = nsString::from(id);
        match ((*self.vtable).registerTimer)(self as *const _, &*id, callback.map_or(::std::ptr::null(), |x| x as *const _), interval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterTimer (in AString id); */
    #[inline]
    pub unsafe fn unregisterTimer(&self, id: &[u16]) -> Result<(), nsresult> {
        let id = nsString::from(id);
        match ((*self.vtable).unregisterTimer)(self as *const _, &*id) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


