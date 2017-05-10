//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULMultSelectCntrlEl.idl
//


#[repr(C)]
pub struct nsIDOMXULMultiSelectControlElement {
    vtable: *const nsIDOMXULMultiSelectControlElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULMultiSelectControlElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x40654a10, 0x8204, 0x4f06,
            [0x9f, 0x21, 0x7b, 0xaa, 0x31, 0xc7, 0xb1, 0xdd])
    }
}

unsafe impl RefCounted for nsIDOMXULMultiSelectControlElement {
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
pub trait nsIDOMXULMultiSelectControlElementCoerce {
    fn coerce_from(v: &nsIDOMXULMultiSelectControlElement) -> &Self;
}

impl nsIDOMXULMultiSelectControlElementCoerce for nsIDOMXULMultiSelectControlElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULMultiSelectControlElement) -> &Self {
        v
    }
}

impl nsIDOMXULMultiSelectControlElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULMultiSelectControlElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULMultiSelectControlElement {
    type Target = nsIDOMXULSelectControlElement;
    #[inline]
    fn deref(&self) -> &nsIDOMXULSelectControlElement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMXULSelectControlElementCoerce> nsIDOMXULMultiSelectControlElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULMultiSelectControlElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULMultiSelectControlElementVTable {
    pub __base: nsIDOMXULSelectControlElementVTable,

    /* attribute DOMString selType; */
    pub get_selType: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement, aSelType: *mut nsAString) -> nsresult,
    pub set_selType: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement, aSelType: *const nsAString) -> nsresult,

    /* attribute nsIDOMXULSelectControlItemElement currentItem; */
    pub get_currentItem: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement, aCurrentItem: *mut *const nsIDOMXULSelectControlItemElement) -> nsresult,
    pub set_currentItem: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement, aCurrentItem: *const nsIDOMXULSelectControlItemElement) -> nsresult,

    /* attribute long currentIndex; */
    pub get_currentIndex: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement, aCurrentIndex: *mut libc::int32_t) -> nsresult,
    pub set_currentIndex: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement, aCurrentIndex: libc::int32_t) -> nsresult,

    /* readonly attribute nsIDOMNodeList selectedItems; */
    pub get_selectedItems: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement, aSelectedItems: *mut *const nsIDOMNodeList) -> nsresult,

    /* void addItemToSelection (in nsIDOMXULSelectControlItemElement item); */
    pub addItemToSelection: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement, item: *const nsIDOMXULSelectControlItemElement) -> nsresult,

    /* void removeItemFromSelection (in nsIDOMXULSelectControlItemElement item); */
    pub removeItemFromSelection: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement, item: *const nsIDOMXULSelectControlItemElement) -> nsresult,

    /* void toggleItemSelection (in nsIDOMXULSelectControlItemElement item); */
    pub toggleItemSelection: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement, item: *const nsIDOMXULSelectControlItemElement) -> nsresult,

    /* void selectItem (in nsIDOMXULSelectControlItemElement item); */
    pub selectItem: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement, item: *const nsIDOMXULSelectControlItemElement) -> nsresult,

    /* void selectItemRange (in nsIDOMXULSelectControlItemElement startItem, in nsIDOMXULSelectControlItemElement item); */
    pub selectItemRange: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement, startItem: *const nsIDOMXULSelectControlItemElement, item: *const nsIDOMXULSelectControlItemElement) -> nsresult,

    /* void selectAll (); */
    pub selectAll: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement) -> nsresult,

    /* void invertSelection (); */
    pub invertSelection: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement) -> nsresult,

    /* void clearSelection (); */
    pub clearSelection: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement) -> nsresult,

    /* readonly attribute long selectedCount; */
    pub get_selectedCount: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement, aSelectedCount: *mut libc::int32_t) -> nsresult,

    /* [binaryname(MultiGetSelectedItem)] nsIDOMXULSelectControlItemElement getSelectedItem (in long index); */
    pub MultiGetSelectedItem: unsafe extern "C" fn (this: *const nsIDOMXULMultiSelectControlElement, index: libc::int32_t, _retval: *mut *const nsIDOMXULSelectControlItemElement) -> nsresult,

}


impl nsIDOMXULMultiSelectControlElement {
    /* attribute DOMString selType; */
    #[inline]
    pub unsafe fn get_selType(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_selType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_selType(&self, aSelType: &[u16]) -> Result<(), nsresult> {
        let aSelType = nsString::from(aSelType);
        match ((*self.vtable).set_selType)(self as *const _, &*aSelType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIDOMXULSelectControlItemElement currentItem; */
    #[inline]
    pub unsafe fn get_currentItem(&self, ) -> Result<Option<RefPtr<nsIDOMXULSelectControlItemElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_currentItem)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_currentItem(&self, aCurrentItem: Option<&nsIDOMXULSelectControlItemElement>) -> Result<(), nsresult> {

        match ((*self.vtable).set_currentItem)(self as *const _, aCurrentItem.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long currentIndex; */
    #[inline]
    pub unsafe fn get_currentIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_currentIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_currentIndex(&self, aCurrentIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_currentIndex)(self as *const _, aCurrentIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMNodeList selectedItems; */
    #[inline]
    pub unsafe fn get_selectedItems(&self, ) -> Result<Option<RefPtr<nsIDOMNodeList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_selectedItems)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void addItemToSelection (in nsIDOMXULSelectControlItemElement item); */
    #[inline]
    pub unsafe fn addItemToSelection(&self, item: Option<&nsIDOMXULSelectControlItemElement>) -> Result<(), nsresult> {

        match ((*self.vtable).addItemToSelection)(self as *const _, item.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeItemFromSelection (in nsIDOMXULSelectControlItemElement item); */
    #[inline]
    pub unsafe fn removeItemFromSelection(&self, item: Option<&nsIDOMXULSelectControlItemElement>) -> Result<(), nsresult> {

        match ((*self.vtable).removeItemFromSelection)(self as *const _, item.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void toggleItemSelection (in nsIDOMXULSelectControlItemElement item); */
    #[inline]
    pub unsafe fn toggleItemSelection(&self, item: Option<&nsIDOMXULSelectControlItemElement>) -> Result<(), nsresult> {

        match ((*self.vtable).toggleItemSelection)(self as *const _, item.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectItem (in nsIDOMXULSelectControlItemElement item); */
    #[inline]
    pub unsafe fn selectItem(&self, item: Option<&nsIDOMXULSelectControlItemElement>) -> Result<(), nsresult> {

        match ((*self.vtable).selectItem)(self as *const _, item.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectItemRange (in nsIDOMXULSelectControlItemElement startItem, in nsIDOMXULSelectControlItemElement item); */
    #[inline]
    pub unsafe fn selectItemRange(&self, startItem: Option<&nsIDOMXULSelectControlItemElement>, item: Option<&nsIDOMXULSelectControlItemElement>) -> Result<(), nsresult> {

        match ((*self.vtable).selectItemRange)(self as *const _, startItem.map_or(::std::ptr::null(), |x| x as *const _), item.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectAll (); */
    #[inline]
    pub unsafe fn selectAll(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).selectAll)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void invertSelection (); */
    #[inline]
    pub unsafe fn invertSelection(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).invertSelection)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clearSelection (); */
    #[inline]
    pub unsafe fn clearSelection(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearSelection)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute long selectedCount; */
    #[inline]
    pub unsafe fn get_selectedCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_selectedCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [binaryname(MultiGetSelectedItem)] nsIDOMXULSelectControlItemElement getSelectedItem (in long index); */
    #[inline]
    pub unsafe fn MultiGetSelectedItem(&self, index: libc::int32_t) -> Result<Option<RefPtr<nsIDOMXULSelectControlItemElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).MultiGetSelectedItem)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


