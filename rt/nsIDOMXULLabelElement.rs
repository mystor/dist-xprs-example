//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULLabelElement.idl
//


#[repr(C)]
pub struct nsIDOMXULLabelElement {
    vtable: *const nsIDOMXULLabelElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULLabelElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8b67286b, 0xbf86, 0x4808,
            [0x8a, 0x68, 0xec, 0xdb, 0xc7, 0xd9, 0x87, 0x7b])
    }
}

unsafe impl RefCounted for nsIDOMXULLabelElement {
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
pub trait nsIDOMXULLabelElementCoerce {
    fn coerce_from(v: &nsIDOMXULLabelElement) -> &Self;
}

impl nsIDOMXULLabelElementCoerce for nsIDOMXULLabelElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULLabelElement) -> &Self {
        v
    }
}

impl nsIDOMXULLabelElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULLabelElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULLabelElement {
    type Target = nsIDOMXULDescriptionElement;
    #[inline]
    fn deref(&self) -> &nsIDOMXULDescriptionElement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMXULDescriptionElementCoerce> nsIDOMXULLabelElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULLabelElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULLabelElementVTable {
    pub __base: nsIDOMXULDescriptionElementVTable,

    /* attribute DOMString accessKey; */
    pub get_accessKey: unsafe extern "C" fn (this: *const nsIDOMXULLabelElement, aAccessKey: *mut nsAString) -> nsresult,
    pub set_accessKey: unsafe extern "C" fn (this: *const nsIDOMXULLabelElement, aAccessKey: *const nsAString) -> nsresult,

    /* attribute DOMString control; */
    pub get_control: unsafe extern "C" fn (this: *const nsIDOMXULLabelElement, aControl: *mut nsAString) -> nsresult,
    pub set_control: unsafe extern "C" fn (this: *const nsIDOMXULLabelElement, aControl: *const nsAString) -> nsresult,

}


impl nsIDOMXULLabelElement {
    /* attribute DOMString accessKey; */
    #[inline]
    pub unsafe fn get_accessKey(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_accessKey)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_accessKey(&self, aAccessKey: &[u16]) -> Result<(), nsresult> {
        let aAccessKey = nsString::from(aAccessKey);
        match ((*self.vtable).set_accessKey)(self as *const _, &*aAccessKey) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString control; */
    #[inline]
    pub unsafe fn get_control(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_control)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_control(&self, aControl: &[u16]) -> Result<(), nsresult> {
        let aControl = nsString::from(aControl);
        match ((*self.vtable).set_control)(self as *const _, &*aControl) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


