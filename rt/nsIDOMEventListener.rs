//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMEventListener.idl
//


#[repr(C)]
pub struct nsIDOMEventListener {
    vtable: *const nsIDOMEventListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMEventListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdf31c120, 0xded6, 0x11d1,
            [0xbd, 0x85, 0x00, 0x80, 0x5f, 0x8a, 0xe3, 0xf4])
    }
}

unsafe impl RefCounted for nsIDOMEventListener {
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
pub trait nsIDOMEventListenerCoerce {
    fn coerce_from(v: &nsIDOMEventListener) -> &Self;
}

impl nsIDOMEventListenerCoerce for nsIDOMEventListener {
    #[inline]
    fn coerce_from(v: &nsIDOMEventListener) -> &Self {
        v
    }
}

impl nsIDOMEventListener {
    #[inline]
    pub fn coerce<T: nsIDOMEventListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMEventListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMEventListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMEventListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMEventListenerVTable {
    pub __base: nsISupportsVTable,

    /* void handleEvent (in nsIDOMEvent event); */
    pub handleEvent: unsafe extern "C" fn (this: *const nsIDOMEventListener, event: *const nsIDOMEvent) -> nsresult,

}


impl nsIDOMEventListener {
    /* void handleEvent (in nsIDOMEvent event); */
    #[inline]
    pub unsafe fn handleEvent(&self, event: Option<&nsIDOMEvent>) -> Result<(), nsresult> {

        match ((*self.vtable).handleEvent)(self as *const _, event.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


