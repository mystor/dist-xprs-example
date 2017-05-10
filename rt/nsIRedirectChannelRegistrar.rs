//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRedirectChannelRegistrar.idl
//


#[repr(C)]
pub struct nsIRedirectChannelRegistrar {
    vtable: *const nsIRedirectChannelRegistrarVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRedirectChannelRegistrar {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xefa36ea2, 0x5b07, 0x46fc,
            [0x95, 0x34, 0xa5, 0xac, 0xb8, 0xb7, 0x7b, 0x72])
    }
}

unsafe impl RefCounted for nsIRedirectChannelRegistrar {
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
pub trait nsIRedirectChannelRegistrarCoerce {
    fn coerce_from(v: &nsIRedirectChannelRegistrar) -> &Self;
}

impl nsIRedirectChannelRegistrarCoerce for nsIRedirectChannelRegistrar {
    #[inline]
    fn coerce_from(v: &nsIRedirectChannelRegistrar) -> &Self {
        v
    }
}

impl nsIRedirectChannelRegistrar {
    #[inline]
    pub fn coerce<T: nsIRedirectChannelRegistrarCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRedirectChannelRegistrar {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRedirectChannelRegistrarCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRedirectChannelRegistrar) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRedirectChannelRegistrarVTable {
    pub __base: nsISupportsVTable,

    /* uint32_t registerChannel (in nsIChannel channel); */
    pub registerChannel: unsafe extern "C" fn (this: *const nsIRedirectChannelRegistrar, channel: *const nsIChannel, _retval: *mut uint32_t) -> nsresult,

    /* nsIChannel linkChannels (in uint32_t id, in nsIParentChannel channel); */
    pub linkChannels: unsafe extern "C" fn (this: *const nsIRedirectChannelRegistrar, id: uint32_t, channel: *const nsIParentChannel, _retval: *mut *const nsIChannel) -> nsresult,

    /* nsIChannel getRegisteredChannel (in uint32_t id); */
    pub getRegisteredChannel: unsafe extern "C" fn (this: *const nsIRedirectChannelRegistrar, id: uint32_t, _retval: *mut *const nsIChannel) -> nsresult,

    /* nsIParentChannel getParentChannel (in uint32_t id); */
    pub getParentChannel: unsafe extern "C" fn (this: *const nsIRedirectChannelRegistrar, id: uint32_t, _retval: *mut *const nsIParentChannel) -> nsresult,

    /* void deregisterChannels (in uint32_t id); */
    pub deregisterChannels: unsafe extern "C" fn (this: *const nsIRedirectChannelRegistrar, id: uint32_t) -> nsresult,

}


impl nsIRedirectChannelRegistrar {
    /* uint32_t registerChannel (in nsIChannel channel); */
    #[inline]
    pub unsafe fn registerChannel(&self, channel: Option<&nsIChannel>) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).registerChannel)(self as *const _, channel.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIChannel linkChannels (in uint32_t id, in nsIParentChannel channel); */
    #[inline]
    pub unsafe fn linkChannels(&self, id: uint32_t, channel: Option<&nsIParentChannel>) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).linkChannels)(self as *const _, id, channel.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIChannel getRegisteredChannel (in uint32_t id); */
    #[inline]
    pub unsafe fn getRegisteredChannel(&self, id: uint32_t) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRegisteredChannel)(self as *const _, id, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIParentChannel getParentChannel (in uint32_t id); */
    #[inline]
    pub unsafe fn getParentChannel(&self, id: uint32_t) -> Result<Option<RefPtr<nsIParentChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getParentChannel)(self as *const _, id, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void deregisterChannels (in uint32_t id); */
    #[inline]
    pub unsafe fn deregisterChannels(&self, id: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).deregisterChannels)(self as *const _, id) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


