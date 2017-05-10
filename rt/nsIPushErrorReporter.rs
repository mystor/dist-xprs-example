//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPushErrorReporter.idl
//


pub mod nsIPushErrorReporter_consts {
    pub const ACK_DELIVERED: i64 = 0;
    pub const ACK_DECRYPTION_ERROR: i64 = 1;
    pub const ACK_NOT_DELIVERED: i64 = 2;
    pub const UNSUBSCRIBE_MANUAL: i64 = 3;
    pub const UNSUBSCRIBE_QUOTA_EXCEEDED: i64 = 4;
    pub const UNSUBSCRIBE_PERMISSION_REVOKED: i64 = 5;
    pub const DELIVERY_UNCAUGHT_EXCEPTION: i64 = 6;
    pub const DELIVERY_UNHANDLED_REJECTION: i64 = 7;
    pub const DELIVERY_INTERNAL_ERROR: i64 = 8;
}


#[repr(C)]
pub struct nsIPushErrorReporter {
    vtable: *const nsIPushErrorReporterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPushErrorReporter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb58249f9, 0x1a04, 0x48cc,
            [0xbc, 0x20, 0x2c, 0x99, 0x2d, 0x64, 0xc7, 0x3e])
    }
}

unsafe impl RefCounted for nsIPushErrorReporter {
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
pub trait nsIPushErrorReporterCoerce {
    fn coerce_from(v: &nsIPushErrorReporter) -> &Self;
}

impl nsIPushErrorReporterCoerce for nsIPushErrorReporter {
    #[inline]
    fn coerce_from(v: &nsIPushErrorReporter) -> &Self {
        v
    }
}

impl nsIPushErrorReporter {
    #[inline]
    pub fn coerce<T: nsIPushErrorReporterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPushErrorReporter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPushErrorReporterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushErrorReporter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPushErrorReporterVTable {
    pub __base: nsISupportsVTable,

    /* void reportDeliveryError (in DOMString messageId, [optional] in uint16_t reason); */
    pub reportDeliveryError: unsafe extern "C" fn (this: *const nsIPushErrorReporter, messageId: *const nsAString, reason: uint16_t) -> nsresult,

}


impl nsIPushErrorReporter {
    /* void reportDeliveryError (in DOMString messageId, [optional] in uint16_t reason); */
    #[inline]
    pub unsafe fn reportDeliveryError(&self, messageId: &[u16], reason: uint16_t) -> Result<(), nsresult> {
        let messageId = nsString::from(messageId);
        match ((*self.vtable).reportDeliveryError)(self as *const _, &*messageId, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


