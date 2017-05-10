//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITreeView.idl
//


pub mod nsITreeView_consts {
    pub const DROP_BEFORE: i64 = -1;
    pub const DROP_ON: i64 = 0;
    pub const DROP_AFTER: i64 = 1;
    pub const PROGRESS_NORMAL: i64 = 1;
    pub const PROGRESS_UNDETERMINED: i64 = 2;
    pub const PROGRESS_NONE: i64 = 3;
}


#[repr(C)]
pub struct nsITreeView {
    vtable: *const nsITreeViewVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITreeView {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x091116f0, 0x0bdc, 0x4b32,
            [0xb9, 0xc8, 0xc8, 0xd5, 0xa3, 0x7c, 0xb0, 0x88])
    }
}

unsafe impl RefCounted for nsITreeView {
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
pub trait nsITreeViewCoerce {
    fn coerce_from(v: &nsITreeView) -> &Self;
}

impl nsITreeViewCoerce for nsITreeView {
    #[inline]
    fn coerce_from(v: &nsITreeView) -> &Self {
        v
    }
}

impl nsITreeView {
    #[inline]
    pub fn coerce<T: nsITreeViewCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITreeView {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITreeViewCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITreeView) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITreeViewVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long rowCount; */
    pub get_rowCount: unsafe extern "C" fn (this: *const nsITreeView, aRowCount: *mut libc::int32_t) -> nsresult,

    /* attribute nsITreeSelection selection; */
    pub get_selection: unsafe extern "C" fn (this: *const nsITreeView, aSelection: *mut *const nsITreeSelection) -> nsresult,
    pub set_selection: unsafe extern "C" fn (this: *const nsITreeView, aSelection: *const nsITreeSelection) -> nsresult,

    /* AString getRowProperties (in long index); */
    pub getRowProperties: unsafe extern "C" fn (this: *const nsITreeView, index: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getCellProperties (in long row, in nsITreeColumn col); */
    pub getCellProperties: unsafe extern "C" fn (this: *const nsITreeView, row: libc::int32_t, col: *const nsITreeColumn, _retval: *mut nsAString) -> nsresult,

    /* AString getColumnProperties (in nsITreeColumn col); */
    pub getColumnProperties: unsafe extern "C" fn (this: *const nsITreeView, col: *const nsITreeColumn, _retval: *mut nsAString) -> nsresult,

    /* boolean isContainer (in long index); */
    pub isContainer: unsafe extern "C" fn (this: *const nsITreeView, index: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* boolean isContainerOpen (in long index); */
    pub isContainerOpen: unsafe extern "C" fn (this: *const nsITreeView, index: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* boolean isContainerEmpty (in long index); */
    pub isContainerEmpty: unsafe extern "C" fn (this: *const nsITreeView, index: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* boolean isSeparator (in long index); */
    pub isSeparator: unsafe extern "C" fn (this: *const nsITreeView, index: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* boolean isSorted (); */
    pub isSorted: unsafe extern "C" fn (this: *const nsITreeView, _retval: *mut bool) -> nsresult,

    /* boolean canDrop (in long index, in long orientation, in nsIDOMDataTransfer dataTransfer); */
    pub canDrop: unsafe extern "C" fn (this: *const nsITreeView, index: libc::int32_t, orientation: libc::int32_t, dataTransfer: *const nsIDOMDataTransfer, _retval: *mut bool) -> nsresult,

    /* void drop (in long row, in long orientation, in nsIDOMDataTransfer dataTransfer); */
    pub drop: unsafe extern "C" fn (this: *const nsITreeView, row: libc::int32_t, orientation: libc::int32_t, dataTransfer: *const nsIDOMDataTransfer) -> nsresult,

    /* long getParentIndex (in long rowIndex); */
    pub getParentIndex: unsafe extern "C" fn (this: *const nsITreeView, rowIndex: libc::int32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* boolean hasNextSibling (in long rowIndex, in long afterIndex); */
    pub hasNextSibling: unsafe extern "C" fn (this: *const nsITreeView, rowIndex: libc::int32_t, afterIndex: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* long getLevel (in long index); */
    pub getLevel: unsafe extern "C" fn (this: *const nsITreeView, index: libc::int32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* AString getImageSrc (in long row, in nsITreeColumn col); */
    pub getImageSrc: unsafe extern "C" fn (this: *const nsITreeView, row: libc::int32_t, col: *const nsITreeColumn, _retval: *mut nsAString) -> nsresult,

    /* long getProgressMode (in long row, in nsITreeColumn col); */
    pub getProgressMode: unsafe extern "C" fn (this: *const nsITreeView, row: libc::int32_t, col: *const nsITreeColumn, _retval: *mut libc::int32_t) -> nsresult,

    /* AString getCellValue (in long row, in nsITreeColumn col); */
    pub getCellValue: unsafe extern "C" fn (this: *const nsITreeView, row: libc::int32_t, col: *const nsITreeColumn, _retval: *mut nsAString) -> nsresult,

    /* AString getCellText (in long row, in nsITreeColumn col); */
    pub getCellText: unsafe extern "C" fn (this: *const nsITreeView, row: libc::int32_t, col: *const nsITreeColumn, _retval: *mut nsAString) -> nsresult,

    /* void setTree (in nsITreeBoxObject tree); */
    pub setTree: unsafe extern "C" fn (this: *const nsITreeView, tree: *const nsITreeBoxObject) -> nsresult,

    /* void toggleOpenState (in long index); */
    pub toggleOpenState: unsafe extern "C" fn (this: *const nsITreeView, index: libc::int32_t) -> nsresult,

    /* void cycleHeader (in nsITreeColumn col); */
    pub cycleHeader: unsafe extern "C" fn (this: *const nsITreeView, col: *const nsITreeColumn) -> nsresult,

    /* void selectionChanged (); */
    pub selectionChanged: unsafe extern "C" fn (this: *const nsITreeView) -> nsresult,

    /* void cycleCell (in long row, in nsITreeColumn col); */
    pub cycleCell: unsafe extern "C" fn (this: *const nsITreeView, row: libc::int32_t, col: *const nsITreeColumn) -> nsresult,

    /* boolean isEditable (in long row, in nsITreeColumn col); */
    pub isEditable: unsafe extern "C" fn (this: *const nsITreeView, row: libc::int32_t, col: *const nsITreeColumn, _retval: *mut bool) -> nsresult,

    /* boolean isSelectable (in long row, in nsITreeColumn col); */
    pub isSelectable: unsafe extern "C" fn (this: *const nsITreeView, row: libc::int32_t, col: *const nsITreeColumn, _retval: *mut bool) -> nsresult,

    /* void setCellValue (in long row, in nsITreeColumn col, in AString value); */
    pub setCellValue: unsafe extern "C" fn (this: *const nsITreeView, row: libc::int32_t, col: *const nsITreeColumn, value: *const nsAString) -> nsresult,

    /* void setCellText (in long row, in nsITreeColumn col, in AString value); */
    pub setCellText: unsafe extern "C" fn (this: *const nsITreeView, row: libc::int32_t, col: *const nsITreeColumn, value: *const nsAString) -> nsresult,

    /* void performAction (in wstring action); */
    pub performAction: unsafe extern "C" fn (this: *const nsITreeView, action: *const libc::int16_t) -> nsresult,

    /* void performActionOnRow (in wstring action, in long row); */
    pub performActionOnRow: unsafe extern "C" fn (this: *const nsITreeView, action: *const libc::int16_t, row: libc::int32_t) -> nsresult,

    /* void performActionOnCell (in wstring action, in long row, in nsITreeColumn col); */
    pub performActionOnCell: unsafe extern "C" fn (this: *const nsITreeView, action: *const libc::int16_t, row: libc::int32_t, col: *const nsITreeColumn) -> nsresult,

}


impl nsITreeView {
    /* readonly attribute long rowCount; */
    #[inline]
    pub unsafe fn get_rowCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_rowCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsITreeSelection selection; */
    #[inline]
    pub unsafe fn get_selection(&self, ) -> Result<Option<RefPtr<nsITreeSelection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_selection)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_selection(&self, aSelection: Option<&nsITreeSelection>) -> Result<(), nsresult> {

        match ((*self.vtable).set_selection)(self as *const _, aSelection.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString getRowProperties (in long index); */
    #[inline]
    pub unsafe fn getRowProperties(&self, index: libc::int32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getRowProperties)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getCellProperties (in long row, in nsITreeColumn col); */
    #[inline]
    pub unsafe fn getCellProperties(&self, row: libc::int32_t, col: Option<&nsITreeColumn>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getCellProperties)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getColumnProperties (in nsITreeColumn col); */
    #[inline]
    pub unsafe fn getColumnProperties(&self, col: Option<&nsITreeColumn>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getColumnProperties)(self as *const _, col.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isContainer (in long index); */
    #[inline]
    pub unsafe fn isContainer(&self, index: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isContainer)(self as *const _, index, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isContainerOpen (in long index); */
    #[inline]
    pub unsafe fn isContainerOpen(&self, index: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isContainerOpen)(self as *const _, index, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isContainerEmpty (in long index); */
    #[inline]
    pub unsafe fn isContainerEmpty(&self, index: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isContainerEmpty)(self as *const _, index, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isSeparator (in long index); */
    #[inline]
    pub unsafe fn isSeparator(&self, index: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isSeparator)(self as *const _, index, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isSorted (); */
    #[inline]
    pub unsafe fn isSorted(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isSorted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean canDrop (in long index, in long orientation, in nsIDOMDataTransfer dataTransfer); */
    #[inline]
    pub unsafe fn canDrop(&self, index: libc::int32_t, orientation: libc::int32_t, dataTransfer: Option<&nsIDOMDataTransfer>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canDrop)(self as *const _, index, orientation, dataTransfer.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void drop (in long row, in long orientation, in nsIDOMDataTransfer dataTransfer); */
    #[inline]
    pub unsafe fn drop(&self, row: libc::int32_t, orientation: libc::int32_t, dataTransfer: Option<&nsIDOMDataTransfer>) -> Result<(), nsresult> {

        match ((*self.vtable).drop)(self as *const _, row, orientation, dataTransfer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* long getParentIndex (in long rowIndex); */
    #[inline]
    pub unsafe fn getParentIndex(&self, rowIndex: libc::int32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getParentIndex)(self as *const _, rowIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean hasNextSibling (in long rowIndex, in long afterIndex); */
    #[inline]
    pub unsafe fn hasNextSibling(&self, rowIndex: libc::int32_t, afterIndex: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasNextSibling)(self as *const _, rowIndex, afterIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getLevel (in long index); */
    #[inline]
    pub unsafe fn getLevel(&self, index: libc::int32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getLevel)(self as *const _, index, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getImageSrc (in long row, in nsITreeColumn col); */
    #[inline]
    pub unsafe fn getImageSrc(&self, row: libc::int32_t, col: Option<&nsITreeColumn>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getImageSrc)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getProgressMode (in long row, in nsITreeColumn col); */
    #[inline]
    pub unsafe fn getProgressMode(&self, row: libc::int32_t, col: Option<&nsITreeColumn>) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getProgressMode)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getCellValue (in long row, in nsITreeColumn col); */
    #[inline]
    pub unsafe fn getCellValue(&self, row: libc::int32_t, col: Option<&nsITreeColumn>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getCellValue)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getCellText (in long row, in nsITreeColumn col); */
    #[inline]
    pub unsafe fn getCellText(&self, row: libc::int32_t, col: Option<&nsITreeColumn>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getCellText)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setTree (in nsITreeBoxObject tree); */
    #[inline]
    pub unsafe fn setTree(&self, tree: Option<&nsITreeBoxObject>) -> Result<(), nsresult> {

        match ((*self.vtable).setTree)(self as *const _, tree.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void toggleOpenState (in long index); */
    #[inline]
    pub unsafe fn toggleOpenState(&self, index: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).toggleOpenState)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cycleHeader (in nsITreeColumn col); */
    #[inline]
    pub unsafe fn cycleHeader(&self, col: Option<&nsITreeColumn>) -> Result<(), nsresult> {

        match ((*self.vtable).cycleHeader)(self as *const _, col.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectionChanged (); */
    #[inline]
    pub unsafe fn selectionChanged(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).selectionChanged)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cycleCell (in long row, in nsITreeColumn col); */
    #[inline]
    pub unsafe fn cycleCell(&self, row: libc::int32_t, col: Option<&nsITreeColumn>) -> Result<(), nsresult> {

        match ((*self.vtable).cycleCell)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isEditable (in long row, in nsITreeColumn col); */
    #[inline]
    pub unsafe fn isEditable(&self, row: libc::int32_t, col: Option<&nsITreeColumn>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isEditable)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isSelectable (in long row, in nsITreeColumn col); */
    #[inline]
    pub unsafe fn isSelectable(&self, row: libc::int32_t, col: Option<&nsITreeColumn>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isSelectable)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setCellValue (in long row, in nsITreeColumn col, in AString value); */
    #[inline]
    pub unsafe fn setCellValue(&self, row: libc::int32_t, col: Option<&nsITreeColumn>, value: &[u16]) -> Result<(), nsresult> {
        let value = nsString::from(value);
        match ((*self.vtable).setCellValue)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _), &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setCellText (in long row, in nsITreeColumn col, in AString value); */
    #[inline]
    pub unsafe fn setCellText(&self, row: libc::int32_t, col: Option<&nsITreeColumn>, value: &[u16]) -> Result<(), nsresult> {
        let value = nsString::from(value);
        match ((*self.vtable).setCellText)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _), &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void performAction (in wstring action); */
    #[inline]
    pub unsafe fn performAction(&self, action: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).performAction)(self as *const _, action) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void performActionOnRow (in wstring action, in long row); */
    #[inline]
    pub unsafe fn performActionOnRow(&self, action: *const libc::int16_t, row: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).performActionOnRow)(self as *const _, action, row) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void performActionOnCell (in wstring action, in long row, in nsITreeColumn col); */
    #[inline]
    pub unsafe fn performActionOnCell(&self, action: *const libc::int16_t, row: libc::int32_t, col: Option<&nsITreeColumn>) -> Result<(), nsresult> {

        match ((*self.vtable).performActionOnCell)(self as *const _, action, row, col.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsINativeTreeView {
    vtable: *const nsINativeTreeViewVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINativeTreeView {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x46c90265, 0x6553, 0x41ae,
            [0x8d, 0x39, 0x70, 0x22, 0xe7, 0xd0, 0x91, 0x45])
    }
}

unsafe impl RefCounted for nsINativeTreeView {
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
pub trait nsINativeTreeViewCoerce {
    fn coerce_from(v: &nsINativeTreeView) -> &Self;
}

impl nsINativeTreeViewCoerce for nsINativeTreeView {
    #[inline]
    fn coerce_from(v: &nsINativeTreeView) -> &Self {
        v
    }
}

impl nsINativeTreeView {
    #[inline]
    pub fn coerce<T: nsINativeTreeViewCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINativeTreeView {
    type Target = nsITreeView;
    #[inline]
    fn deref(&self) -> &nsITreeView {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsITreeViewCoerce> nsINativeTreeViewCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeTreeView) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINativeTreeViewVTable {
    pub __base: nsITreeViewVTable,

    /* [noscript] void ensureNative (); */
    pub ensureNative: unsafe extern "C" fn (this: *const nsINativeTreeView) -> nsresult,

}


impl nsINativeTreeView {
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


