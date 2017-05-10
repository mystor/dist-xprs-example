//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITokenPasswordDialogs.idl
//


#[repr(C)]
pub struct nsITokenPasswordDialogs {
    vtable: *const nsITokenPasswordDialogsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITokenPasswordDialogs {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x87dbd64a, 0x4466, 0x474e,
            [0x95, 0xf5, 0x1a, 0xd1, 0xce, 0xe5, 0x70, 0x2c])
    }
}

unsafe impl RefCounted for nsITokenPasswordDialogs {
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
pub trait nsITokenPasswordDialogsCoerce {
    fn coerce_from(v: &nsITokenPasswordDialogs) -> &Self;
}

impl nsITokenPasswordDialogsCoerce for nsITokenPasswordDialogs {
    #[inline]
    fn coerce_from(v: &nsITokenPasswordDialogs) -> &Self {
        v
    }
}

impl nsITokenPasswordDialogs {
    #[inline]
    pub fn coerce<T: nsITokenPasswordDialogsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITokenPasswordDialogs {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITokenPasswordDialogsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITokenPasswordDialogs) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITokenPasswordDialogsVTable {
    pub __base: nsISupportsVTable,

    /* boolean setPassword (in nsIInterfaceRequestor ctx, in AString tokenName); */
    pub setPassword: unsafe extern "C" fn (this: *const nsITokenPasswordDialogs, ctx: *const nsIInterfaceRequestor, tokenName: *const nsAString, _retval: *mut bool) -> nsresult,

}


impl nsITokenPasswordDialogs {
    /* boolean setPassword (in nsIInterfaceRequestor ctx, in AString tokenName); */
    #[inline]
    pub unsafe fn setPassword(&self, ctx: Option<&nsIInterfaceRequestor>, tokenName: &[u16]) -> Result<bool, nsresult> {
        let tokenName = nsString::from(tokenName);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).setPassword)(self as *const _, ctx.map_or(::std::ptr::null(), |x| x as *const _), &*tokenName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


