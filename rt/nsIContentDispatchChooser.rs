//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentDispatchChooser.idl
//


pub mod nsIContentDispatchChooser_consts {
    pub const REASON_CANNOT_HANDLE: i64 = 0;
}


#[repr(C)]
pub struct nsIContentDispatchChooser {
    vtable: *const nsIContentDispatchChooserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentDispatchChooser {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x456ca3b2, 0x02be, 0x4f97,
            [0x89, 0xa2, 0x08, 0xc0, 0x8d, 0x3a, 0xd8, 0x8f])
    }
}

unsafe impl RefCounted for nsIContentDispatchChooser {
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
pub trait nsIContentDispatchChooserCoerce {
    fn coerce_from(v: &nsIContentDispatchChooser) -> &Self;
}

impl nsIContentDispatchChooserCoerce for nsIContentDispatchChooser {
    #[inline]
    fn coerce_from(v: &nsIContentDispatchChooser) -> &Self {
        v
    }
}

impl nsIContentDispatchChooser {
    #[inline]
    pub fn coerce<T: nsIContentDispatchChooserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentDispatchChooser {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentDispatchChooserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentDispatchChooser) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentDispatchChooserVTable {
    pub __base: nsISupportsVTable,

    /* void ask (in nsIHandlerInfo aHandler, in nsIInterfaceRequestor aWindowContext, in nsIURI aURI, in unsigned long aReason); */
    pub ask: unsafe extern "C" fn (this: *const nsIContentDispatchChooser, aHandler: *const nsIHandlerInfo, aWindowContext: *const nsIInterfaceRequestor, aURI: *const nsIURI, aReason: libc::uint32_t) -> nsresult,

}


impl nsIContentDispatchChooser {
    /* void ask (in nsIHandlerInfo aHandler, in nsIInterfaceRequestor aWindowContext, in nsIURI aURI, in unsigned long aReason); */
    #[inline]
    pub unsafe fn ask(&self, aHandler: Option<&nsIHandlerInfo>, aWindowContext: Option<&nsIInterfaceRequestor>, aURI: Option<&nsIURI>, aReason: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).ask)(self as *const _, aHandler.map_or(::std::ptr::null(), |x| x as *const _), aWindowContext.map_or(::std::ptr::null(), |x| x as *const _), aURI.map_or(::std::ptr::null(), |x| x as *const _), aReason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


