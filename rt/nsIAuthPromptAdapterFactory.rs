//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAuthPromptAdapterFactory.idl
//


#[repr(C)]
pub struct nsIAuthPromptAdapterFactory {
    vtable: *const nsIAuthPromptAdapterFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAuthPromptAdapterFactory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x60e46383, 0xbb9a, 0x4860,
            [0x89, 0x62, 0x80, 0xd9, 0xc5, 0xc0, 0x5d, 0xdc])
    }
}

unsafe impl RefCounted for nsIAuthPromptAdapterFactory {
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
pub trait nsIAuthPromptAdapterFactoryCoerce {
    fn coerce_from(v: &nsIAuthPromptAdapterFactory) -> &Self;
}

impl nsIAuthPromptAdapterFactoryCoerce for nsIAuthPromptAdapterFactory {
    #[inline]
    fn coerce_from(v: &nsIAuthPromptAdapterFactory) -> &Self {
        v
    }
}

impl nsIAuthPromptAdapterFactory {
    #[inline]
    pub fn coerce<T: nsIAuthPromptAdapterFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAuthPromptAdapterFactory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAuthPromptAdapterFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAuthPromptAdapterFactory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAuthPromptAdapterFactoryVTable {
    pub __base: nsISupportsVTable,

    /* nsIAuthPrompt2 createAdapter (in nsIAuthPrompt aPrompt); */
    pub createAdapter: unsafe extern "C" fn (this: *const nsIAuthPromptAdapterFactory, aPrompt: *const nsIAuthPrompt, _retval: *mut *const nsIAuthPrompt2) -> nsresult,

}


impl nsIAuthPromptAdapterFactory {
    /* nsIAuthPrompt2 createAdapter (in nsIAuthPrompt aPrompt); */
    #[inline]
    pub unsafe fn createAdapter(&self, aPrompt: Option<&nsIAuthPrompt>) -> Result<Option<RefPtr<nsIAuthPrompt2>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createAdapter)(self as *const _, aPrompt.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


