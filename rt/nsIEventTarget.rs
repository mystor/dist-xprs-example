//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEventTarget.idl
//


pub mod nsIEventTarget_consts {
    pub const DISPATCH_NORMAL: i64 = 0;
    pub const DISPATCH_SYNC: i64 = 1;
    pub const DISPATCH_AT_END: i64 = 2;
}


#[repr(C)]
pub struct nsIEventTarget {
    vtable: *const nsIEventTargetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEventTarget {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x88145945, 0x3278, 0x424e,
            [0x9f, 0x37, 0xd8, 0x74, 0xcb, 0xdd, 0x9f, 0x6f])
    }
}

unsafe impl RefCounted for nsIEventTarget {
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
pub trait nsIEventTargetCoerce {
    fn coerce_from(v: &nsIEventTarget) -> &Self;
}

impl nsIEventTargetCoerce for nsIEventTarget {
    #[inline]
    fn coerce_from(v: &nsIEventTarget) -> &Self {
        v
    }
}

impl nsIEventTarget {
    #[inline]
    pub fn coerce<T: nsIEventTargetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEventTarget {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEventTargetCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEventTarget) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEventTargetVTable {
    pub __base: nsISupportsVTable,

    /* boolean isOnCurrentThread (); */
    pub isOnCurrentThread: unsafe extern "C" fn (this: *const nsIEventTarget, _retval: *mut bool) -> nsresult,

    /* [binaryname(Dispatch),noscript] void dispatchFromC (in alreadyAddRefed_nsIRunnable event, in unsigned long flags); */
    /// Unable to call function as its signature contains a non-rust type
    pub Dispatch: *const ::libc::c_void,

    /* [binaryname(DispatchFromScript)] void dispatch (in nsIRunnable event, in unsigned long flags); */
    pub DispatchFromScript: unsafe extern "C" fn (this: *const nsIEventTarget, event: *const nsIRunnable, flags: libc::uint32_t) -> nsresult,

    /* [noscript] void delayedDispatch (in alreadyAddRefed_nsIRunnable event, in unsigned long delay); */
    /// Unable to call function as its signature contains a non-rust type
    pub delayedDispatch: *const ::libc::c_void,

}


impl nsIEventTarget {
    /* boolean isOnCurrentThread (); */
    #[inline]
    pub unsafe fn isOnCurrentThread(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isOnCurrentThread)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [binaryname(Dispatch),noscript] void dispatchFromC (in alreadyAddRefed_nsIRunnable event, in unsigned long flags); */


    /* [binaryname(DispatchFromScript)] void dispatch (in nsIRunnable event, in unsigned long flags); */
    #[inline]
    pub unsafe fn DispatchFromScript(&self, event: Option<&nsIRunnable>, flags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).DispatchFromScript)(self as *const _, event.map_or(::std::ptr::null(), |x| x as *const _), flags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void delayedDispatch (in alreadyAddRefed_nsIRunnable event, in unsigned long delay); */


}


