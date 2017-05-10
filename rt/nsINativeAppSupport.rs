//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINativeAppSupport.idl
//


#[repr(C)]
pub struct nsINativeAppSupport {
    vtable: *const nsINativeAppSupportVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINativeAppSupport {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5fdf8480, 0x1f98, 0x11d4,
            [0x80, 0x77, 0x00, 0x60, 0x08, 0x11, 0xa9, 0xc3])
    }
}

unsafe impl RefCounted for nsINativeAppSupport {
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
pub trait nsINativeAppSupportCoerce {
    fn coerce_from(v: &nsINativeAppSupport) -> &Self;
}

impl nsINativeAppSupportCoerce for nsINativeAppSupport {
    #[inline]
    fn coerce_from(v: &nsINativeAppSupport) -> &Self {
        v
    }
}

impl nsINativeAppSupport {
    #[inline]
    pub fn coerce<T: nsINativeAppSupportCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINativeAppSupport {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINativeAppSupportCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeAppSupport) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINativeAppSupportVTable {
    pub __base: nsISupportsVTable,

    /* boolean start (); */
    pub start: unsafe extern "C" fn (this: *const nsINativeAppSupport, _retval: *mut bool) -> nsresult,

    /* void enable (); */
    pub enable: unsafe extern "C" fn (this: *const nsINativeAppSupport) -> nsresult,

    /* boolean stop (); */
    pub stop: unsafe extern "C" fn (this: *const nsINativeAppSupport, _retval: *mut bool) -> nsresult,

    /* void quit (); */
    pub quit: unsafe extern "C" fn (this: *const nsINativeAppSupport) -> nsresult,

    /* void onLastWindowClosing (); */
    pub onLastWindowClosing: unsafe extern "C" fn (this: *const nsINativeAppSupport) -> nsresult,

    /* void ReOpen (); */
    pub ReOpen: unsafe extern "C" fn (this: *const nsINativeAppSupport) -> nsresult,

}


impl nsINativeAppSupport {
    /* boolean start (); */
    #[inline]
    pub unsafe fn start(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).start)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void enable (); */
    #[inline]
    pub unsafe fn enable(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).enable)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean stop (); */
    #[inline]
    pub unsafe fn stop(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).stop)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void quit (); */
    #[inline]
    pub unsafe fn quit(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).quit)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onLastWindowClosing (); */
    #[inline]
    pub unsafe fn onLastWindowClosing(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onLastWindowClosing)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void ReOpen (); */
    #[inline]
    pub unsafe fn ReOpen(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).ReOpen)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


