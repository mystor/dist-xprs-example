//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULImageElement.idl
//


#[repr(C)]
pub struct nsIDOMXULImageElement {
    vtable: *const nsIDOMXULImageElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULImageElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0a391077, 0xc509, 0x49d2,
            [0xaf, 0x73, 0x72, 0xe2, 0x11, 0x4e, 0xdd, 0x65])
    }
}

unsafe impl RefCounted for nsIDOMXULImageElement {
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
pub trait nsIDOMXULImageElementCoerce {
    fn coerce_from(v: &nsIDOMXULImageElement) -> &Self;
}

impl nsIDOMXULImageElementCoerce for nsIDOMXULImageElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULImageElement) -> &Self {
        v
    }
}

impl nsIDOMXULImageElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULImageElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULImageElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMXULImageElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULImageElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULImageElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString src; */
    pub get_src: unsafe extern "C" fn (this: *const nsIDOMXULImageElement, aSrc: *mut nsAString) -> nsresult,
    pub set_src: unsafe extern "C" fn (this: *const nsIDOMXULImageElement, aSrc: *const nsAString) -> nsresult,

}


impl nsIDOMXULImageElement {
    /* attribute DOMString src; */
    #[inline]
    pub unsafe fn get_src(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_src)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_src(&self, aSrc: &[u16]) -> Result<(), nsresult> {
        let aSrc = nsString::from(aSrc);
        match ((*self.vtable).set_src)(self as *const _, &*aSrc) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


