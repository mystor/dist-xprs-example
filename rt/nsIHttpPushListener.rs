//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpPushListener.idl
//


#[repr(C)]
pub struct nsIHttpPushListener {
    vtable: *const nsIHttpPushListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpPushListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0d6ce59c, 0xad5d, 0x4520,
            [0xb4, 0xd3, 0x09, 0x66, 0x48, 0x68, 0xf2, 0x79])
    }
}

unsafe impl RefCounted for nsIHttpPushListener {
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
pub trait nsIHttpPushListenerCoerce {
    fn coerce_from(v: &nsIHttpPushListener) -> &Self;
}

impl nsIHttpPushListenerCoerce for nsIHttpPushListener {
    #[inline]
    fn coerce_from(v: &nsIHttpPushListener) -> &Self {
        v
    }
}

impl nsIHttpPushListener {
    #[inline]
    pub fn coerce<T: nsIHttpPushListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpPushListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpPushListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpPushListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpPushListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onPush (in nsIHttpChannel associatedChannel, in nsIHttpChannel pushChannel); */
    pub onPush: unsafe extern "C" fn (this: *const nsIHttpPushListener, associatedChannel: *const nsIHttpChannel, pushChannel: *const nsIHttpChannel) -> nsresult,

}


impl nsIHttpPushListener {
    /* void onPush (in nsIHttpChannel associatedChannel, in nsIHttpChannel pushChannel); */
    #[inline]
    pub unsafe fn onPush(&self, associatedChannel: Option<&nsIHttpChannel>, pushChannel: Option<&nsIHttpChannel>) -> Result<(), nsresult> {

        match ((*self.vtable).onPush)(self as *const _, associatedChannel.map_or(::std::ptr::null(), |x| x as *const _), pushChannel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


