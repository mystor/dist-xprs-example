//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDisplayInfo.idl
//


#[repr(C)]
pub struct nsIDisplayInfo {
    vtable: *const nsIDisplayInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDisplayInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x615bc23d, 0x6346, 0x4b15,
            [0x9c, 0x10, 0xad, 0xd0, 0x02, 0xf1, 0x40, 0xb6])
    }
}

unsafe impl RefCounted for nsIDisplayInfo {
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
pub trait nsIDisplayInfoCoerce {
    fn coerce_from(v: &nsIDisplayInfo) -> &Self;
}

impl nsIDisplayInfoCoerce for nsIDisplayInfo {
    #[inline]
    fn coerce_from(v: &nsIDisplayInfo) -> &Self {
        v
    }
}

impl nsIDisplayInfo {
    #[inline]
    pub fn coerce<T: nsIDisplayInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDisplayInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDisplayInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDisplayInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDisplayInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long id; */
    pub get_id: unsafe extern "C" fn (this: *const nsIDisplayInfo, aId: *mut libc::int32_t) -> nsresult,

    /* readonly attribute boolean connected; */
    pub get_connected: unsafe extern "C" fn (this: *const nsIDisplayInfo, aConnected: *mut bool) -> nsresult,

}


impl nsIDisplayInfo {
    /* readonly attribute long id; */
    #[inline]
    pub unsafe fn get_id(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_id)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean connected; */
    #[inline]
    pub unsafe fn get_connected(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_connected)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


