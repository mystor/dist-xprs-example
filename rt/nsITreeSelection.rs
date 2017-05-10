//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITreeSelection.idl
//


#[repr(C)]
pub struct nsITreeSelection {
    vtable: *const nsITreeSelectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITreeSelection {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xab6fe746, 0x300b, 0x4ab4,
            [0xab, 0xb9, 0x1c, 0x0e, 0x39, 0x77, 0x87, 0x4c])
    }
}

unsafe impl RefCounted for nsITreeSelection {
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
pub trait nsITreeSelectionCoerce {
    fn coerce_from(v: &nsITreeSelection) -> &Self;
}

impl nsITreeSelectionCoerce for nsITreeSelection {
    #[inline]
    fn coerce_from(v: &nsITreeSelection) -> &Self {
        v
    }
}

impl nsITreeSelection {
    #[inline]
    pub fn coerce<T: nsITreeSelectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITreeSelection {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITreeSelectionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITreeSelection) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITreeSelectionVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsITreeBoxObject tree; */
    pub get_tree: unsafe extern "C" fn (this: *const nsITreeSelection, aTree: *mut *const nsITreeBoxObject) -> nsresult,
    pub set_tree: unsafe extern "C" fn (this: *const nsITreeSelection, aTree: *const nsITreeBoxObject) -> nsresult,

    /* readonly attribute boolean single; */
    pub get_single: unsafe extern "C" fn (this: *const nsITreeSelection, aSingle: *mut bool) -> nsresult,

    /* readonly attribute long count; */
    pub get_count: unsafe extern "C" fn (this: *const nsITreeSelection, aCount: *mut libc::int32_t) -> nsresult,

    /* boolean isSelected (in long index); */
    pub isSelected: unsafe extern "C" fn (this: *const nsITreeSelection, index: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* void select (in long index); */
    pub select: unsafe extern "C" fn (this: *const nsITreeSelection, index: libc::int32_t) -> nsresult,

    /* void timedSelect (in long index, in long delay); */
    pub timedSelect: unsafe extern "C" fn (this: *const nsITreeSelection, index: libc::int32_t, delay: libc::int32_t) -> nsresult,

    /* void toggleSelect (in long index); */
    pub toggleSelect: unsafe extern "C" fn (this: *const nsITreeSelection, index: libc::int32_t) -> nsresult,

    /* void rangedSelect (in long startIndex, in long endIndex, in boolean augment); */
    pub rangedSelect: unsafe extern "C" fn (this: *const nsITreeSelection, startIndex: libc::int32_t, endIndex: libc::int32_t, augment: bool) -> nsresult,

    /* void clearRange (in long startIndex, in long endIndex); */
    pub clearRange: unsafe extern "C" fn (this: *const nsITreeSelection, startIndex: libc::int32_t, endIndex: libc::int32_t) -> nsresult,

    /* void clearSelection (); */
    pub clearSelection: unsafe extern "C" fn (this: *const nsITreeSelection) -> nsresult,

    /* void invertSelection (); */
    pub invertSelection: unsafe extern "C" fn (this: *const nsITreeSelection) -> nsresult,

    /* void selectAll (); */
    pub selectAll: unsafe extern "C" fn (this: *const nsITreeSelection) -> nsresult,

    /* long getRangeCount (); */
    pub getRangeCount: unsafe extern "C" fn (this: *const nsITreeSelection, _retval: *mut libc::int32_t) -> nsresult,

    /* void getRangeAt (in long i, out long min, out long max); */
    pub getRangeAt: unsafe extern "C" fn (this: *const nsITreeSelection, i: libc::int32_t, min: *mut libc::int32_t, max: *mut libc::int32_t) -> nsresult,

    /* void invalidateSelection (); */
    pub invalidateSelection: unsafe extern "C" fn (this: *const nsITreeSelection) -> nsresult,

    /* void adjustSelection (in long index, in long count); */
    pub adjustSelection: unsafe extern "C" fn (this: *const nsITreeSelection, index: libc::int32_t, count: libc::int32_t) -> nsresult,

    /* attribute boolean selectEventsSuppressed; */
    pub get_selectEventsSuppressed: unsafe extern "C" fn (this: *const nsITreeSelection, aSelectEventsSuppressed: *mut bool) -> nsresult,
    pub set_selectEventsSuppressed: unsafe extern "C" fn (this: *const nsITreeSelection, aSelectEventsSuppressed: bool) -> nsresult,

    /* attribute long currentIndex; */
    pub get_currentIndex: unsafe extern "C" fn (this: *const nsITreeSelection, aCurrentIndex: *mut libc::int32_t) -> nsresult,
    pub set_currentIndex: unsafe extern "C" fn (this: *const nsITreeSelection, aCurrentIndex: libc::int32_t) -> nsresult,

    /* attribute nsITreeColumn currentColumn; */
    pub get_currentColumn: unsafe extern "C" fn (this: *const nsITreeSelection, aCurrentColumn: *mut *const nsITreeColumn) -> nsresult,
    pub set_currentColumn: unsafe extern "C" fn (this: *const nsITreeSelection, aCurrentColumn: *const nsITreeColumn) -> nsresult,

    /* readonly attribute long shiftSelectPivot; */
    pub get_shiftSelectPivot: unsafe extern "C" fn (this: *const nsITreeSelection, aShiftSelectPivot: *mut libc::int32_t) -> nsresult,

}


impl nsITreeSelection {
    /* attribute nsITreeBoxObject tree; */
    #[inline]
    pub unsafe fn get_tree(&self, ) -> Result<Option<RefPtr<nsITreeBoxObject>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_tree)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_tree(&self, aTree: Option<&nsITreeBoxObject>) -> Result<(), nsresult> {

        match ((*self.vtable).set_tree)(self as *const _, aTree.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean single; */
    #[inline]
    pub unsafe fn get_single(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_single)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long count; */
    #[inline]
    pub unsafe fn get_count(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_count)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isSelected (in long index); */
    #[inline]
    pub unsafe fn isSelected(&self, index: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isSelected)(self as *const _, index, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void select (in long index); */
    #[inline]
    pub unsafe fn select(&self, index: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).select)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void timedSelect (in long index, in long delay); */
    #[inline]
    pub unsafe fn timedSelect(&self, index: libc::int32_t, delay: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).timedSelect)(self as *const _, index, delay) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void toggleSelect (in long index); */
    #[inline]
    pub unsafe fn toggleSelect(&self, index: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).toggleSelect)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void rangedSelect (in long startIndex, in long endIndex, in boolean augment); */
    #[inline]
    pub unsafe fn rangedSelect(&self, startIndex: libc::int32_t, endIndex: libc::int32_t, augment: bool) -> Result<(), nsresult> {

        match ((*self.vtable).rangedSelect)(self as *const _, startIndex, endIndex, augment) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clearRange (in long startIndex, in long endIndex); */
    #[inline]
    pub unsafe fn clearRange(&self, startIndex: libc::int32_t, endIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).clearRange)(self as *const _, startIndex, endIndex) {
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

    /* void invertSelection (); */
    #[inline]
    pub unsafe fn invertSelection(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).invertSelection)(self as *const _, ) {
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

    /* long getRangeCount (); */
    #[inline]
    pub unsafe fn getRangeCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getRangeCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getRangeAt (in long i, out long min, out long max); */
    #[inline]
    pub unsafe fn getRangeAt(&self, i: libc::int32_t) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut min: libc::int32_t = ::std::mem::zeroed();
        let mut max: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getRangeAt)(self as *const _, i, &mut min as *mut _, &mut max as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((min, max))
    }

    /* void invalidateSelection (); */
    #[inline]
    pub unsafe fn invalidateSelection(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).invalidateSelection)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void adjustSelection (in long index, in long count); */
    #[inline]
    pub unsafe fn adjustSelection(&self, index: libc::int32_t, count: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).adjustSelection)(self as *const _, index, count) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean selectEventsSuppressed; */
    #[inline]
    pub unsafe fn get_selectEventsSuppressed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_selectEventsSuppressed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_selectEventsSuppressed(&self, aSelectEventsSuppressed: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_selectEventsSuppressed)(self as *const _, aSelectEventsSuppressed) {
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

    /* attribute nsITreeColumn currentColumn; */
    #[inline]
    pub unsafe fn get_currentColumn(&self, ) -> Result<Option<RefPtr<nsITreeColumn>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_currentColumn)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_currentColumn(&self, aCurrentColumn: Option<&nsITreeColumn>) -> Result<(), nsresult> {

        match ((*self.vtable).set_currentColumn)(self as *const _, aCurrentColumn.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute long shiftSelectPivot; */
    #[inline]
    pub unsafe fn get_shiftSelectPivot(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_shiftSelectPivot)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsINativeTreeSelection {
    vtable: *const nsINativeTreeSelectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINativeTreeSelection {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1bd59678, 0x5cb3, 0x4316,
            [0xb2, 0x46, 0x31, 0xa9, 0x1b, 0x19, 0xaa, 0xbe])
    }
}

unsafe impl RefCounted for nsINativeTreeSelection {
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
pub trait nsINativeTreeSelectionCoerce {
    fn coerce_from(v: &nsINativeTreeSelection) -> &Self;
}

impl nsINativeTreeSelectionCoerce for nsINativeTreeSelection {
    #[inline]
    fn coerce_from(v: &nsINativeTreeSelection) -> &Self {
        v
    }
}

impl nsINativeTreeSelection {
    #[inline]
    pub fn coerce<T: nsINativeTreeSelectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINativeTreeSelection {
    type Target = nsITreeSelection;
    #[inline]
    fn deref(&self) -> &nsITreeSelection {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsITreeSelectionCoerce> nsINativeTreeSelectionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeTreeSelection) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINativeTreeSelectionVTable {
    pub __base: nsITreeSelectionVTable,

    /* [noscript] void ensureNative (); */
    pub ensureNative: unsafe extern "C" fn (this: *const nsINativeTreeSelection) -> nsresult,

}


impl nsINativeTreeSelection {
    /* [noscript] void ensureNative (); */
    #[inline]
    pub unsafe fn ensureNative(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).ensureNative)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


