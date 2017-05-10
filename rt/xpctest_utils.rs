//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpctest_utils.idl
//


#[repr(C)]
pub struct nsIXPCTestFunctionInterface {
    vtable: *const nsIXPCTestFunctionInterfaceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCTestFunctionInterface {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd58a82ab, 0xd8f7, 0x4ca9,
            [0x92, 0x73, 0xb3, 0x29, 0x0d, 0x42, 0xa0, 0xcf])
    }
}

unsafe impl RefCounted for nsIXPCTestFunctionInterface {
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
pub trait nsIXPCTestFunctionInterfaceCoerce {
    fn coerce_from(v: &nsIXPCTestFunctionInterface) -> &Self;
}

impl nsIXPCTestFunctionInterfaceCoerce for nsIXPCTestFunctionInterface {
    #[inline]
    fn coerce_from(v: &nsIXPCTestFunctionInterface) -> &Self {
        v
    }
}

impl nsIXPCTestFunctionInterface {
    #[inline]
    pub fn coerce<T: nsIXPCTestFunctionInterfaceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCTestFunctionInterface {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCTestFunctionInterfaceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCTestFunctionInterface) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCTestFunctionInterfaceVTable {
    pub __base: nsISupportsVTable,

    /* string echo (in string arg); */
    pub echo: unsafe extern "C" fn (this: *const nsIXPCTestFunctionInterface, arg: *const libc::c_char, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsIXPCTestFunctionInterface {
    /* string echo (in string arg); */
    #[inline]
    pub unsafe fn echo(&self, arg: *const libc::c_char) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).echo)(self as *const _, arg, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIXPCTestUtils {
    vtable: *const nsIXPCTestUtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCTestUtils {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1e9cddeb, 0x510d, 0x449a,
            [0xb1, 0x52, 0x3c, 0x1b, 0x5b, 0x31, 0xd4, 0x1d])
    }
}

unsafe impl RefCounted for nsIXPCTestUtils {
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
pub trait nsIXPCTestUtilsCoerce {
    fn coerce_from(v: &nsIXPCTestUtils) -> &Self;
}

impl nsIXPCTestUtilsCoerce for nsIXPCTestUtils {
    #[inline]
    fn coerce_from(v: &nsIXPCTestUtils) -> &Self {
        v
    }
}

impl nsIXPCTestUtils {
    #[inline]
    pub fn coerce<T: nsIXPCTestUtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCTestUtils {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCTestUtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCTestUtils) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCTestUtilsVTable {
    pub __base: nsISupportsVTable,

    /* nsIXPCTestFunctionInterface doubleWrapFunction (in nsIXPCTestFunctionInterface f); */
    pub doubleWrapFunction: unsafe extern "C" fn (this: *const nsIXPCTestUtils, f: *const nsIXPCTestFunctionInterface, _retval: *mut *const nsIXPCTestFunctionInterface) -> nsresult,

}


impl nsIXPCTestUtils {
    /* nsIXPCTestFunctionInterface doubleWrapFunction (in nsIXPCTestFunctionInterface f); */
    #[inline]
    pub unsafe fn doubleWrapFunction(&self, f: Option<&nsIXPCTestFunctionInterface>) -> Result<Option<RefPtr<nsIXPCTestFunctionInterface>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).doubleWrapFunction)(self as *const _, f.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


