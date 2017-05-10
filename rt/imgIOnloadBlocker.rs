//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgIOnloadBlocker.idl
//


#[repr(C)]
pub struct imgIOnloadBlocker {
    vtable: *const imgIOnloadBlockerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for imgIOnloadBlocker {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdc126d90, 0x0ee0, 0x4683,
            [0xb9, 0x42, 0x2f, 0xa6, 0x6e, 0x44, 0x3a, 0xbc])
    }
}

unsafe impl RefCounted for imgIOnloadBlocker {
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
pub trait imgIOnloadBlockerCoerce {
    fn coerce_from(v: &imgIOnloadBlocker) -> &Self;
}

impl imgIOnloadBlockerCoerce for imgIOnloadBlocker {
    #[inline]
    fn coerce_from(v: &imgIOnloadBlocker) -> &Self {
        v
    }
}

impl imgIOnloadBlocker {
    #[inline]
    pub fn coerce<T: imgIOnloadBlockerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for imgIOnloadBlocker {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> imgIOnloadBlockerCoerce for T {
    #[inline]
    fn coerce_from(v: &imgIOnloadBlocker) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct imgIOnloadBlockerVTable {
    pub __base: nsISupportsVTable,

    /* void blockOnload (in imgIRequest aRequest); */
    pub blockOnload: unsafe extern "C" fn (this: *const imgIOnloadBlocker, aRequest: *const imgIRequest) -> nsresult,

    /* void unblockOnload (in imgIRequest aRequest); */
    pub unblockOnload: unsafe extern "C" fn (this: *const imgIOnloadBlocker, aRequest: *const imgIRequest) -> nsresult,

}


impl imgIOnloadBlocker {
    /* void blockOnload (in imgIRequest aRequest); */
    #[inline]
    pub unsafe fn blockOnload(&self, aRequest: Option<&imgIRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).blockOnload)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unblockOnload (in imgIRequest aRequest); */
    #[inline]
    pub unsafe fn unblockOnload(&self, aRequest: Option<&imgIRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).unblockOnload)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


