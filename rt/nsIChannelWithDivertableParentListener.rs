//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIChannelWithDivertableParentListener.idl
//


#[repr(C)]
pub struct nsIChannelWithDivertableParentListener {
    vtable: *const nsIChannelWithDivertableParentListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIChannelWithDivertableParentListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc073d79f, 0x2503, 0x4dff,
            [0xba, 0x87, 0xd3, 0x07, 0x1f, 0x8b, 0x43, 0x3b])
    }
}

unsafe impl RefCounted for nsIChannelWithDivertableParentListener {
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
pub trait nsIChannelWithDivertableParentListenerCoerce {
    fn coerce_from(v: &nsIChannelWithDivertableParentListener) -> &Self;
}

impl nsIChannelWithDivertableParentListenerCoerce for nsIChannelWithDivertableParentListener {
    #[inline]
    fn coerce_from(v: &nsIChannelWithDivertableParentListener) -> &Self {
        v
    }
}

impl nsIChannelWithDivertableParentListener {
    #[inline]
    pub fn coerce<T: nsIChannelWithDivertableParentListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIChannelWithDivertableParentListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIChannelWithDivertableParentListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIChannelWithDivertableParentListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIChannelWithDivertableParentListenerVTable {
    pub __base: nsISupportsVTable,

    /* void MessageDiversionStarted (in ADivertableParentChannelPtr aParentChannel); */
    /// Unable to call function as its signature contains a non-rust type
    pub MessageDiversionStarted: *const ::libc::c_void,

    /* void MessageDiversionStop (); */
    pub MessageDiversionStop: unsafe extern "C" fn (this: *const nsIChannelWithDivertableParentListener) -> nsresult,

    /* void SuspendInternal (); */
    pub SuspendInternal: unsafe extern "C" fn (this: *const nsIChannelWithDivertableParentListener) -> nsresult,

    /* void ResumeInternal (); */
    pub ResumeInternal: unsafe extern "C" fn (this: *const nsIChannelWithDivertableParentListener) -> nsresult,

}


impl nsIChannelWithDivertableParentListener {
    /* void MessageDiversionStarted (in ADivertableParentChannelPtr aParentChannel); */


    /* void MessageDiversionStop (); */
    #[inline]
    pub unsafe fn MessageDiversionStop(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).MessageDiversionStop)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void SuspendInternal (); */
    #[inline]
    pub unsafe fn SuspendInternal(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).SuspendInternal)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void ResumeInternal (); */
    #[inline]
    pub unsafe fn ResumeInternal(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).ResumeInternal)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


