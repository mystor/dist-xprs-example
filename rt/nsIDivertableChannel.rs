//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDivertableChannel.idl
//


#[repr(C)]
pub struct nsIDivertableChannel {
    vtable: *const nsIDivertableChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDivertableChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7a9bf52d, 0xf828, 0x4b31,
            [0xb8, 0xdf, 0xb4, 0x0f, 0xdd, 0x37, 0xd0, 0x07])
    }
}

unsafe impl RefCounted for nsIDivertableChannel {
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
pub trait nsIDivertableChannelCoerce {
    fn coerce_from(v: &nsIDivertableChannel) -> &Self;
}

impl nsIDivertableChannelCoerce for nsIDivertableChannel {
    #[inline]
    fn coerce_from(v: &nsIDivertableChannel) -> &Self {
        v
    }
}

impl nsIDivertableChannel {
    #[inline]
    pub fn coerce<T: nsIDivertableChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDivertableChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDivertableChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDivertableChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDivertableChannelVTable {
    pub __base: nsISupportsVTable,

    /* ChannelDiverterChild divertToParent (); */
    /// Unable to call function as its signature contains a non-rust type
    pub divertToParent: *const ::libc::c_void,

    /* void unknownDecoderInvolvedKeepData (); */
    pub unknownDecoderInvolvedKeepData: unsafe extern "C" fn (this: *const nsIDivertableChannel) -> nsresult,

    /* void unknownDecoderInvolvedOnStartRequestCalled (); */
    pub unknownDecoderInvolvedOnStartRequestCalled: unsafe extern "C" fn (this: *const nsIDivertableChannel) -> nsresult,

    /* readonly attribute bool divertingToParent; */
    pub get_divertingToParent: unsafe extern "C" fn (this: *const nsIDivertableChannel, aDivertingToParent: *mut bool) -> nsresult,

}


impl nsIDivertableChannel {
    /* ChannelDiverterChild divertToParent (); */


    /* void unknownDecoderInvolvedKeepData (); */
    #[inline]
    pub unsafe fn unknownDecoderInvolvedKeepData(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).unknownDecoderInvolvedKeepData)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unknownDecoderInvolvedOnStartRequestCalled (); */
    #[inline]
    pub unsafe fn unknownDecoderInvolvedOnStartRequestCalled(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).unknownDecoderInvolvedOnStartRequestCalled)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute bool divertingToParent; */
    #[inline]
    pub unsafe fn get_divertingToParent(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_divertingToParent)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


