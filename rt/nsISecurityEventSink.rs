//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISecurityEventSink.idl
//


#[repr(C)]
pub struct nsISecurityEventSink {
    vtable: *const nsISecurityEventSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISecurityEventSink {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa71aee68, 0xdd38, 0x4736,
            [0xbd, 0x79, 0x03, 0x5f, 0xea, 0x1a, 0x1e, 0xc6])
    }
}

unsafe impl RefCounted for nsISecurityEventSink {
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
pub trait nsISecurityEventSinkCoerce {
    fn coerce_from(v: &nsISecurityEventSink) -> &Self;
}

impl nsISecurityEventSinkCoerce for nsISecurityEventSink {
    #[inline]
    fn coerce_from(v: &nsISecurityEventSink) -> &Self {
        v
    }
}

impl nsISecurityEventSink {
    #[inline]
    pub fn coerce<T: nsISecurityEventSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISecurityEventSink {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISecurityEventSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISecurityEventSink) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISecurityEventSinkVTable {
    pub __base: nsISupportsVTable,

    /* void onSecurityChange (in nsISupports i_Context, in unsigned long state); */
    pub onSecurityChange: unsafe extern "C" fn (this: *const nsISecurityEventSink, i_Context: *const nsISupports, state: libc::uint32_t) -> nsresult,

}


impl nsISecurityEventSink {
    /* void onSecurityChange (in nsISupports i_Context, in unsigned long state); */
    #[inline]
    pub unsafe fn onSecurityChange(&self, i_Context: Option<&nsISupports>, state: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onSecurityChange)(self as *const _, i_Context.map_or(::std::ptr::null(), |x| x as *const _), state) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


