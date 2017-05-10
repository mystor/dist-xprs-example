//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleSelectable.idl
//


#[repr(C)]
pub struct nsIAccessibleSelectable {
    vtable: *const nsIAccessibleSelectableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleSelectable {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8efb03d4, 0x1354, 0x4875,
            [0x94, 0xcf, 0x26, 0x13, 0x36, 0x05, 0x76, 0x26])
    }
}

unsafe impl RefCounted for nsIAccessibleSelectable {
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
pub trait nsIAccessibleSelectableCoerce {
    fn coerce_from(v: &nsIAccessibleSelectable) -> &Self;
}

impl nsIAccessibleSelectableCoerce for nsIAccessibleSelectable {
    #[inline]
    fn coerce_from(v: &nsIAccessibleSelectable) -> &Self {
        v
    }
}

impl nsIAccessibleSelectable {
    #[inline]
    pub fn coerce<T: nsIAccessibleSelectableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleSelectable {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleSelectableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleSelectable) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleSelectableVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIArray selectedItems; */
    pub get_selectedItems: unsafe extern "C" fn (this: *const nsIAccessibleSelectable, aSelectedItems: *mut *const nsIArray) -> nsresult,

    /* readonly attribute unsigned long selectedItemCount; */
    pub get_selectedItemCount: unsafe extern "C" fn (this: *const nsIAccessibleSelectable, aSelectedItemCount: *mut libc::uint32_t) -> nsresult,

    /* nsIAccessible getSelectedItemAt (in unsigned long index); */
    pub getSelectedItemAt: unsafe extern "C" fn (this: *const nsIAccessibleSelectable, index: libc::uint32_t, _retval: *mut *const nsIAccessible) -> nsresult,

    /* boolean isItemSelected (in unsigned long index); */
    pub isItemSelected: unsafe extern "C" fn (this: *const nsIAccessibleSelectable, index: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* void addItemToSelection (in unsigned long index); */
    pub addItemToSelection: unsafe extern "C" fn (this: *const nsIAccessibleSelectable, index: libc::uint32_t) -> nsresult,

    /* void removeItemFromSelection (in unsigned long index); */
    pub removeItemFromSelection: unsafe extern "C" fn (this: *const nsIAccessibleSelectable, index: libc::uint32_t) -> nsresult,

    /* boolean selectAll (); */
    pub selectAll: unsafe extern "C" fn (this: *const nsIAccessibleSelectable, _retval: *mut bool) -> nsresult,

    /* void unselectAll (); */
    pub unselectAll: unsafe extern "C" fn (this: *const nsIAccessibleSelectable) -> nsresult,

}


impl nsIAccessibleSelectable {
    /* readonly attribute nsIArray selectedItems; */
    #[inline]
    pub unsafe fn get_selectedItems(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_selectedItems)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute unsigned long selectedItemCount; */
    #[inline]
    pub unsafe fn get_selectedItemCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_selectedItemCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIAccessible getSelectedItemAt (in unsigned long index); */
    #[inline]
    pub unsafe fn getSelectedItemAt(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSelectedItemAt)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean isItemSelected (in unsigned long index); */
    #[inline]
    pub unsafe fn isItemSelected(&self, index: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isItemSelected)(self as *const _, index, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addItemToSelection (in unsigned long index); */
    #[inline]
    pub unsafe fn addItemToSelection(&self, index: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).addItemToSelection)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeItemFromSelection (in unsigned long index); */
    #[inline]
    pub unsafe fn removeItemFromSelection(&self, index: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeItemFromSelection)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean selectAll (); */
    #[inline]
    pub unsafe fn selectAll(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).selectAll)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void unselectAll (); */
    #[inline]
    pub unsafe fn unselectAll(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).unselectAll)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


