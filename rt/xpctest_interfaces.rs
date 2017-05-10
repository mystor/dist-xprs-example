//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpctest_interfaces.idl
//


#[repr(C)]
pub struct nsIXPCTestInterfaceA {
    vtable: *const nsIXPCTestInterfaceAVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCTestInterfaceA {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3c8fd2f5, 0x970c, 0x42c6,
            [0xb5, 0xdd, 0xcd, 0xa1, 0xc1, 0x6d, 0xcf, 0xd8])
    }
}

unsafe impl RefCounted for nsIXPCTestInterfaceA {
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
pub trait nsIXPCTestInterfaceACoerce {
    fn coerce_from(v: &nsIXPCTestInterfaceA) -> &Self;
}

impl nsIXPCTestInterfaceACoerce for nsIXPCTestInterfaceA {
    #[inline]
    fn coerce_from(v: &nsIXPCTestInterfaceA) -> &Self {
        v
    }
}

impl nsIXPCTestInterfaceA {
    #[inline]
    pub fn coerce<T: nsIXPCTestInterfaceACoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCTestInterfaceA {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCTestInterfaceACoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCTestInterfaceA) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCTestInterfaceAVTable {
    pub __base: nsISupportsVTable,

    /* attribute string name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIXPCTestInterfaceA, aName: *mut *const libc::c_char) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIXPCTestInterfaceA, aName: *const libc::c_char) -> nsresult,

}


impl nsIXPCTestInterfaceA {
    /* attribute string name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_name)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_name(&self, aName: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).set_name)(self as *const _, aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIXPCTestInterfaceB {
    vtable: *const nsIXPCTestInterfaceBVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCTestInterfaceB {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xff528c3a, 0x2410, 0x46de,
            [0xac, 0xaa, 0x44, 0x9a, 0xa6, 0x40, 0x3a, 0x33])
    }
}

unsafe impl RefCounted for nsIXPCTestInterfaceB {
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
pub trait nsIXPCTestInterfaceBCoerce {
    fn coerce_from(v: &nsIXPCTestInterfaceB) -> &Self;
}

impl nsIXPCTestInterfaceBCoerce for nsIXPCTestInterfaceB {
    #[inline]
    fn coerce_from(v: &nsIXPCTestInterfaceB) -> &Self {
        v
    }
}

impl nsIXPCTestInterfaceB {
    #[inline]
    pub fn coerce<T: nsIXPCTestInterfaceBCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCTestInterfaceB {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCTestInterfaceBCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCTestInterfaceB) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCTestInterfaceBVTable {
    pub __base: nsISupportsVTable,

    /* attribute string name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIXPCTestInterfaceB, aName: *mut *const libc::c_char) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIXPCTestInterfaceB, aName: *const libc::c_char) -> nsresult,

}


impl nsIXPCTestInterfaceB {
    /* attribute string name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_name)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_name(&self, aName: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).set_name)(self as *const _, aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIXPCTestInterfaceC {
    vtable: *const nsIXPCTestInterfaceCVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCTestInterfaceC {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x401cf1b4, 0x355b, 0x4cee,
            [0xb7, 0xb3, 0xc7, 0x97, 0x3a, 0xee, 0x49, 0xbd])
    }
}

unsafe impl RefCounted for nsIXPCTestInterfaceC {
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
pub trait nsIXPCTestInterfaceCCoerce {
    fn coerce_from(v: &nsIXPCTestInterfaceC) -> &Self;
}

impl nsIXPCTestInterfaceCCoerce for nsIXPCTestInterfaceC {
    #[inline]
    fn coerce_from(v: &nsIXPCTestInterfaceC) -> &Self {
        v
    }
}

impl nsIXPCTestInterfaceC {
    #[inline]
    pub fn coerce<T: nsIXPCTestInterfaceCCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCTestInterfaceC {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCTestInterfaceCCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCTestInterfaceC) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCTestInterfaceCVTable {
    pub __base: nsISupportsVTable,

    /* attribute long someInteger; */
    pub get_someInteger: unsafe extern "C" fn (this: *const nsIXPCTestInterfaceC, aSomeInteger: *mut libc::int32_t) -> nsresult,
    pub set_someInteger: unsafe extern "C" fn (this: *const nsIXPCTestInterfaceC, aSomeInteger: libc::int32_t) -> nsresult,

}


impl nsIXPCTestInterfaceC {
    /* attribute long someInteger; */
    #[inline]
    pub unsafe fn get_someInteger(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_someInteger)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_someInteger(&self, aSomeInteger: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_someInteger)(self as *const _, aSomeInteger) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


