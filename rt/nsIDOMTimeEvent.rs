//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMTimeEvent.idl
//


#[repr(C)]
pub struct nsIDOMTimeEvent {
    vtable: *const nsIDOMTimeEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMTimeEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb5e7fbac, 0xf572, 0x426c,
            [0x93, 0x20, 0x0e, 0xf7, 0x63, 0x0f, 0x03, 0xc1])
    }
}

unsafe impl RefCounted for nsIDOMTimeEvent {
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
pub trait nsIDOMTimeEventCoerce {
    fn coerce_from(v: &nsIDOMTimeEvent) -> &Self;
}

impl nsIDOMTimeEventCoerce for nsIDOMTimeEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMTimeEvent) -> &Self {
        v
    }
}

impl nsIDOMTimeEvent {
    #[inline]
    pub fn coerce<T: nsIDOMTimeEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMTimeEvent {
    type Target = nsIDOMEvent;
    #[inline]
    fn deref(&self) -> &nsIDOMEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMEventCoerce> nsIDOMTimeEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMTimeEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMTimeEventVTable {
    pub __base: nsIDOMEventVTable,

    /* readonly attribute long detail; */
    pub get_detail: unsafe extern "C" fn (this: *const nsIDOMTimeEvent, aDetail: *mut libc::int32_t) -> nsresult,

}


impl nsIDOMTimeEvent {
    /* readonly attribute long detail; */
    #[inline]
    pub unsafe fn get_detail(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_detail)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


