//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLPreElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLPreElement {
    vtable: *const nsIDOMHTMLPreElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLPreElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa2441b77, 0xad22, 0x4275,
            [0xb1, 0xdd, 0x1b, 0x58, 0xc0, 0x44, 0xfd, 0x04])
    }
}

unsafe impl RefCounted for nsIDOMHTMLPreElement {
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
pub trait nsIDOMHTMLPreElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLPreElement) -> &Self;
}

impl nsIDOMHTMLPreElementCoerce for nsIDOMHTMLPreElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLPreElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLPreElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLPreElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLPreElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLPreElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLPreElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLPreElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute long width; */
    pub get_width: unsafe extern "C" fn (this: *const nsIDOMHTMLPreElement, aWidth: *mut libc::int32_t) -> nsresult,
    pub set_width: unsafe extern "C" fn (this: *const nsIDOMHTMLPreElement, aWidth: libc::int32_t) -> nsresult,

}


impl nsIDOMHTMLPreElement {
    /* attribute long width; */
    #[inline]
    pub unsafe fn get_width(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_width)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_width(&self, aWidth: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_width)(self as *const _, aWidth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


