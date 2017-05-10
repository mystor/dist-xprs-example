//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozISpellI18NManager.idl
//


#[repr(C)]
pub struct mozISpellI18NManager {
    vtable: *const mozISpellI18NManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozISpellI18NManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xaeb8936f, 0x219c, 0x4d3c,
            [0x83, 0x85, 0xd9, 0x38, 0x2d, 0xaa, 0x55, 0x1a])
    }
}

unsafe impl RefCounted for mozISpellI18NManager {
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
pub trait mozISpellI18NManagerCoerce {
    fn coerce_from(v: &mozISpellI18NManager) -> &Self;
}

impl mozISpellI18NManagerCoerce for mozISpellI18NManager {
    #[inline]
    fn coerce_from(v: &mozISpellI18NManager) -> &Self {
        v
    }
}

impl mozISpellI18NManager {
    #[inline]
    pub fn coerce<T: mozISpellI18NManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozISpellI18NManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozISpellI18NManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &mozISpellI18NManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozISpellI18NManagerVTable {
    pub __base: nsISupportsVTable,

    /* mozISpellI18NUtil getUtil (in wstring language); */
    pub getUtil: unsafe extern "C" fn (this: *const mozISpellI18NManager, language: *const libc::int16_t, _retval: *mut *const mozISpellI18NUtil) -> nsresult,

}


impl mozISpellI18NManager {
    /* mozISpellI18NUtil getUtil (in wstring language); */
    #[inline]
    pub unsafe fn getUtil(&self, language: *const libc::int16_t) -> Result<Option<RefPtr<mozISpellI18NUtil>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getUtil)(self as *const _, language, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


