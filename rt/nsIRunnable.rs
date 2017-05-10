//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRunnable.idl
//


#[repr(C)]
pub struct nsIRunnable {
    vtable: *const nsIRunnableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRunnable {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4a2abaf0, 0x6886, 0x11d3,
            [0x93, 0x82, 0x00, 0x10, 0x4b, 0xa0, 0xfd, 0x40])
    }
}

unsafe impl RefCounted for nsIRunnable {
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
pub trait nsIRunnableCoerce {
    fn coerce_from(v: &nsIRunnable) -> &Self;
}

impl nsIRunnableCoerce for nsIRunnable {
    #[inline]
    fn coerce_from(v: &nsIRunnable) -> &Self {
        v
    }
}

impl nsIRunnable {
    #[inline]
    pub fn coerce<T: nsIRunnableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRunnable {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRunnableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRunnable) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRunnableVTable {
    pub __base: nsISupportsVTable,

    /* void run (); */
    pub run: unsafe extern "C" fn (this: *const nsIRunnable) -> nsresult,

}


impl nsIRunnable {
    /* void run (); */
    #[inline]
    pub unsafe fn run(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).run)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIRunnablePriority_consts {
    pub const PRIORITY_NORMAL: i64 = 0;
    pub const PRIORITY_HIGH: i64 = 1;
}


#[repr(C)]
pub struct nsIRunnablePriority {
    vtable: *const nsIRunnablePriorityVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRunnablePriority {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe75aa42a, 0x80a9, 0x11e6,
            [0xaf, 0xb5, 0xe8, 0x9d, 0x87, 0x34, 0x8e, 0x2c])
    }
}

unsafe impl RefCounted for nsIRunnablePriority {
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
pub trait nsIRunnablePriorityCoerce {
    fn coerce_from(v: &nsIRunnablePriority) -> &Self;
}

impl nsIRunnablePriorityCoerce for nsIRunnablePriority {
    #[inline]
    fn coerce_from(v: &nsIRunnablePriority) -> &Self {
        v
    }
}

impl nsIRunnablePriority {
    #[inline]
    pub fn coerce<T: nsIRunnablePriorityCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRunnablePriority {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRunnablePriorityCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRunnablePriority) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRunnablePriorityVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long priority; */
    pub get_priority: unsafe extern "C" fn (this: *const nsIRunnablePriority, aPriority: *mut libc::uint32_t) -> nsresult,

}


impl nsIRunnablePriority {
    /* readonly attribute unsigned long priority; */
    #[inline]
    pub unsafe fn get_priority(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_priority)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


