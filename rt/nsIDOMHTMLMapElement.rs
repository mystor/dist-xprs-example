//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLMapElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLMapElement {
    vtable: *const nsIDOMHTMLMapElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLMapElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3f49f8c6, 0x2e9d, 0x4323,
            [0xb3, 0x0c, 0x24, 0x04, 0xd5, 0xff, 0x1f, 0x57])
    }
}

unsafe impl RefCounted for nsIDOMHTMLMapElement {
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
pub trait nsIDOMHTMLMapElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLMapElement) -> &Self;
}

impl nsIDOMHTMLMapElementCoerce for nsIDOMHTMLMapElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLMapElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLMapElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLMapElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLMapElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLMapElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLMapElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLMapElementVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMHTMLCollection areas; */
    pub get_areas: unsafe extern "C" fn (this: *const nsIDOMHTMLMapElement, aAreas: *mut *const nsIDOMHTMLCollection) -> nsresult,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMHTMLMapElement, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMHTMLMapElement, aName: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLMapElement {
    /* readonly attribute nsIDOMHTMLCollection areas; */
    #[inline]
    pub unsafe fn get_areas(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLCollection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_areas)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute DOMString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_name(&self, aName: &[u16]) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).set_name)(self as *const _, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


