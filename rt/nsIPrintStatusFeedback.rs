//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrintStatusFeedback.idl
//


#[repr(C)]
pub struct nsIPrintStatusFeedback {
    vtable: *const nsIPrintStatusFeedbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrintStatusFeedback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8eb6ffc9, 0x715c, 0x487e,
            [0x92, 0x7c, 0xc5, 0x61, 0x39, 0x22, 0x95, 0x48])
    }
}

unsafe impl RefCounted for nsIPrintStatusFeedback {
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
pub trait nsIPrintStatusFeedbackCoerce {
    fn coerce_from(v: &nsIPrintStatusFeedback) -> &Self;
}

impl nsIPrintStatusFeedbackCoerce for nsIPrintStatusFeedback {
    #[inline]
    fn coerce_from(v: &nsIPrintStatusFeedback) -> &Self {
        v
    }
}

impl nsIPrintStatusFeedback {
    #[inline]
    pub fn coerce<T: nsIPrintStatusFeedbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrintStatusFeedback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPrintStatusFeedbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrintStatusFeedback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrintStatusFeedbackVTable {
    pub __base: nsISupportsVTable,

    /* void showStatusString (in wstring status); */
    pub showStatusString: unsafe extern "C" fn (this: *const nsIPrintStatusFeedback, status: *const libc::int16_t) -> nsresult,

    /* void startMeteors (); */
    pub startMeteors: unsafe extern "C" fn (this: *const nsIPrintStatusFeedback) -> nsresult,

    /* void stopMeteors (); */
    pub stopMeteors: unsafe extern "C" fn (this: *const nsIPrintStatusFeedback) -> nsresult,

    /* void showProgress (in long percent); */
    pub showProgress: unsafe extern "C" fn (this: *const nsIPrintStatusFeedback, percent: libc::int32_t) -> nsresult,

    /* [noscript] void setDocShell (in nsIDocShell shell, in mozIDOMWindowProxy window); */
    pub setDocShell: unsafe extern "C" fn (this: *const nsIPrintStatusFeedback, shell: *const nsIDocShell, window: *const mozIDOMWindowProxy) -> nsresult,

    /* void closeWindow (); */
    pub closeWindow: unsafe extern "C" fn (this: *const nsIPrintStatusFeedback) -> nsresult,

}


impl nsIPrintStatusFeedback {
    /* void showStatusString (in wstring status); */
    #[inline]
    pub unsafe fn showStatusString(&self, status: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).showStatusString)(self as *const _, status) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void startMeteors (); */
    #[inline]
    pub unsafe fn startMeteors(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).startMeteors)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stopMeteors (); */
    #[inline]
    pub unsafe fn stopMeteors(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stopMeteors)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showProgress (in long percent); */
    #[inline]
    pub unsafe fn showProgress(&self, percent: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).showProgress)(self as *const _, percent) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setDocShell (in nsIDocShell shell, in mozIDOMWindowProxy window); */
    #[inline]
    pub unsafe fn setDocShell(&self, shell: Option<&nsIDocShell>, window: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).setDocShell)(self as *const _, shell.map_or(::std::ptr::null(), |x| x as *const _), window.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void closeWindow (); */
    #[inline]
    pub unsafe fn closeWindow(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).closeWindow)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


