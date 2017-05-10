//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFormPOSTActionChannel.idl
//


#[repr(C)]
pub struct nsIFormPOSTActionChannel {
    vtable: *const nsIFormPOSTActionChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFormPOSTActionChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfc826b53, 0x0db8, 0x42b4,
            [0xaa, 0x6a, 0x5d, 0xd2, 0xcf, 0xca, 0x52, 0xa4])
    }
}

unsafe impl RefCounted for nsIFormPOSTActionChannel {
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
pub trait nsIFormPOSTActionChannelCoerce {
    fn coerce_from(v: &nsIFormPOSTActionChannel) -> &Self;
}

impl nsIFormPOSTActionChannelCoerce for nsIFormPOSTActionChannel {
    #[inline]
    fn coerce_from(v: &nsIFormPOSTActionChannel) -> &Self {
        v
    }
}

impl nsIFormPOSTActionChannel {
    #[inline]
    pub fn coerce<T: nsIFormPOSTActionChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFormPOSTActionChannel {
    type Target = nsIUploadChannel;
    #[inline]
    fn deref(&self) -> &nsIUploadChannel {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIUploadChannelCoerce> nsIFormPOSTActionChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFormPOSTActionChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFormPOSTActionChannelVTable {
    pub __base: nsIUploadChannelVTable,

}


impl nsIFormPOSTActionChannel {
}


