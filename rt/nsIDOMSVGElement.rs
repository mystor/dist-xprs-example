//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMSVGElement.idl
//


#[repr(C)]
pub struct nsIDOMSVGElement {
    vtable: *const nsIDOMSVGElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMSVGElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc63517c5, 0x8bab, 0x4cd1,
            [0x86, 0x94, 0xbc, 0xca, 0xfc, 0x32, 0xa1, 0x95])
    }
}

unsafe impl RefCounted for nsIDOMSVGElement {
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
pub trait nsIDOMSVGElementCoerce {
    fn coerce_from(v: &nsIDOMSVGElement) -> &Self;
}

impl nsIDOMSVGElementCoerce for nsIDOMSVGElement {
    #[inline]
    fn coerce_from(v: &nsIDOMSVGElement) -> &Self {
        v
    }
}

impl nsIDOMSVGElement {
    #[inline]
    pub fn coerce<T: nsIDOMSVGElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMSVGElement {
    type Target = nsIDOMElement;
    #[inline]
    fn deref(&self) -> &nsIDOMElement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMElementCoerce> nsIDOMSVGElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMSVGElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMSVGElementVTable {
    pub __base: nsIDOMElementVTable,

    /* readonly attribute nsIDOMSVGElement ownerSVGElement; */
    pub get_ownerSVGElement: unsafe extern "C" fn (this: *const nsIDOMSVGElement, aOwnerSVGElement: *mut *const nsIDOMSVGElement) -> nsresult,

    /* readonly attribute nsIDOMSVGElement viewportElement; */
    pub get_viewportElement: unsafe extern "C" fn (this: *const nsIDOMSVGElement, aViewportElement: *mut *const nsIDOMSVGElement) -> nsresult,

    /* [binaryname(SVGClassName)] readonly attribute nsISupports className; */
    pub get_SVGClassName: unsafe extern "C" fn (this: *const nsIDOMSVGElement, aClassName: *mut *const nsISupports) -> nsresult,

    /* readonly attribute nsIDOMCSSStyleDeclaration style; */
    pub get_style: unsafe extern "C" fn (this: *const nsIDOMSVGElement, aStyle: *mut *const nsIDOMCSSStyleDeclaration) -> nsresult,

}


impl nsIDOMSVGElement {
    /* readonly attribute nsIDOMSVGElement ownerSVGElement; */
    #[inline]
    pub unsafe fn get_ownerSVGElement(&self, ) -> Result<Option<RefPtr<nsIDOMSVGElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_ownerSVGElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMSVGElement viewportElement; */
    #[inline]
    pub unsafe fn get_viewportElement(&self, ) -> Result<Option<RefPtr<nsIDOMSVGElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_viewportElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [binaryname(SVGClassName)] readonly attribute nsISupports className; */
    #[inline]
    pub unsafe fn get_SVGClassName(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_SVGClassName)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMCSSStyleDeclaration style; */
    #[inline]
    pub unsafe fn get_style(&self, ) -> Result<Option<RefPtr<nsIDOMCSSStyleDeclaration>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_style)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


