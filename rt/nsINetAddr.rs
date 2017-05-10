//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINetAddr.idl
//


pub mod nsINetAddr_consts {
    pub const FAMILY_INET: i64 = 1;
    pub const FAMILY_INET6: i64 = 2;
    pub const FAMILY_LOCAL: i64 = 3;
}


#[repr(C)]
pub struct nsINetAddr {
    vtable: *const nsINetAddrVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINetAddr {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x652b9ec5, 0xd159, 0x45d7,
            [0x91, 0x27, 0x50, 0xbb, 0x55, 0x94, 0x86, 0xcd])
    }
}

unsafe impl RefCounted for nsINetAddr {
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
pub trait nsINetAddrCoerce {
    fn coerce_from(v: &nsINetAddr) -> &Self;
}

impl nsINetAddrCoerce for nsINetAddr {
    #[inline]
    fn coerce_from(v: &nsINetAddr) -> &Self {
        v
    }
}

impl nsINetAddr {
    #[inline]
    pub fn coerce<T: nsINetAddrCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINetAddr {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINetAddrCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetAddr) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINetAddrVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short family; */
    pub get_family: unsafe extern "C" fn (this: *const nsINetAddr, aFamily: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute AUTF8String address; */
    pub get_address: unsafe extern "C" fn (this: *const nsINetAddr, aAddress: *mut nsACString) -> nsresult,

    /* readonly attribute unsigned short port; */
    pub get_port: unsafe extern "C" fn (this: *const nsINetAddr, aPort: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute unsigned long flow; */
    pub get_flow: unsafe extern "C" fn (this: *const nsINetAddr, aFlow: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long scope; */
    pub get_scope: unsafe extern "C" fn (this: *const nsINetAddr, aScope: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute boolean isV4Mapped; */
    pub get_isV4Mapped: unsafe extern "C" fn (this: *const nsINetAddr, aIsV4Mapped: *mut bool) -> nsresult,

    /* [noscript] NetAddr getNetAddr (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getNetAddr: *const ::libc::c_void,

}


impl nsINetAddr {
    /* readonly attribute unsigned short family; */
    #[inline]
    pub unsafe fn get_family(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_family)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String address; */
    #[inline]
    pub unsafe fn get_address(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_address)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned short port; */
    #[inline]
    pub unsafe fn get_port(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_port)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long flow; */
    #[inline]
    pub unsafe fn get_flow(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_flow)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long scope; */
    #[inline]
    pub unsafe fn get_scope(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_scope)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isV4Mapped; */
    #[inline]
    pub unsafe fn get_isV4Mapped(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isV4Mapped)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] NetAddr getNetAddr (); */


}


