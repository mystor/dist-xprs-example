//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMNSEditableElement.idl
//


#[repr(C)]
pub struct nsIDOMNSEditableElement {
    vtable: *const nsIDOMNSEditableElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMNSEditableElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3503de34, 0x6631, 0x4594,
            [0xb7, 0xbe, 0xc3, 0x6f, 0xf6, 0xa5, 0x20, 0xc4])
    }
}

unsafe impl RefCounted for nsIDOMNSEditableElement {
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
pub trait nsIDOMNSEditableElementCoerce {
    fn coerce_from(v: &nsIDOMNSEditableElement) -> &Self;
}

impl nsIDOMNSEditableElementCoerce for nsIDOMNSEditableElement {
    #[inline]
    fn coerce_from(v: &nsIDOMNSEditableElement) -> &Self {
        v
    }
}

impl nsIDOMNSEditableElement {
    #[inline]
    pub fn coerce<T: nsIDOMNSEditableElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMNSEditableElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMNSEditableElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMNSEditableElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMNSEditableElementVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] readonly attribute nsIEditor editor; */
    pub get_editor: unsafe extern "C" fn (this: *const nsIDOMNSEditableElement, aEditor: *mut *const nsIEditor) -> nsresult,

    /* [noscript] void setUserInput (in DOMString input); */
    pub setUserInput: unsafe extern "C" fn (this: *const nsIDOMNSEditableElement, input: *const nsAString) -> nsresult,

}


impl nsIDOMNSEditableElement {
    /* [noscript] readonly attribute nsIEditor editor; */
    #[inline]
    pub unsafe fn get_editor(&self, ) -> Result<Option<RefPtr<nsIEditor>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_editor)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] void setUserInput (in DOMString input); */
    #[inline]
    pub unsafe fn setUserInput(&self, input: &[u16]) -> Result<(), nsresult> {
        let input = nsString::from(input);
        match ((*self.vtable).setUserInput)(self as *const _, &*input) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


