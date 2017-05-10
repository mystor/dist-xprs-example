//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISystemStatusBar.idl
//


#[repr(C)]
pub struct nsISystemStatusBar {
    vtable: *const nsISystemStatusBarVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISystemStatusBar {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x24493180, 0xee81, 0x4b7c,
            [0x8b, 0x17, 0x9e, 0x69, 0x48, 0x0b, 0x7b, 0x8a])
    }
}

unsafe impl RefCounted for nsISystemStatusBar {
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
pub trait nsISystemStatusBarCoerce {
    fn coerce_from(v: &nsISystemStatusBar) -> &Self;
}

impl nsISystemStatusBarCoerce for nsISystemStatusBar {
    #[inline]
    fn coerce_from(v: &nsISystemStatusBar) -> &Self {
        v
    }
}

impl nsISystemStatusBar {
    #[inline]
    pub fn coerce<T: nsISystemStatusBarCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISystemStatusBar {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISystemStatusBarCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISystemStatusBar) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISystemStatusBarVTable {
    pub __base: nsISupportsVTable,

    /* void addItem (in nsIDOMElement aDOMMenuElement); */
    pub addItem: unsafe extern "C" fn (this: *const nsISystemStatusBar, aDOMMenuElement: *const nsIDOMElement) -> nsresult,

    /* void removeItem (in nsIDOMElement aDOMMenuElement); */
    pub removeItem: unsafe extern "C" fn (this: *const nsISystemStatusBar, aDOMMenuElement: *const nsIDOMElement) -> nsresult,

}


impl nsISystemStatusBar {
    /* void addItem (in nsIDOMElement aDOMMenuElement); */
    #[inline]
    pub unsafe fn addItem(&self, aDOMMenuElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).addItem)(self as *const _, aDOMMenuElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeItem (in nsIDOMElement aDOMMenuElement); */
    #[inline]
    pub unsafe fn removeItem(&self, aDOMMenuElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).removeItem)(self as *const _, aDOMMenuElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


