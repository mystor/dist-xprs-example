//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpctest_bug809674.idl
//


#[repr(C)]
pub struct nsIXPCTestBug809674 {
    vtable: *const nsIXPCTestBug809674VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCTestBug809674 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2df46559, 0xda21, 0x49bf,
            [0xb8, 0x63, 0x0d, 0x7b, 0x7b, 0xbc, 0xbc, 0x73])
    }
}

unsafe impl RefCounted for nsIXPCTestBug809674 {
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
pub trait nsIXPCTestBug809674Coerce {
    fn coerce_from(v: &nsIXPCTestBug809674) -> &Self;
}

impl nsIXPCTestBug809674Coerce for nsIXPCTestBug809674 {
    #[inline]
    fn coerce_from(v: &nsIXPCTestBug809674) -> &Self {
        v
    }
}

impl nsIXPCTestBug809674 {
    #[inline]
    pub fn coerce<T: nsIXPCTestBug809674Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCTestBug809674 {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCTestBug809674Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCTestBug809674) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCTestBug809674VTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] attribute jsval jsvalProperty; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_jsvalProperty: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_jsvalProperty: *const ::libc::c_void,

}


impl nsIXPCTestBug809674 {
    /* [implicit_jscontext] attribute jsval jsvalProperty; */



}


