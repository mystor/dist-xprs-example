//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICancelable.idl
//


#[repr(C)]
pub struct nsICancelable {
    vtable: *const nsICancelableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICancelable {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd94ac0a0, 0xbb18, 0x46b8,
            [0x84, 0x4e, 0x84, 0x15, 0x90, 0x64, 0xb0, 0xbd])
    }
}

unsafe impl RefCounted for nsICancelable {
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
pub trait nsICancelableCoerce {
    fn coerce_from(v: &nsICancelable) -> &Self;
}

impl nsICancelableCoerce for nsICancelable {
    #[inline]
    fn coerce_from(v: &nsICancelable) -> &Self {
        v
    }
}

impl nsICancelable {
    #[inline]
    pub fn coerce<T: nsICancelableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICancelable {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICancelableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICancelable) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICancelableVTable {
    pub __base: nsISupportsVTable,

    /* void cancel (in nsresult aReason); */
    pub cancel: unsafe extern "C" fn (this: *const nsICancelable, aReason: nsresult) -> nsresult,

}


impl nsICancelable {
    /* void cancel (in nsresult aReason); */
    #[inline]
    pub unsafe fn cancel(&self, aReason: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).cancel)(self as *const _, aReason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


