//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISocketFilter.idl
//


pub mod nsISocketFilter_consts {
    pub const SF_INCOMING: i64 = 0;
    pub const SF_OUTGOING: i64 = 1;
}


#[repr(C)]
pub struct nsISocketFilter {
    vtable: *const nsISocketFilterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISocketFilter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xafe2c40c, 0xb9b9, 0x4207,
            [0xb8, 0x98, 0xe5, 0xcd, 0xe1, 0x8c, 0x61, 0x39])
    }
}

unsafe impl RefCounted for nsISocketFilter {
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
pub trait nsISocketFilterCoerce {
    fn coerce_from(v: &nsISocketFilter) -> &Self;
}

impl nsISocketFilterCoerce for nsISocketFilter {
    #[inline]
    fn coerce_from(v: &nsISocketFilter) -> &Self {
        v
    }
}

impl nsISocketFilter {
    #[inline]
    pub fn coerce<T: nsISocketFilterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISocketFilter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISocketFilterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISocketFilter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISocketFilterVTable {
    pub __base: nsISupportsVTable,

    /* bool filterPacket ([const] in NetAddrPtr remote_addr, [array, size_is (len), const] in uint8_t data, in unsigned long len, in long direction); */
    /// Unable to call function as its signature contains a non-rust type
    pub filterPacket: *const ::libc::c_void,

}


impl nsISocketFilter {
    /* bool filterPacket ([const] in NetAddrPtr remote_addr, [array, size_is (len), const] in uint8_t data, in unsigned long len, in long direction); */


}


#[repr(C)]
pub struct nsISocketFilterHandler {
    vtable: *const nsISocketFilterHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISocketFilterHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x81ee76c6, 0x4753, 0x4125,
            [0x9c, 0x8c, 0x29, 0x0e, 0xd9, 0xba, 0x62, 0xfb])
    }
}

unsafe impl RefCounted for nsISocketFilterHandler {
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
pub trait nsISocketFilterHandlerCoerce {
    fn coerce_from(v: &nsISocketFilterHandler) -> &Self;
}

impl nsISocketFilterHandlerCoerce for nsISocketFilterHandler {
    #[inline]
    fn coerce_from(v: &nsISocketFilterHandler) -> &Self {
        v
    }
}

impl nsISocketFilterHandler {
    #[inline]
    pub fn coerce<T: nsISocketFilterHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISocketFilterHandler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISocketFilterHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISocketFilterHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISocketFilterHandlerVTable {
    pub __base: nsISupportsVTable,

    /* nsISocketFilter newFilter (); */
    pub newFilter: unsafe extern "C" fn (this: *const nsISocketFilterHandler, _retval: *mut *const nsISocketFilter) -> nsresult,

}


impl nsISocketFilterHandler {
    /* nsISocketFilter newFilter (); */
    #[inline]
    pub unsafe fn newFilter(&self, ) -> Result<Option<RefPtr<nsISocketFilter>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newFilter)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


