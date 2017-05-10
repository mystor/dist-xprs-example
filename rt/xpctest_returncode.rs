//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpctest_returncode.idl
//


#[repr(C)]
pub struct nsIXPCTestReturnCodeParent {
    vtable: *const nsIXPCTestReturnCodeParentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCTestReturnCodeParent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x479e4532, 0x95cf, 0x48b8,
            [0xa9, 0x9b, 0x8a, 0x58, 0x81, 0xe4, 0x71, 0x38])
    }
}

unsafe impl RefCounted for nsIXPCTestReturnCodeParent {
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
pub trait nsIXPCTestReturnCodeParentCoerce {
    fn coerce_from(v: &nsIXPCTestReturnCodeParent) -> &Self;
}

impl nsIXPCTestReturnCodeParentCoerce for nsIXPCTestReturnCodeParent {
    #[inline]
    fn coerce_from(v: &nsIXPCTestReturnCodeParent) -> &Self {
        v
    }
}

impl nsIXPCTestReturnCodeParent {
    #[inline]
    pub fn coerce<T: nsIXPCTestReturnCodeParentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCTestReturnCodeParent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCTestReturnCodeParentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCTestReturnCodeParent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCTestReturnCodeParentVTable {
    pub __base: nsISupportsVTable,

    /* nsresult callChild (in long childBehavior); */
    pub callChild: unsafe extern "C" fn (this: *const nsIXPCTestReturnCodeParent, childBehavior: libc::int32_t, _retval: *mut nsresult) -> nsresult,

}


impl nsIXPCTestReturnCodeParent {
    /* nsresult callChild (in long childBehavior); */
    #[inline]
    pub unsafe fn callChild(&self, childBehavior: libc::int32_t) -> Result<nsresult, nsresult> {
        let mut _retval: nsresult = ::std::mem::zeroed();
        match ((*self.vtable).callChild)(self as *const _, childBehavior, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


pub mod nsIXPCTestReturnCodeChild_consts {
    pub const CHILD_SHOULD_THROW: i64 = 0;
    pub const CHILD_SHOULD_RETURN_SUCCESS: i64 = 1;
    pub const CHILD_SHOULD_RETURN_RESULTCODE: i64 = 2;
    pub const CHILD_SHOULD_NEST_RESULTCODES: i64 = 3;
}


#[repr(C)]
pub struct nsIXPCTestReturnCodeChild {
    vtable: *const nsIXPCTestReturnCodeChildVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCTestReturnCodeChild {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x672cfd34, 0x1fd1, 0x455d,
            [0x99, 0x01, 0xd8, 0x79, 0xfa, 0x6f, 0xdb, 0x95])
    }
}

unsafe impl RefCounted for nsIXPCTestReturnCodeChild {
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
pub trait nsIXPCTestReturnCodeChildCoerce {
    fn coerce_from(v: &nsIXPCTestReturnCodeChild) -> &Self;
}

impl nsIXPCTestReturnCodeChildCoerce for nsIXPCTestReturnCodeChild {
    #[inline]
    fn coerce_from(v: &nsIXPCTestReturnCodeChild) -> &Self {
        v
    }
}

impl nsIXPCTestReturnCodeChild {
    #[inline]
    pub fn coerce<T: nsIXPCTestReturnCodeChildCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCTestReturnCodeChild {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCTestReturnCodeChildCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCTestReturnCodeChild) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCTestReturnCodeChildVTable {
    pub __base: nsISupportsVTable,

    /* void doIt (in long behavior); */
    pub doIt: unsafe extern "C" fn (this: *const nsIXPCTestReturnCodeChild, behavior: libc::int32_t) -> nsresult,

}


impl nsIXPCTestReturnCodeChild {
    /* void doIt (in long behavior); */
    #[inline]
    pub unsafe fn doIt(&self, behavior: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).doIt)(self as *const _, behavior) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


