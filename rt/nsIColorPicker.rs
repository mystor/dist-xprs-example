//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIColorPicker.idl
//


#[repr(C)]
pub struct nsIColorPickerShownCallback {
    vtable: *const nsIColorPickerShownCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIColorPickerShownCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd2ce78d1, 0x40b5, 0x49d1,
            [0xb6, 0x6d, 0x58, 0x01, 0xfc, 0xb9, 0xa3, 0x85])
    }
}

unsafe impl RefCounted for nsIColorPickerShownCallback {
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
pub trait nsIColorPickerShownCallbackCoerce {
    fn coerce_from(v: &nsIColorPickerShownCallback) -> &Self;
}

impl nsIColorPickerShownCallbackCoerce for nsIColorPickerShownCallback {
    #[inline]
    fn coerce_from(v: &nsIColorPickerShownCallback) -> &Self {
        v
    }
}

impl nsIColorPickerShownCallback {
    #[inline]
    pub fn coerce<T: nsIColorPickerShownCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIColorPickerShownCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIColorPickerShownCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIColorPickerShownCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIColorPickerShownCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void update (in AString color); */
    pub update: unsafe extern "C" fn (this: *const nsIColorPickerShownCallback, color: *const nsAString) -> nsresult,

    /* void done (in AString color); */
    pub done: unsafe extern "C" fn (this: *const nsIColorPickerShownCallback, color: *const nsAString) -> nsresult,

}


impl nsIColorPickerShownCallback {
    /* void update (in AString color); */
    #[inline]
    pub unsafe fn update(&self, color: &[u16]) -> Result<(), nsresult> {
        let color = nsString::from(color);
        match ((*self.vtable).update)(self as *const _, &*color) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void done (in AString color); */
    #[inline]
    pub unsafe fn done(&self, color: &[u16]) -> Result<(), nsresult> {
        let color = nsString::from(color);
        match ((*self.vtable).done)(self as *const _, &*color) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIColorPicker {
    vtable: *const nsIColorPickerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIColorPicker {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xde229d37, 0xa8a6, 0x46f1,
            [0x96, 0x9a, 0x0c, 0x1d, 0xe3, 0x3d, 0x0a, 0xd7])
    }
}

unsafe impl RefCounted for nsIColorPicker {
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
pub trait nsIColorPickerCoerce {
    fn coerce_from(v: &nsIColorPicker) -> &Self;
}

impl nsIColorPickerCoerce for nsIColorPicker {
    #[inline]
    fn coerce_from(v: &nsIColorPicker) -> &Self {
        v
    }
}

impl nsIColorPicker {
    #[inline]
    pub fn coerce<T: nsIColorPickerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIColorPicker {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIColorPickerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIColorPicker) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIColorPickerVTable {
    pub __base: nsISupportsVTable,

    /* void init (in mozIDOMWindowProxy parent, in AString title, in AString initialColor); */
    pub init: unsafe extern "C" fn (this: *const nsIColorPicker, parent: *const mozIDOMWindowProxy, title: *const nsAString, initialColor: *const nsAString) -> nsresult,

    /* void open (in nsIColorPickerShownCallback aColorPickerShownCallback); */
    pub open: unsafe extern "C" fn (this: *const nsIColorPicker, aColorPickerShownCallback: *const nsIColorPickerShownCallback) -> nsresult,

}


impl nsIColorPicker {
    /* void init (in mozIDOMWindowProxy parent, in AString title, in AString initialColor); */
    #[inline]
    pub unsafe fn init(&self, parent: Option<&mozIDOMWindowProxy>, title: &[u16], initialColor: &[u16]) -> Result<(), nsresult> {
        let title = nsString::from(title);
        let initialColor = nsString::from(initialColor);
        match ((*self.vtable).init)(self as *const _, parent.map_or(::std::ptr::null(), |x| x as *const _), &*title, &*initialColor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void open (in nsIColorPickerShownCallback aColorPickerShownCallback); */
    #[inline]
    pub unsafe fn open(&self, aColorPickerShownCallback: Option<&nsIColorPickerShownCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).open)(self as *const _, aColorPickerShownCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


