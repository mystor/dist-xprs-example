//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAppShell.idl
//


#[repr(C)]
pub struct nsIAppShell {
    vtable: *const nsIAppShellVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAppShell {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7cd5c71d, 0x223b, 0x4afe,
            [0x93, 0x1d, 0x5e, 0xed, 0xb1, 0xf2, 0xb0, 0x1f])
    }
}

unsafe impl RefCounted for nsIAppShell {
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
pub trait nsIAppShellCoerce {
    fn coerce_from(v: &nsIAppShell) -> &Self;
}

impl nsIAppShellCoerce for nsIAppShell {
    #[inline]
    fn coerce_from(v: &nsIAppShell) -> &Self {
        v
    }
}

impl nsIAppShell {
    #[inline]
    pub fn coerce<T: nsIAppShellCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAppShell {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAppShellCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAppShell) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAppShellVTable {
    pub __base: nsISupportsVTable,

    /* void run (); */
    pub run: unsafe extern "C" fn (this: *const nsIAppShell) -> nsresult,

    /* void exit (); */
    pub exit: unsafe extern "C" fn (this: *const nsIAppShell) -> nsresult,

    /* void favorPerformanceHint (in boolean favorPerfOverStarvation, in unsigned long starvationDelay); */
    pub favorPerformanceHint: unsafe extern "C" fn (this: *const nsIAppShell, favorPerfOverStarvation: bool, starvationDelay: libc::uint32_t) -> nsresult,

    /* void suspendNative (); */
    pub suspendNative: unsafe extern "C" fn (this: *const nsIAppShell) -> nsresult,

    /* void resumeNative (); */
    pub resumeNative: unsafe extern "C" fn (this: *const nsIAppShell) -> nsresult,

    /* readonly attribute unsigned long eventloopNestingLevel; */
    pub get_eventloopNestingLevel: unsafe extern "C" fn (this: *const nsIAppShell, aEventloopNestingLevel: *mut libc::uint32_t) -> nsresult,

}


impl nsIAppShell {
    /* void run (); */
    #[inline]
    pub unsafe fn run(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).run)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void exit (); */
    #[inline]
    pub unsafe fn exit(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).exit)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void favorPerformanceHint (in boolean favorPerfOverStarvation, in unsigned long starvationDelay); */
    #[inline]
    pub unsafe fn favorPerformanceHint(&self, favorPerfOverStarvation: bool, starvationDelay: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).favorPerformanceHint)(self as *const _, favorPerfOverStarvation, starvationDelay) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void suspendNative (); */
    #[inline]
    pub unsafe fn suspendNative(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).suspendNative)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resumeNative (); */
    #[inline]
    pub unsafe fn resumeNative(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resumeNative)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long eventloopNestingLevel; */
    #[inline]
    pub unsafe fn get_eventloopNestingLevel(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_eventloopNestingLevel)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


