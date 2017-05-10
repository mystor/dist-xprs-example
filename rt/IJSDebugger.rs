//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/IJSDebugger.idl
//


#[repr(C)]
pub struct IJSDebugger {
    vtable: *const IJSDebuggerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for IJSDebugger {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa36fa816, 0x31da, 0x4b23,
            [0xbc, 0x97, 0x64, 0x12, 0x77, 0x1f, 0x08, 0x67])
    }
}

unsafe impl RefCounted for IJSDebugger {
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
pub trait IJSDebuggerCoerce {
    fn coerce_from(v: &IJSDebugger) -> &Self;
}

impl IJSDebuggerCoerce for IJSDebugger {
    #[inline]
    fn coerce_from(v: &IJSDebugger) -> &Self {
        v
    }
}

impl IJSDebugger {
    #[inline]
    pub fn coerce<T: IJSDebuggerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for IJSDebugger {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> IJSDebuggerCoerce for T {
    #[inline]
    fn coerce_from(v: &IJSDebugger) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct IJSDebuggerVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void addClass (in jsval global); */
    /// Unable to call function as its signature contains a non-rust type
    pub addClass: *const ::libc::c_void,

}


impl IJSDebugger {
    /* [implicit_jscontext] void addClass (in jsval global); */


}


