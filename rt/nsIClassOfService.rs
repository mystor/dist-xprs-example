//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClassOfService.idl
//


pub mod nsIClassOfService_consts {
    pub const Leader: i64 = 1;
    pub const Follower: i64 = 2;
    pub const Speculative: i64 = 4;
    pub const Background: i64 = 8;
    pub const Unblocked: i64 = 16;
    pub const Throttleable: i64 = 32;
    pub const UrgentStart: i64 = 64;
}


#[repr(C)]
pub struct nsIClassOfService {
    vtable: *const nsIClassOfServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIClassOfService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1ccb58ec, 0x5e07, 0x4cf9,
            [0xa3, 0x0d, 0xac, 0x54, 0x90, 0xd2, 0x3b, 0x41])
    }
}

unsafe impl RefCounted for nsIClassOfService {
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
pub trait nsIClassOfServiceCoerce {
    fn coerce_from(v: &nsIClassOfService) -> &Self;
}

impl nsIClassOfServiceCoerce for nsIClassOfService {
    #[inline]
    fn coerce_from(v: &nsIClassOfService) -> &Self {
        v
    }
}

impl nsIClassOfService {
    #[inline]
    pub fn coerce<T: nsIClassOfServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIClassOfService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIClassOfServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClassOfService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIClassOfServiceVTable {
    pub __base: nsISupportsVTable,

    /* attribute unsigned long classFlags; */
    pub get_classFlags: unsafe extern "C" fn (this: *const nsIClassOfService, aClassFlags: *mut libc::uint32_t) -> nsresult,
    pub set_classFlags: unsafe extern "C" fn (this: *const nsIClassOfService, aClassFlags: libc::uint32_t) -> nsresult,

    /* void clearClassFlags (in unsigned long flags); */
    pub clearClassFlags: unsafe extern "C" fn (this: *const nsIClassOfService, flags: libc::uint32_t) -> nsresult,

    /* void addClassFlags (in unsigned long flags); */
    pub addClassFlags: unsafe extern "C" fn (this: *const nsIClassOfService, flags: libc::uint32_t) -> nsresult,

}


impl nsIClassOfService {
    /* attribute unsigned long classFlags; */
    #[inline]
    pub unsafe fn get_classFlags(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_classFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_classFlags(&self, aClassFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_classFlags)(self as *const _, aClassFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clearClassFlags (in unsigned long flags); */
    #[inline]
    pub unsafe fn clearClassFlags(&self, flags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).clearClassFlags)(self as *const _, flags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addClassFlags (in unsigned long flags); */
    #[inline]
    pub unsafe fn addClassFlags(&self, flags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).addClassFlags)(self as *const _, flags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


