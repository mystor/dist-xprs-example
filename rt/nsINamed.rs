//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINamed.idl
//


#[repr(C)]
pub struct nsINamed {
    vtable: *const nsINamedVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINamed {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0c5fe7de, 0x7e83, 0x4d0d,
            [0xa8, 0xa6, 0x4a, 0x65, 0x18, 0xb9, 0xa7, 0xb3])
    }
}

unsafe impl RefCounted for nsINamed {
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
pub trait nsINamedCoerce {
    fn coerce_from(v: &nsINamed) -> &Self;
}

impl nsINamedCoerce for nsINamed {
    #[inline]
    fn coerce_from(v: &nsINamed) -> &Self {
        v
    }
}

impl nsINamed {
    #[inline]
    pub fn coerce<T: nsINamedCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINamed {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINamedCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINamed) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINamedVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsINamed, aName: *mut nsACString) -> nsresult,

    /* [noscript] void setName (in string aName); */
    pub setName: unsafe extern "C" fn (this: *const nsINamed, aName: *const libc::c_char) -> nsresult,

}


impl nsINamed {
    /* readonly attribute ACString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void setName (in string aName); */
    #[inline]
    pub unsafe fn setName(&self, aName: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).setName)(self as *const _, aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


