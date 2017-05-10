//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMFormData.idl
//


#[repr(C)]
pub struct nsIDOMFormData {
    vtable: *const nsIDOMFormDataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMFormData {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x256c9139, 0x5a29, 0x41e1,
            [0x86, 0x98, 0xf9, 0xf9, 0xaa, 0xe7, 0xd6, 0xcf])
    }
}

unsafe impl RefCounted for nsIDOMFormData {
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
pub trait nsIDOMFormDataCoerce {
    fn coerce_from(v: &nsIDOMFormData) -> &Self;
}

impl nsIDOMFormDataCoerce for nsIDOMFormData {
    #[inline]
    fn coerce_from(v: &nsIDOMFormData) -> &Self {
        v
    }
}

impl nsIDOMFormData {
    #[inline]
    pub fn coerce<T: nsIDOMFormDataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMFormData {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMFormDataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMFormData) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMFormDataVTable {
    pub __base: nsISupportsVTable,

    /* void append (in DOMString name, in nsIVariant value); */
    pub append: unsafe extern "C" fn (this: *const nsIDOMFormData, name: *const nsAString, value: *const nsIVariant) -> nsresult,

}


impl nsIDOMFormData {
    /* void append (in DOMString name, in nsIVariant value); */
    #[inline]
    pub unsafe fn append(&self, name: &[u16], value: Option<&nsIVariant>) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).append)(self as *const _, &*name, value.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


