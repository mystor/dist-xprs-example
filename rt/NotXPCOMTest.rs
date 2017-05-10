//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/NotXPCOMTest.idl
//


#[repr(C)]
pub struct ScriptableOK {
    vtable: *const ScriptableOKVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for ScriptableOK {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x93142a4f, 0xe4cf, 0x424a,
            [0xb8, 0x33, 0xe6, 0x38, 0xf8, 0x7d, 0x26, 0x07])
    }
}

unsafe impl RefCounted for ScriptableOK {
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
pub trait ScriptableOKCoerce {
    fn coerce_from(v: &ScriptableOK) -> &Self;
}

impl ScriptableOKCoerce for ScriptableOK {
    #[inline]
    fn coerce_from(v: &ScriptableOK) -> &Self {
        v
    }
}

impl ScriptableOK {
    #[inline]
    pub fn coerce<T: ScriptableOKCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for ScriptableOK {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> ScriptableOKCoerce for T {
    #[inline]
    fn coerce_from(v: &ScriptableOK) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct ScriptableOKVTable {
    pub __base: nsISupportsVTable,

    /* void method1 (); */
    pub method1: unsafe extern "C" fn (this: *const ScriptableOK) -> nsresult,

}


impl ScriptableOK {
    /* void method1 (); */
    #[inline]
    pub unsafe fn method1(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).method1)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct ScriptableWithNotXPCOM {
    vtable: *const ScriptableWithNotXPCOMVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for ScriptableWithNotXPCOM {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x237d01a3, 0x771e, 0x4c6e,
            [0xad, 0xf9, 0xc9, 0x7f, 0x9a, 0xab, 0x29, 0x50])
    }
}

unsafe impl RefCounted for ScriptableWithNotXPCOM {
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
pub trait ScriptableWithNotXPCOMCoerce {
    fn coerce_from(v: &ScriptableWithNotXPCOM) -> &Self;
}

impl ScriptableWithNotXPCOMCoerce for ScriptableWithNotXPCOM {
    #[inline]
    fn coerce_from(v: &ScriptableWithNotXPCOM) -> &Self {
        v
    }
}

impl ScriptableWithNotXPCOM {
    #[inline]
    pub fn coerce<T: ScriptableWithNotXPCOMCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for ScriptableWithNotXPCOM {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> ScriptableWithNotXPCOMCoerce for T {
    #[inline]
    fn coerce_from(v: &ScriptableWithNotXPCOM) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct ScriptableWithNotXPCOMVTable {
    pub __base: nsISupportsVTable,

    /* [notxpcom] void method2 (); */
    pub method2: unsafe extern "C" fn (this: *const ScriptableWithNotXPCOM) -> libc::c_void,

}


impl ScriptableWithNotXPCOM {
    /* [notxpcom] void method2 (); */
    #[inline]
    pub unsafe fn method2(&self, ) -> () {

        let _retval = ((*self.vtable).method2)(self as *const _, );
        ()
    }

}


#[repr(C)]
pub struct ScriptableWithNotXPCOMBase {
    vtable: *const ScriptableWithNotXPCOMBaseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for ScriptableWithNotXPCOMBase {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4f06ec60, 0x3bb3, 0x4712,
            [0xab, 0x18, 0xb2, 0xb5, 0x95, 0x28, 0x55, 0x58])
    }
}

unsafe impl RefCounted for ScriptableWithNotXPCOMBase {
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
pub trait ScriptableWithNotXPCOMBaseCoerce {
    fn coerce_from(v: &ScriptableWithNotXPCOMBase) -> &Self;
}

impl ScriptableWithNotXPCOMBaseCoerce for ScriptableWithNotXPCOMBase {
    #[inline]
    fn coerce_from(v: &ScriptableWithNotXPCOMBase) -> &Self {
        v
    }
}

impl ScriptableWithNotXPCOMBase {
    #[inline]
    pub fn coerce<T: ScriptableWithNotXPCOMBaseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for ScriptableWithNotXPCOMBase {
    type Target = ScriptableWithNotXPCOM;
    #[inline]
    fn deref(&self) -> &ScriptableWithNotXPCOM {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: ScriptableWithNotXPCOMCoerce> ScriptableWithNotXPCOMBaseCoerce for T {
    #[inline]
    fn coerce_from(v: &ScriptableWithNotXPCOMBase) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct ScriptableWithNotXPCOMBaseVTable {
    pub __base: ScriptableWithNotXPCOMVTable,

    /* void method3 (); */
    pub method3: unsafe extern "C" fn (this: *const ScriptableWithNotXPCOMBase) -> nsresult,

}


impl ScriptableWithNotXPCOMBase {
    /* void method3 (); */
    #[inline]
    pub unsafe fn method3(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).method3)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


