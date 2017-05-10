//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPopupWindowManager.idl
//


pub mod nsIPopupWindowManager_consts {
    pub const ALLOW_POPUP: i64 = 1;
    pub const DENY_POPUP: i64 = 2;
    pub const ALLOW_POPUP_WITH_PREJUDICE: i64 = 3;
}


#[repr(C)]
pub struct nsIPopupWindowManager {
    vtable: *const nsIPopupWindowManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPopupWindowManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x66386aa9, 0x2088, 0x4bae,
            [0x82, 0xc7, 0x9f, 0x58, 0xbc, 0x02, 0xbe, 0x64])
    }
}

unsafe impl RefCounted for nsIPopupWindowManager {
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
pub trait nsIPopupWindowManagerCoerce {
    fn coerce_from(v: &nsIPopupWindowManager) -> &Self;
}

impl nsIPopupWindowManagerCoerce for nsIPopupWindowManager {
    #[inline]
    fn coerce_from(v: &nsIPopupWindowManager) -> &Self {
        v
    }
}

impl nsIPopupWindowManager {
    #[inline]
    pub fn coerce<T: nsIPopupWindowManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPopupWindowManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPopupWindowManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPopupWindowManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPopupWindowManagerVTable {
    pub __base: nsISupportsVTable,

    /* uint32_t testPermission (in nsIPrincipal principal); */
    pub testPermission: unsafe extern "C" fn (this: *const nsIPopupWindowManager, principal: *const nsIPrincipal, _retval: *mut uint32_t) -> nsresult,

}


impl nsIPopupWindowManager {
    /* uint32_t testPermission (in nsIPrincipal principal); */
    #[inline]
    pub unsafe fn testPermission(&self, principal: Option<&nsIPrincipal>) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).testPermission)(self as *const _, principal.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


