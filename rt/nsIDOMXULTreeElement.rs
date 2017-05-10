//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULTreeElement.idl
//


#[repr(C)]
pub struct nsIDOMXULTreeElement {
    vtable: *const nsIDOMXULTreeElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULTreeElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x013b62af, 0x1e2f, 0x4b07,
            [0x90, 0x91, 0xd7, 0xc0, 0xfc, 0x46, 0x87, 0xe2])
    }
}

unsafe impl RefCounted for nsIDOMXULTreeElement {
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
pub trait nsIDOMXULTreeElementCoerce {
    fn coerce_from(v: &nsIDOMXULTreeElement) -> &Self;
}

impl nsIDOMXULTreeElementCoerce for nsIDOMXULTreeElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULTreeElement) -> &Self {
        v
    }
}

impl nsIDOMXULTreeElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULTreeElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULTreeElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMXULTreeElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULTreeElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULTreeElementVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsITreeColumns columns; */
    pub get_columns: unsafe extern "C" fn (this: *const nsIDOMXULTreeElement, aColumns: *mut *const nsITreeColumns) -> nsresult,

    /* attribute nsITreeView view; */
    pub get_view: unsafe extern "C" fn (this: *const nsIDOMXULTreeElement, aView: *mut *const nsITreeView) -> nsresult,
    pub set_view: unsafe extern "C" fn (this: *const nsIDOMXULTreeElement, aView: *const nsITreeView) -> nsresult,

    /* readonly attribute nsIDOMElement body; */
    pub get_body: unsafe extern "C" fn (this: *const nsIDOMXULTreeElement, aBody: *mut *const nsIDOMElement) -> nsresult,

    /* attribute boolean editable; */
    pub get_editable: unsafe extern "C" fn (this: *const nsIDOMXULTreeElement, aEditable: *mut bool) -> nsresult,
    pub set_editable: unsafe extern "C" fn (this: *const nsIDOMXULTreeElement, aEditable: bool) -> nsresult,

    /* readonly attribute nsIDOMXULTextBoxElement inputField; */
    pub get_inputField: unsafe extern "C" fn (this: *const nsIDOMXULTreeElement, aInputField: *mut *const nsIDOMXULTextBoxElement) -> nsresult,

}


impl nsIDOMXULTreeElement {
    /* readonly attribute nsITreeColumns columns; */
    #[inline]
    pub unsafe fn get_columns(&self, ) -> Result<Option<RefPtr<nsITreeColumns>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_columns)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsITreeView view; */
    #[inline]
    pub unsafe fn get_view(&self, ) -> Result<Option<RefPtr<nsITreeView>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_view)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_view(&self, aView: Option<&nsITreeView>) -> Result<(), nsresult> {

        match ((*self.vtable).set_view)(self as *const _, aView.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMElement body; */
    #[inline]
    pub unsafe fn get_body(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_body)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute boolean editable; */
    #[inline]
    pub unsafe fn get_editable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_editable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_editable(&self, aEditable: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_editable)(self as *const _, aEditable) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMXULTextBoxElement inputField; */
    #[inline]
    pub unsafe fn get_inputField(&self, ) -> Result<Option<RefPtr<nsIDOMXULTextBoxElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_inputField)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


