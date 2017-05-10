//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAutoConfig.idl
//


#[repr(C)]
pub struct nsIAutoConfig {
    vtable: *const nsIAutoConfigVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAutoConfig {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x80db54ae, 0x13f2, 0x11d5,
            [0xbe, 0x44, 0x00, 0x10, 0x83, 0x35, 0xa2, 0x20])
    }
}

unsafe impl RefCounted for nsIAutoConfig {
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
pub trait nsIAutoConfigCoerce {
    fn coerce_from(v: &nsIAutoConfig) -> &Self;
}

impl nsIAutoConfigCoerce for nsIAutoConfig {
    #[inline]
    fn coerce_from(v: &nsIAutoConfig) -> &Self {
        v
    }
}

impl nsIAutoConfig {
    #[inline]
    pub fn coerce<T: nsIAutoConfigCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAutoConfig {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAutoConfigCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoConfig) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAutoConfigVTable {
    pub __base: nsISupportsVTable,

    /* attribute string configURL; */
    pub get_configURL: unsafe extern "C" fn (this: *const nsIAutoConfig, aConfigURL: *mut *const libc::c_char) -> nsresult,
    pub set_configURL: unsafe extern "C" fn (this: *const nsIAutoConfig, aConfigURL: *const libc::c_char) -> nsresult,

}


impl nsIAutoConfig {
    /* attribute string configURL; */
    #[inline]
    pub unsafe fn get_configURL(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_configURL)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_configURL(&self, aConfigURL: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).set_configURL)(self as *const _, aConfigURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


