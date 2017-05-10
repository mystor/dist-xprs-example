//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULSelectCntrlEl.idl
//


#[repr(C)]
pub struct nsIDOMXULSelectControlElement {
    vtable: *const nsIDOMXULSelectControlElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULSelectControlElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9bf188a7, 0xd6f9, 0x431b,
            [0xb5, 0xc7, 0x11, 0x80, 0x13, 0x99, 0x8e, 0x8b])
    }
}

unsafe impl RefCounted for nsIDOMXULSelectControlElement {
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
pub trait nsIDOMXULSelectControlElementCoerce {
    fn coerce_from(v: &nsIDOMXULSelectControlElement) -> &Self;
}

impl nsIDOMXULSelectControlElementCoerce for nsIDOMXULSelectControlElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULSelectControlElement) -> &Self {
        v
    }
}

impl nsIDOMXULSelectControlElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULSelectControlElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULSelectControlElement {
    type Target = nsIDOMXULControlElement;
    #[inline]
    fn deref(&self) -> &nsIDOMXULControlElement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMXULControlElementCoerce> nsIDOMXULSelectControlElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULSelectControlElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULSelectControlElementVTable {
    pub __base: nsIDOMXULControlElementVTable,

    /* attribute nsIDOMXULSelectControlItemElement selectedItem; */
    pub get_selectedItem: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlElement, aSelectedItem: *mut *const nsIDOMXULSelectControlItemElement) -> nsresult,
    pub set_selectedItem: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlElement, aSelectedItem: *const nsIDOMXULSelectControlItemElement) -> nsresult,

    /* attribute long selectedIndex; */
    pub get_selectedIndex: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlElement, aSelectedIndex: *mut libc::int32_t) -> nsresult,
    pub set_selectedIndex: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlElement, aSelectedIndex: libc::int32_t) -> nsresult,

    /* attribute DOMString value; */
    pub get_value: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlElement, aValue: *mut nsAString) -> nsresult,
    pub set_value: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlElement, aValue: *const nsAString) -> nsresult,

    /* nsIDOMXULSelectControlItemElement appendItem (in DOMString label, in DOMString value); */
    pub appendItem: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlElement, label: *const nsAString, value: *const nsAString, _retval: *mut *const nsIDOMXULSelectControlItemElement) -> nsresult,

    /* nsIDOMXULSelectControlItemElement insertItemAt (in long index, in DOMString label, in DOMString value); */
    pub insertItemAt: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlElement, index: libc::int32_t, label: *const nsAString, value: *const nsAString, _retval: *mut *const nsIDOMXULSelectControlItemElement) -> nsresult,

    /* nsIDOMXULSelectControlItemElement removeItemAt (in long index); */
    pub removeItemAt: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlElement, index: libc::int32_t, _retval: *mut *const nsIDOMXULSelectControlItemElement) -> nsresult,

    /* readonly attribute unsigned long itemCount; */
    pub get_itemCount: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlElement, aItemCount: *mut libc::uint32_t) -> nsresult,

    /* long getIndexOfItem (in nsIDOMXULSelectControlItemElement item); */
    pub getIndexOfItem: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlElement, item: *const nsIDOMXULSelectControlItemElement, _retval: *mut libc::int32_t) -> nsresult,

    /* nsIDOMXULSelectControlItemElement getItemAtIndex (in long index); */
    pub getItemAtIndex: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlElement, index: libc::int32_t, _retval: *mut *const nsIDOMXULSelectControlItemElement) -> nsresult,

}


impl nsIDOMXULSelectControlElement {
    /* attribute nsIDOMXULSelectControlItemElement selectedItem; */
    #[inline]
    pub unsafe fn get_selectedItem(&self, ) -> Result<Option<RefPtr<nsIDOMXULSelectControlItemElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_selectedItem)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_selectedItem(&self, aSelectedItem: Option<&nsIDOMXULSelectControlItemElement>) -> Result<(), nsresult> {

        match ((*self.vtable).set_selectedItem)(self as *const _, aSelectedItem.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long selectedIndex; */
    #[inline]
    pub unsafe fn get_selectedIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_selectedIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_selectedIndex(&self, aSelectedIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_selectedIndex)(self as *const _, aSelectedIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString value; */
    #[inline]
    pub unsafe fn get_value(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_value)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_value(&self, aValue: &[u16]) -> Result<(), nsresult> {
        let aValue = nsString::from(aValue);
        match ((*self.vtable).set_value)(self as *const _, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMXULSelectControlItemElement appendItem (in DOMString label, in DOMString value); */
    #[inline]
    pub unsafe fn appendItem(&self, label: &[u16], value: &[u16]) -> Result<Option<RefPtr<nsIDOMXULSelectControlItemElement>>, nsresult> {
        let label = nsString::from(label);
        let value = nsString::from(value);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).appendItem)(self as *const _, &*label, &*value, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMXULSelectControlItemElement insertItemAt (in long index, in DOMString label, in DOMString value); */
    #[inline]
    pub unsafe fn insertItemAt(&self, index: libc::int32_t, label: &[u16], value: &[u16]) -> Result<Option<RefPtr<nsIDOMXULSelectControlItemElement>>, nsresult> {
        let label = nsString::from(label);
        let value = nsString::from(value);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).insertItemAt)(self as *const _, index, &*label, &*value, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMXULSelectControlItemElement removeItemAt (in long index); */
    #[inline]
    pub unsafe fn removeItemAt(&self, index: libc::int32_t) -> Result<Option<RefPtr<nsIDOMXULSelectControlItemElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).removeItemAt)(self as *const _, index, _retval.ptr()) {
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

    /* long getIndexOfItem (in nsIDOMXULSelectControlItemElement item); */
    #[inline]
    pub unsafe fn getIndexOfItem(&self, item: Option<&nsIDOMXULSelectControlItemElement>) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getIndexOfItem)(self as *const _, item.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMXULSelectControlItemElement getItemAtIndex (in long index); */
    #[inline]
    pub unsafe fn getItemAtIndex(&self, index: libc::int32_t) -> Result<Option<RefPtr<nsIDOMXULSelectControlItemElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getItemAtIndex)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


