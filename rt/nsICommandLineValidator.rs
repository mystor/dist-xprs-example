//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICommandLineValidator.idl
//


#[repr(C)]
pub struct nsICommandLineValidator {
    vtable: *const nsICommandLineValidatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICommandLineValidator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5ecaa593, 0x7660, 0x4a3a,
            [0x95, 0x7a, 0x92, 0xd5, 0x77, 0x06, 0x71, 0xc7])
    }
}

unsafe impl RefCounted for nsICommandLineValidator {
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
pub trait nsICommandLineValidatorCoerce {
    fn coerce_from(v: &nsICommandLineValidator) -> &Self;
}

impl nsICommandLineValidatorCoerce for nsICommandLineValidator {
    #[inline]
    fn coerce_from(v: &nsICommandLineValidator) -> &Self {
        v
    }
}

impl nsICommandLineValidator {
    #[inline]
    pub fn coerce<T: nsICommandLineValidatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICommandLineValidator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICommandLineValidatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandLineValidator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICommandLineValidatorVTable {
    pub __base: nsISupportsVTable,

    /* void validate (in nsICommandLine aCommandLine); */
    pub validate: unsafe extern "C" fn (this: *const nsICommandLineValidator, aCommandLine: *const nsICommandLine) -> nsresult,

}


impl nsICommandLineValidator {
    /* void validate (in nsICommandLine aCommandLine); */
    #[inline]
    pub unsafe fn validate(&self, aCommandLine: Option<&nsICommandLine>) -> Result<(), nsresult> {

        match ((*self.vtable).validate)(self as *const _, aCommandLine.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


