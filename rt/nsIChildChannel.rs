//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIChildChannel.idl
//


#[repr(C)]
pub struct nsIChildChannel {
    vtable: *const nsIChildChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIChildChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc45b92ae, 0x4f07, 0x41dd,
            [0xb0, 0xef, 0xaa, 0x04, 0x4e, 0xea, 0xbb, 0x1e])
    }
}

unsafe impl RefCounted for nsIChildChannel {
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
pub trait nsIChildChannelCoerce {
    fn coerce_from(v: &nsIChildChannel) -> &Self;
}

impl nsIChildChannelCoerce for nsIChildChannel {
    #[inline]
    fn coerce_from(v: &nsIChildChannel) -> &Self {
        v
    }
}

impl nsIChildChannel {
    #[inline]
    pub fn coerce<T: nsIChildChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIChildChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIChildChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIChildChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIChildChannelVTable {
    pub __base: nsISupportsVTable,

    /* void connectParent (in uint32_t registrarId); */
    pub connectParent: unsafe extern "C" fn (this: *const nsIChildChannel, registrarId: uint32_t) -> nsresult,

    /* void completeRedirectSetup (in nsIStreamListener aListener, in nsISupports aContext); */
    pub completeRedirectSetup: unsafe extern "C" fn (this: *const nsIChildChannel, aListener: *const nsIStreamListener, aContext: *const nsISupports) -> nsresult,

}


impl nsIChildChannel {
    /* void connectParent (in uint32_t registrarId); */
    #[inline]
    pub unsafe fn connectParent(&self, registrarId: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).connectParent)(self as *const _, registrarId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void completeRedirectSetup (in nsIStreamListener aListener, in nsISupports aContext); */
    #[inline]
    pub unsafe fn completeRedirectSetup(&self, aListener: Option<&nsIStreamListener>, aContext: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).completeRedirectSetup)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


