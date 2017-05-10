//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIThreadRetargetableStreamListener.idl
//


#[repr(C)]
pub struct nsIThreadRetargetableStreamListener {
    vtable: *const nsIThreadRetargetableStreamListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIThreadRetargetableStreamListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfb2304b8, 0xf82f, 0x4433,
            [0xaf, 0x68, 0xd8, 0x74, 0xa2, 0xeb, 0xbd, 0xc1])
    }
}

unsafe impl RefCounted for nsIThreadRetargetableStreamListener {
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
pub trait nsIThreadRetargetableStreamListenerCoerce {
    fn coerce_from(v: &nsIThreadRetargetableStreamListener) -> &Self;
}

impl nsIThreadRetargetableStreamListenerCoerce for nsIThreadRetargetableStreamListener {
    #[inline]
    fn coerce_from(v: &nsIThreadRetargetableStreamListener) -> &Self {
        v
    }
}

impl nsIThreadRetargetableStreamListener {
    #[inline]
    pub fn coerce<T: nsIThreadRetargetableStreamListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIThreadRetargetableStreamListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIThreadRetargetableStreamListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThreadRetargetableStreamListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIThreadRetargetableStreamListenerVTable {
    pub __base: nsISupportsVTable,

    /* void checkListenerChain (); */
    pub checkListenerChain: unsafe extern "C" fn (this: *const nsIThreadRetargetableStreamListener) -> nsresult,

}


impl nsIThreadRetargetableStreamListener {
    /* void checkListenerChain (); */
    #[inline]
    pub unsafe fn checkListenerChain(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).checkListenerChain)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


