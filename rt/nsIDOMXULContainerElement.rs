//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULContainerElement.idl
//


#[repr(C)]
pub struct nsIDOMXULContainerItemElement {
    vtable: *const nsIDOMXULContainerItemElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULContainerItemElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x800a68c7, 0xb854, 0x4597,
            [0xa4, 0x36, 0x30, 0x55, 0xce, 0x5c, 0x5c, 0x96])
    }
}

unsafe impl RefCounted for nsIDOMXULContainerItemElement {
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
pub trait nsIDOMXULContainerItemElementCoerce {
    fn coerce_from(v: &nsIDOMXULContainerItemElement) -> &Self;
}

impl nsIDOMXULContainerItemElementCoerce for nsIDOMXULContainerItemElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULContainerItemElement) -> &Self {
        v
    }
}

impl nsIDOMXULContainerItemElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULContainerItemElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULContainerItemElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMXULContainerItemElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULContainerItemElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULContainerItemElementVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMXULContainerElement parentContainer; */
    pub get_parentContainer: unsafe extern "C" fn (this: *const nsIDOMXULContainerItemElement, aParentContainer: *mut *const nsIDOMXULContainerElement) -> nsresult,

}


impl nsIDOMXULContainerItemElement {
    /* readonly attribute nsIDOMXULContainerElement parentContainer; */
    #[inline]
    pub unsafe fn get_parentContainer(&self, ) -> Result<Option<RefPtr<nsIDOMXULContainerElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parentContainer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIDOMXULContainerElement {
    vtable: *const nsIDOMXULContainerElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULContainerElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb2bc96b8, 0x31fc, 0x42f4,
            [0x93, 0x7a, 0xbd, 0x27, 0x29, 0x1a, 0xf4, 0x0b])
    }
}

unsafe impl RefCounted for nsIDOMXULContainerElement {
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
pub trait nsIDOMXULContainerElementCoerce {
    fn coerce_from(v: &nsIDOMXULContainerElement) -> &Self;
}

impl nsIDOMXULContainerElementCoerce for nsIDOMXULContainerElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULContainerElement) -> &Self {
        v
    }
}

impl nsIDOMXULContainerElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULContainerElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULContainerElement {
    type Target = nsIDOMXULContainerItemElement;
    #[inline]
    fn deref(&self) -> &nsIDOMXULContainerItemElement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMXULContainerItemElementCoerce> nsIDOMXULContainerElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULContainerElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULContainerElementVTable {
    pub __base: nsIDOMXULContainerItemElementVTable,

    /* nsIDOMXULElement appendItem (in DOMString aLabel, in DOMString aValue); */
    pub appendItem: unsafe extern "C" fn (this: *const nsIDOMXULContainerElement, aLabel: *const nsAString, aValue: *const nsAString, _retval: *mut *const nsIDOMXULElement) -> nsresult,

    /* nsIDOMXULElement insertItemAt (in long aIndex, in DOMString aLabel, in DOMString aValue); */
    pub insertItemAt: unsafe extern "C" fn (this: *const nsIDOMXULContainerElement, aIndex: libc::int32_t, aLabel: *const nsAString, aValue: *const nsAString, _retval: *mut *const nsIDOMXULElement) -> nsresult,

    /* nsIDOMXULElement removeItemAt (in long aIndex); */
    pub removeItemAt: unsafe extern "C" fn (this: *const nsIDOMXULContainerElement, aIndex: libc::int32_t, _retval: *mut *const nsIDOMXULElement) -> nsresult,

    /* readonly attribute unsigned long itemCount; */
    pub get_itemCount: unsafe extern "C" fn (this: *const nsIDOMXULContainerElement, aItemCount: *mut libc::uint32_t) -> nsresult,

    /* long getIndexOfItem (in nsIDOMXULElement aItem); */
    pub getIndexOfItem: unsafe extern "C" fn (this: *const nsIDOMXULContainerElement, aItem: *const nsIDOMXULElement, _retval: *mut libc::int32_t) -> nsresult,

    /* nsIDOMXULElement getItemAtIndex (in long aIndex); */
    pub getItemAtIndex: unsafe extern "C" fn (this: *const nsIDOMXULContainerElement, aIndex: libc::int32_t, _retval: *mut *const nsIDOMXULElement) -> nsresult,

}


impl nsIDOMXULContainerElement {
    /* nsIDOMXULElement appendItem (in DOMString aLabel, in DOMString aValue); */
    #[inline]
    pub unsafe fn appendItem(&self, aLabel: &[u16], aValue: &[u16]) -> Result<Option<RefPtr<nsIDOMXULElement>>, nsresult> {
        let aLabel = nsString::from(aLabel);
        let aValue = nsString::from(aValue);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).appendItem)(self as *const _, &*aLabel, &*aValue, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMXULElement insertItemAt (in long aIndex, in DOMString aLabel, in DOMString aValue); */
    #[inline]
    pub unsafe fn insertItemAt(&self, aIndex: libc::int32_t, aLabel: &[u16], aValue: &[u16]) -> Result<Option<RefPtr<nsIDOMXULElement>>, nsresult> {
        let aLabel = nsString::from(aLabel);
        let aValue = nsString::from(aValue);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).insertItemAt)(self as *const _, aIndex, &*aLabel, &*aValue, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMXULElement removeItemAt (in long aIndex); */
    #[inline]
    pub unsafe fn removeItemAt(&self, aIndex: libc::int32_t) -> Result<Option<RefPtr<nsIDOMXULElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).removeItemAt)(self as *const _, aIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute unsigned long itemCount; */
    #[inline]
    pub unsafe fn get_itemCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_itemCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getIndexOfItem (in nsIDOMXULElement aItem); */
    #[inline]
    pub unsafe fn getIndexOfItem(&self, aItem: Option<&nsIDOMXULElement>) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getIndexOfItem)(self as *const _, aItem.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMXULElement getItemAtIndex (in long aIndex); */
    #[inline]
    pub unsafe fn getItemAtIndex(&self, aIndex: libc::int32_t) -> Result<Option<RefPtr<nsIDOMXULElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getItemAtIndex)(self as *const _, aIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


