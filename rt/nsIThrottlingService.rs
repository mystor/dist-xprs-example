//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIThrottlingService.idl
//


#[repr(C)]
pub struct nsIThrottlingService {
    vtable: *const nsIThrottlingServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIThrottlingService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc755ef98, 0xb749, 0x4f30,
            [0xa6, 0x58, 0x1e, 0x61, 0x10, 0x01, 0x3a, 0x66])
    }
}

unsafe impl RefCounted for nsIThrottlingService {
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
pub trait nsIThrottlingServiceCoerce {
    fn coerce_from(v: &nsIThrottlingService) -> &Self;
}

impl nsIThrottlingServiceCoerce for nsIThrottlingService {
    #[inline]
    fn coerce_from(v: &nsIThrottlingService) -> &Self {
        v
    }
}

impl nsIThrottlingService {
    #[inline]
    pub fn coerce<T: nsIThrottlingServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIThrottlingService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIThrottlingServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThrottlingService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIThrottlingServiceVTable {
    pub __base: nsISupportsVTable,

    /* void addChannel (in nsIHttpChannel channel); */
    pub addChannel: unsafe extern "C" fn (this: *const nsIThrottlingService, channel: *const nsIHttpChannel) -> nsresult,

    /* void removeChannel (in nsIHttpChannel channel); */
    pub removeChannel: unsafe extern "C" fn (this: *const nsIThrottlingService, channel: *const nsIHttpChannel) -> nsresult,

    /* void increasePressure (); */
    pub increasePressure: unsafe extern "C" fn (this: *const nsIThrottlingService) -> nsresult,

    /* void decreasePressure (); */
    pub decreasePressure: unsafe extern "C" fn (this: *const nsIThrottlingService) -> nsresult,

}


impl nsIThrottlingService {
    /* void addChannel (in nsIHttpChannel channel); */
    #[inline]
    pub unsafe fn addChannel(&self, channel: Option<&nsIHttpChannel>) -> Result<(), nsresult> {

        match ((*self.vtable).addChannel)(self as *const _, channel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeChannel (in nsIHttpChannel channel); */
    #[inline]
    pub unsafe fn removeChannel(&self, channel: Option<&nsIHttpChannel>) -> Result<(), nsresult> {

        match ((*self.vtable).removeChannel)(self as *const _, channel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void increasePressure (); */
    #[inline]
    pub unsafe fn increasePressure(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).increasePressure)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void decreasePressure (); */
    #[inline]
    pub unsafe fn decreasePressure(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).decreasePressure)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


