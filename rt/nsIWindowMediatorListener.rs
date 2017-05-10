//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWindowMediatorListener.idl
//


#[repr(C)]
pub struct nsIWindowMediatorListener {
    vtable: *const nsIWindowMediatorListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWindowMediatorListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2f276982, 0x0d60, 0x4377,
            [0xa5, 0x95, 0xd3, 0x50, 0xba, 0x51, 0x63, 0x95])
    }
}

unsafe impl RefCounted for nsIWindowMediatorListener {
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
pub trait nsIWindowMediatorListenerCoerce {
    fn coerce_from(v: &nsIWindowMediatorListener) -> &Self;
}

impl nsIWindowMediatorListenerCoerce for nsIWindowMediatorListener {
    #[inline]
    fn coerce_from(v: &nsIWindowMediatorListener) -> &Self {
        v
    }
}

impl nsIWindowMediatorListener {
    #[inline]
    pub fn coerce<T: nsIWindowMediatorListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWindowMediatorListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWindowMediatorListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWindowMediatorListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWindowMediatorListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onWindowTitleChange (in nsIXULWindow window, in wstring newTitle); */
    pub onWindowTitleChange: unsafe extern "C" fn (this: *const nsIWindowMediatorListener, window: *const nsIXULWindow, newTitle: *const libc::int16_t) -> nsresult,

    /* void onOpenWindow (in nsIXULWindow window); */
    pub onOpenWindow: unsafe extern "C" fn (this: *const nsIWindowMediatorListener, window: *const nsIXULWindow) -> nsresult,

    /* void onCloseWindow (in nsIXULWindow window); */
    pub onCloseWindow: unsafe extern "C" fn (this: *const nsIWindowMediatorListener, window: *const nsIXULWindow) -> nsresult,

}


impl nsIWindowMediatorListener {
    /* void onWindowTitleChange (in nsIXULWindow window, in wstring newTitle); */
    #[inline]
    pub unsafe fn onWindowTitleChange(&self, window: Option<&nsIXULWindow>, newTitle: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).onWindowTitleChange)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), newTitle) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onOpenWindow (in nsIXULWindow window); */
    #[inline]
    pub unsafe fn onOpenWindow(&self, window: Option<&nsIXULWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).onOpenWindow)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onCloseWindow (in nsIXULWindow window); */
    #[inline]
    pub unsafe fn onCloseWindow(&self, window: Option<&nsIXULWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).onCloseWindow)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


