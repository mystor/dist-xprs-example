//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMTimeRanges.idl
//


#[repr(C)]
pub struct nsIDOMTimeRanges {
    vtable: *const nsIDOMTimeRangesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMTimeRanges {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc43448db, 0x0bab, 0x461d,
            [0xb6, 0x48, 0x1c, 0xa1, 0x4a, 0x96, 0x7f, 0x7e])
    }
}

unsafe impl RefCounted for nsIDOMTimeRanges {
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
pub trait nsIDOMTimeRangesCoerce {
    fn coerce_from(v: &nsIDOMTimeRanges) -> &Self;
}

impl nsIDOMTimeRangesCoerce for nsIDOMTimeRanges {
    #[inline]
    fn coerce_from(v: &nsIDOMTimeRanges) -> &Self {
        v
    }
}

impl nsIDOMTimeRanges {
    #[inline]
    pub fn coerce<T: nsIDOMTimeRangesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMTimeRanges {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMTimeRangesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMTimeRanges) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMTimeRangesVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMTimeRanges, aLength: *mut libc::uint32_t) -> nsresult,

    /* double start (in unsigned long index); */
    pub start: unsafe extern "C" fn (this: *const nsIDOMTimeRanges, index: libc::uint32_t, _retval: *mut libc::c_double) -> nsresult,

    /* double end (in unsigned long index); */
    pub end: unsafe extern "C" fn (this: *const nsIDOMTimeRanges, index: libc::uint32_t, _retval: *mut libc::c_double) -> nsresult,

}


impl nsIDOMTimeRanges {
    /* readonly attribute unsigned long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* double start (in unsigned long index); */
    #[inline]
    pub unsafe fn start(&self, index: libc::uint32_t) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).start)(self as *const _, index, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* double end (in unsigned long index); */
    #[inline]
    pub unsafe fn end(&self, index: libc::uint32_t) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).end)(self as *const _, index, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


