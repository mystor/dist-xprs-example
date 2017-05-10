//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAtomService.idl
//


#[repr(C)]
pub struct nsIAtomService {
    vtable: *const nsIAtomServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAtomService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9c1f50b9, 0xf9eb, 0x42d4,
            [0xa8, 0xcb, 0x2c, 0x76, 0x00, 0xae, 0xb2, 0x41])
    }
}

unsafe impl RefCounted for nsIAtomService {
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
pub trait nsIAtomServiceCoerce {
    fn coerce_from(v: &nsIAtomService) -> &Self;
}

impl nsIAtomServiceCoerce for nsIAtomService {
    #[inline]
    fn coerce_from(v: &nsIAtomService) -> &Self {
        v
    }
}

impl nsIAtomService {
    #[inline]
    pub fn coerce<T: nsIAtomServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAtomService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAtomServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAtomService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAtomServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIAtom getAtom (in AString value); */
    pub getAtom: unsafe extern "C" fn (this: *const nsIAtomService, value: *const nsAString, _retval: *mut *const nsIAtom) -> nsresult,

}


impl nsIAtomService {
    /* nsIAtom getAtom (in AString value); */
    #[inline]
    pub unsafe fn getAtom(&self, value: &[u16]) -> Result<Option<RefPtr<nsIAtom>>, nsresult> {
        let value = nsString::from(value);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAtom)(self as *const _, &*value, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


