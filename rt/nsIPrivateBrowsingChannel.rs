//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrivateBrowsingChannel.idl
//


#[repr(C)]
pub struct nsIPrivateBrowsingChannel {
    vtable: *const nsIPrivateBrowsingChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrivateBrowsingChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdf702bb0, 0x55b8, 0x11e2,
            [0xbc, 0xfd, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsIPrivateBrowsingChannel {
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
pub trait nsIPrivateBrowsingChannelCoerce {
    fn coerce_from(v: &nsIPrivateBrowsingChannel) -> &Self;
}

impl nsIPrivateBrowsingChannelCoerce for nsIPrivateBrowsingChannel {
    #[inline]
    fn coerce_from(v: &nsIPrivateBrowsingChannel) -> &Self {
        v
    }
}

impl nsIPrivateBrowsingChannel {
    #[inline]
    pub fn coerce<T: nsIPrivateBrowsingChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrivateBrowsingChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPrivateBrowsingChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrivateBrowsingChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrivateBrowsingChannelVTable {
    pub __base: nsISupportsVTable,

    /* void setPrivate (in boolean aPrivate); */
    pub setPrivate: unsafe extern "C" fn (this: *const nsIPrivateBrowsingChannel, aPrivate: bool) -> nsresult,

    /* readonly attribute boolean isChannelPrivate; */
    pub get_isChannelPrivate: unsafe extern "C" fn (this: *const nsIPrivateBrowsingChannel, aIsChannelPrivate: *mut bool) -> nsresult,

    /* [noscript] boolean isPrivateModeOverriden (out boolean aValue); */
    pub isPrivateModeOverriden: unsafe extern "C" fn (this: *const nsIPrivateBrowsingChannel, aValue: *mut bool, _retval: *mut bool) -> nsresult,

}


impl nsIPrivateBrowsingChannel {
    /* void setPrivate (in boolean aPrivate); */
    #[inline]
    pub unsafe fn setPrivate(&self, aPrivate: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setPrivate)(self as *const _, aPrivate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean isChannelPrivate; */
    #[inline]
    pub unsafe fn get_isChannelPrivate(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isChannelPrivate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] boolean isPrivateModeOverriden (out boolean aValue); */
    #[inline]
    pub unsafe fn isPrivateModeOverriden(&self, ) -> Result<(bool, bool), nsresult> {
        let mut aValue: bool = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isPrivateModeOverriden)(self as *const _, &mut aValue as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aValue, _retval))
    }

}


