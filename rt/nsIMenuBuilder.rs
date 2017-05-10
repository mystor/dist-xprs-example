//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMenuBuilder.idl
//


#[repr(C)]
pub struct nsIMenuBuilder {
    vtable: *const nsIMenuBuilderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMenuBuilder {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x93f4a48f, 0xd043, 0x4f45,
            [0x97, 0xfd, 0x97, 0x71, 0xea, 0x1a, 0xf9, 0x76])
    }
}

unsafe impl RefCounted for nsIMenuBuilder {
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
pub trait nsIMenuBuilderCoerce {
    fn coerce_from(v: &nsIMenuBuilder) -> &Self;
}

impl nsIMenuBuilderCoerce for nsIMenuBuilder {
    #[inline]
    fn coerce_from(v: &nsIMenuBuilder) -> &Self {
        v
    }
}

impl nsIMenuBuilder {
    #[inline]
    pub fn coerce<T: nsIMenuBuilderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMenuBuilder {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMenuBuilderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMenuBuilder) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMenuBuilderVTable {
    pub __base: nsISupportsVTable,

    /* void openContainer (in DOMString aLabel); */
    pub openContainer: unsafe extern "C" fn (this: *const nsIMenuBuilder, aLabel: *const nsAString) -> nsresult,

    /* void addItemFor (in nsIDOMHTMLMenuItemElement aElement, in boolean aCanLoadIcon); */
    pub addItemFor: unsafe extern "C" fn (this: *const nsIMenuBuilder, aElement: *const nsIDOMHTMLMenuItemElement, aCanLoadIcon: bool) -> nsresult,

    /* void addSeparator (); */
    pub addSeparator: unsafe extern "C" fn (this: *const nsIMenuBuilder) -> nsresult,

    /* void undoAddSeparator (); */
    pub undoAddSeparator: unsafe extern "C" fn (this: *const nsIMenuBuilder) -> nsresult,

    /* void closeContainer (); */
    pub closeContainer: unsafe extern "C" fn (this: *const nsIMenuBuilder) -> nsresult,

    /* AString toJSONString (); */
    pub toJSONString: unsafe extern "C" fn (this: *const nsIMenuBuilder, _retval: *mut nsAString) -> nsresult,

    /* void click (in DOMString aGeneratedItemId); */
    pub click: unsafe extern "C" fn (this: *const nsIMenuBuilder, aGeneratedItemId: *const nsAString) -> nsresult,

}


impl nsIMenuBuilder {
    /* void openContainer (in DOMString aLabel); */
    #[inline]
    pub unsafe fn openContainer(&self, aLabel: &[u16]) -> Result<(), nsresult> {
        let aLabel = nsString::from(aLabel);
        match ((*self.vtable).openContainer)(self as *const _, &*aLabel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addItemFor (in nsIDOMHTMLMenuItemElement aElement, in boolean aCanLoadIcon); */
    #[inline]
    pub unsafe fn addItemFor(&self, aElement: Option<&nsIDOMHTMLMenuItemElement>, aCanLoadIcon: bool) -> Result<(), nsresult> {

        match ((*self.vtable).addItemFor)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), aCanLoadIcon) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addSeparator (); */
    #[inline]
    pub unsafe fn addSeparator(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).addSeparator)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void undoAddSeparator (); */
    #[inline]
    pub unsafe fn undoAddSeparator(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).undoAddSeparator)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void closeContainer (); */
    #[inline]
    pub unsafe fn closeContainer(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).closeContainer)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString toJSONString (); */
    #[inline]
    pub unsafe fn toJSONString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).toJSONString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void click (in DOMString aGeneratedItemId); */
    #[inline]
    pub unsafe fn click(&self, aGeneratedItemId: &[u16]) -> Result<(), nsresult> {
        let aGeneratedItemId = nsString::from(aGeneratedItemId);
        match ((*self.vtable).click)(self as *const _, &*aGeneratedItemId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


