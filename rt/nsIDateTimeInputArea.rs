//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDateTimeInputArea.idl
//


#[repr(C)]
pub struct nsIDateTimeInputArea {
    vtable: *const nsIDateTimeInputAreaVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDateTimeInputArea {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x465c0cc3, 0x24cb, 0x48ce,
            [0xaf, 0x1a, 0xb1, 0x84, 0x02, 0x32, 0x6b, 0x05])
    }
}

unsafe impl RefCounted for nsIDateTimeInputArea {
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
pub trait nsIDateTimeInputAreaCoerce {
    fn coerce_from(v: &nsIDateTimeInputArea) -> &Self;
}

impl nsIDateTimeInputAreaCoerce for nsIDateTimeInputArea {
    #[inline]
    fn coerce_from(v: &nsIDateTimeInputArea) -> &Self {
        v
    }
}

impl nsIDateTimeInputArea {
    #[inline]
    pub fn coerce<T: nsIDateTimeInputAreaCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDateTimeInputArea {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDateTimeInputAreaCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDateTimeInputArea) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDateTimeInputAreaVTable {
    pub __base: nsISupportsVTable,

    /* void notifyInputElementValueChanged (); */
    pub notifyInputElementValueChanged: unsafe extern "C" fn (this: *const nsIDateTimeInputArea) -> nsresult,

    /* void setValueFromPicker (in jsval value); */
    /// Unable to call function as its signature contains a non-rust type
    pub setValueFromPicker: *const ::libc::c_void,

    /* void focusInnerTextBox (); */
    pub focusInnerTextBox: unsafe extern "C" fn (this: *const nsIDateTimeInputArea) -> nsresult,

    /* void blurInnerTextBox (); */
    pub blurInnerTextBox: unsafe extern "C" fn (this: *const nsIDateTimeInputArea) -> nsresult,

    /* void setPickerState (in boolean isOpen); */
    pub setPickerState: unsafe extern "C" fn (this: *const nsIDateTimeInputArea, isOpen: bool) -> nsresult,

    /* void setEditAttribute (in DOMString name, in DOMString value); */
    pub setEditAttribute: unsafe extern "C" fn (this: *const nsIDateTimeInputArea, name: *const nsAString, value: *const nsAString) -> nsresult,

    /* void removeEditAttribute (in DOMString name); */
    pub removeEditAttribute: unsafe extern "C" fn (this: *const nsIDateTimeInputArea, name: *const nsAString) -> nsresult,

}


impl nsIDateTimeInputArea {
    /* void notifyInputElementValueChanged (); */
    #[inline]
    pub unsafe fn notifyInputElementValueChanged(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).notifyInputElementValueChanged)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setValueFromPicker (in jsval value); */


    /* void focusInnerTextBox (); */
    #[inline]
    pub unsafe fn focusInnerTextBox(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).focusInnerTextBox)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void blurInnerTextBox (); */
    #[inline]
    pub unsafe fn blurInnerTextBox(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).blurInnerTextBox)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPickerState (in boolean isOpen); */
    #[inline]
    pub unsafe fn setPickerState(&self, isOpen: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setPickerState)(self as *const _, isOpen) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setEditAttribute (in DOMString name, in DOMString value); */
    #[inline]
    pub unsafe fn setEditAttribute(&self, name: &[u16], value: &[u16]) -> Result<(), nsresult> {
        let name = nsString::from(name);
        let value = nsString::from(value);
        match ((*self.vtable).setEditAttribute)(self as *const _, &*name, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeEditAttribute (in DOMString name); */
    #[inline]
    pub unsafe fn removeEditAttribute(&self, name: &[u16]) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).removeEditAttribute)(self as *const _, &*name) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


