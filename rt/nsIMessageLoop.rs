//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMessageLoop.idl
//


#[repr(C)]
pub struct nsIMessageLoop {
    vtable: *const nsIMessageLoopVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMessageLoop {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3e8c58e8, 0xe52c, 0x43e0,
            [0x8e, 0x66, 0x66, 0x9c, 0xa7, 0x88, 0xff, 0x5f])
    }
}

unsafe impl RefCounted for nsIMessageLoop {
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
pub trait nsIMessageLoopCoerce {
    fn coerce_from(v: &nsIMessageLoop) -> &Self;
}

impl nsIMessageLoopCoerce for nsIMessageLoop {
    #[inline]
    fn coerce_from(v: &nsIMessageLoop) -> &Self {
        v
    }
}

impl nsIMessageLoop {
    #[inline]
    pub fn coerce<T: nsIMessageLoopCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMessageLoop {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMessageLoopCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMessageLoop) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMessageLoopVTable {
    pub __base: nsISupportsVTable,

    /* void postIdleTask (in nsIRunnable task, in uint32_t ensureRunsAfterMS); */
    pub postIdleTask: unsafe extern "C" fn (this: *const nsIMessageLoop, task: *const nsIRunnable, ensureRunsAfterMS: uint32_t) -> nsresult,

}


impl nsIMessageLoop {
    /* void postIdleTask (in nsIRunnable task, in uint32_t ensureRunsAfterMS); */
    #[inline]
    pub unsafe fn postIdleTask(&self, task: Option<&nsIRunnable>, ensureRunsAfterMS: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).postIdleTask)(self as *const _, task.map_or(::std::ptr::null(), |x| x as *const _), ensureRunsAfterMS) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


