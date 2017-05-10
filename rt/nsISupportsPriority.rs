//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISupportsPriority.idl
//


pub mod nsISupportsPriority_consts {
    pub const PRIORITY_HIGHEST: i64 = -20;
    pub const PRIORITY_HIGH: i64 = -10;
    pub const PRIORITY_NORMAL: i64 = 0;
    pub const PRIORITY_LOW: i64 = 10;
    pub const PRIORITY_LOWEST: i64 = 20;
}


#[repr(C)]
pub struct nsISupportsPriority {
    vtable: *const nsISupportsPriorityVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsPriority {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xaa578b44, 0xabd5, 0x4c19,
            [0x8b, 0x14, 0x36, 0xd4, 0xde, 0x6f, 0xdc, 0x36])
    }
}

unsafe impl RefCounted for nsISupportsPriority {
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
pub trait nsISupportsPriorityCoerce {
    fn coerce_from(v: &nsISupportsPriority) -> &Self;
}

impl nsISupportsPriorityCoerce for nsISupportsPriority {
    #[inline]
    fn coerce_from(v: &nsISupportsPriority) -> &Self {
        v
    }
}

impl nsISupportsPriority {
    #[inline]
    pub fn coerce<T: nsISupportsPriorityCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsPriority {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISupportsPriorityCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPriority) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsPriorityVTable {
    pub __base: nsISupportsVTable,

    /* attribute long priority; */
    pub get_priority: unsafe extern "C" fn (this: *const nsISupportsPriority, aPriority: *mut libc::int32_t) -> nsresult,
    pub set_priority: unsafe extern "C" fn (this: *const nsISupportsPriority, aPriority: libc::int32_t) -> nsresult,

    /* void adjustPriority (in long delta); */
    pub adjustPriority: unsafe extern "C" fn (this: *const nsISupportsPriority, delta: libc::int32_t) -> nsresult,

}


impl nsISupportsPriority {
    /* attribute long priority; */
    #[inline]
    pub unsafe fn get_priority(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_priority)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_priority(&self, aPriority: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_priority)(self as *const _, aPriority) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void adjustPriority (in long delta); */
    #[inline]
    pub unsafe fn adjustPriority(&self, delta: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).adjustPriority)(self as *const _, delta) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


