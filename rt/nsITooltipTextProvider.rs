//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITooltipTextProvider.idl
//


#[repr(C)]
pub struct nsITooltipTextProvider {
    vtable: *const nsITooltipTextProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITooltipTextProvider {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb128a1e6, 0x44f3, 0x4331,
            [0x8f, 0xbe, 0x5a, 0xf3, 0x60, 0xff, 0x21, 0xee])
    }
}

unsafe impl RefCounted for nsITooltipTextProvider {
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
pub trait nsITooltipTextProviderCoerce {
    fn coerce_from(v: &nsITooltipTextProvider) -> &Self;
}

impl nsITooltipTextProviderCoerce for nsITooltipTextProvider {
    #[inline]
    fn coerce_from(v: &nsITooltipTextProvider) -> &Self {
        v
    }
}

impl nsITooltipTextProvider {
    #[inline]
    pub fn coerce<T: nsITooltipTextProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITooltipTextProvider {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITooltipTextProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITooltipTextProvider) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITooltipTextProviderVTable {
    pub __base: nsISupportsVTable,

    /* boolean getNodeText (in nsIDOMNode aNode, out wstring aText, out wstring aDirection); */
    pub getNodeText: unsafe extern "C" fn (this: *const nsITooltipTextProvider, aNode: *const nsIDOMNode, aText: *mut *const libc::int16_t, aDirection: *mut *const libc::int16_t, _retval: *mut bool) -> nsresult,

}


impl nsITooltipTextProvider {
    /* boolean getNodeText (in nsIDOMNode aNode, out wstring aText, out wstring aDirection); */
    #[inline]
    pub unsafe fn getNodeText(&self, aNode: Option<&nsIDOMNode>) -> Result<(*const libc::int16_t, *const libc::int16_t, bool), nsresult> {
        let mut aText: *const libc::int16_t = ::std::mem::zeroed();
        let mut aDirection: *const libc::int16_t = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getNodeText)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), &mut aText as *mut _, &mut aDirection as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aText, aDirection, _retval))
    }

}


