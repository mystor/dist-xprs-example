//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHashable.idl
//


#[repr(C)]
pub struct nsIHashable {
    vtable: *const nsIHashableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHashable {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x17e595fa, 0xb57a, 0x4933,
            [0xbd, 0x0f, 0xb1, 0x81, 0x2e, 0x8a, 0xb1, 0x88])
    }
}

unsafe impl RefCounted for nsIHashable {
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
pub trait nsIHashableCoerce {
    fn coerce_from(v: &nsIHashable) -> &Self;
}

impl nsIHashableCoerce for nsIHashable {
    #[inline]
    fn coerce_from(v: &nsIHashable) -> &Self {
        v
    }
}

impl nsIHashable {
    #[inline]
    pub fn coerce<T: nsIHashableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHashable {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHashableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHashable) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHashableVTable {
    pub __base: nsISupportsVTable,

    /* boolean equals (in nsIHashable aOther); */
    pub equals: unsafe extern "C" fn (this: *const nsIHashable, aOther: *const nsIHashable, _retval: *mut bool) -> nsresult,

    /* readonly attribute unsigned long hashCode; */
    pub get_hashCode: unsafe extern "C" fn (this: *const nsIHashable, aHashCode: *mut libc::uint32_t) -> nsresult,

}


impl nsIHashable {
    /* boolean equals (in nsIHashable aOther); */
    #[inline]
    pub unsafe fn equals(&self, aOther: Option<&nsIHashable>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).equals)(self as *const _, aOther.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long hashCode; */
    #[inline]
    pub unsafe fn get_hashCode(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_hashCode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


