//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMutable.idl
//


#[repr(C)]
pub struct nsIMutable {
    vtable: *const nsIMutableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMutable {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x321578d0, 0x03c1, 0x4d95,
            [0x88, 0x21, 0x02, 0x1a, 0xc6, 0x12, 0xd1, 0x8d])
    }
}

unsafe impl RefCounted for nsIMutable {
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
pub trait nsIMutableCoerce {
    fn coerce_from(v: &nsIMutable) -> &Self;
}

impl nsIMutableCoerce for nsIMutable {
    #[inline]
    fn coerce_from(v: &nsIMutable) -> &Self {
        v
    }
}

impl nsIMutable {
    #[inline]
    pub fn coerce<T: nsIMutableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMutable {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMutableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMutable) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMutableVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean mutable; */
    pub get_mutable: unsafe extern "C" fn (this: *const nsIMutable, aMutable: *mut bool) -> nsresult,
    pub set_mutable: unsafe extern "C" fn (this: *const nsIMutable, aMutable: bool) -> nsresult,

}


impl nsIMutable {
    /* attribute boolean mutable; */
    #[inline]
    pub unsafe fn get_mutable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mutable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_mutable(&self, aMutable: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_mutable)(self as *const _, aMutable) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


