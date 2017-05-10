//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDeprecationWarner.idl
//


#[repr(C)]
pub struct nsIDeprecationWarner {
    vtable: *const nsIDeprecationWarnerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDeprecationWarner {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x665c5124, 0x2c52, 0x41ba,
            [0xae, 0x72, 0x23, 0x93, 0xf8, 0xe7, 0x6c, 0x25])
    }
}

unsafe impl RefCounted for nsIDeprecationWarner {
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
pub trait nsIDeprecationWarnerCoerce {
    fn coerce_from(v: &nsIDeprecationWarner) -> &Self;
}

impl nsIDeprecationWarnerCoerce for nsIDeprecationWarner {
    #[inline]
    fn coerce_from(v: &nsIDeprecationWarner) -> &Self {
        v
    }
}

impl nsIDeprecationWarner {
    #[inline]
    pub fn coerce<T: nsIDeprecationWarnerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDeprecationWarner {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDeprecationWarnerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDeprecationWarner) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDeprecationWarnerVTable {
    pub __base: nsISupportsVTable,

    /* void issueWarning (in uint32_t aWarning, [optional] in bool aAsError); */
    pub issueWarning: unsafe extern "C" fn (this: *const nsIDeprecationWarner, aWarning: uint32_t, aAsError: bool) -> nsresult,

}


impl nsIDeprecationWarner {
    /* void issueWarning (in uint32_t aWarning, [optional] in bool aAsError); */
    #[inline]
    pub unsafe fn issueWarning(&self, aWarning: uint32_t, aAsError: bool) -> Result<(), nsresult> {

        match ((*self.vtable).issueWarning)(self as *const _, aWarning, aAsError) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


