//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITreeBoxObject.idl
//


#[repr(C)]
pub struct nsITreeBoxObject {
    vtable: *const nsITreeBoxObjectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITreeBoxObject {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf3da0c5e, 0x51f5, 0x45f0,
            [0xb2, 0xcd, 0x6b, 0xe3, 0xab, 0x68, 0x47, 0xae])
    }
}

unsafe impl RefCounted for nsITreeBoxObject {
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
pub trait nsITreeBoxObjectCoerce {
    fn coerce_from(v: &nsITreeBoxObject) -> &Self;
}

impl nsITreeBoxObjectCoerce for nsITreeBoxObject {
    #[inline]
    fn coerce_from(v: &nsITreeBoxObject) -> &Self {
        v
    }
}

impl nsITreeBoxObject {
    #[inline]
    pub fn coerce<T: nsITreeBoxObjectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITreeBoxObject {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITreeBoxObjectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITreeBoxObject) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITreeBoxObjectVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsITreeColumns columns; */
    pub get_columns: unsafe extern "C" fn (this: *const nsITreeBoxObject, aColumns: *mut *const nsITreeColumns) -> nsresult,

    /* attribute nsITreeView view; */
    pub get_view: unsafe extern "C" fn (this: *const nsITreeBoxObject, aView: *mut *const nsITreeView) -> nsresult,
    pub set_view: unsafe extern "C" fn (this: *const nsITreeBoxObject, aView: *const nsITreeView) -> nsresult,

    /* attribute boolean focused; */
    pub get_focused: unsafe extern "C" fn (this: *const nsITreeBoxObject, aFocused: *mut bool) -> nsresult,
    pub set_focused: unsafe extern "C" fn (this: *const nsITreeBoxObject, aFocused: bool) -> nsresult,

    /* readonly attribute nsIDOMElement treeBody; */
    pub get_treeBody: unsafe extern "C" fn (this: *const nsITreeBoxObject, aTreeBody: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute long rowHeight; */
    pub get_rowHeight: unsafe extern "C" fn (this: *const nsITreeBoxObject, aRowHeight: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long rowWidth; */
    pub get_rowWidth: unsafe extern "C" fn (this: *const nsITreeBoxObject, aRowWidth: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long horizontalPosition; */
    pub get_horizontalPosition: unsafe extern "C" fn (this: *const nsITreeBoxObject, aHorizontalPosition: *mut libc::int32_t) -> nsresult,

    /* readonly attribute nsIScriptableRegion selectionRegion; */
    pub get_selectionRegion: unsafe extern "C" fn (this: *const nsITreeBoxObject, aSelectionRegion: *mut *const nsIScriptableRegion) -> nsresult,

    /* long getFirstVisibleRow (); */
    pub getFirstVisibleRow: unsafe extern "C" fn (this: *const nsITreeBoxObject, _retval: *mut libc::int32_t) -> nsresult,

    /* long getLastVisibleRow (); */
    pub getLastVisibleRow: unsafe extern "C" fn (this: *const nsITreeBoxObject, _retval: *mut libc::int32_t) -> nsresult,

    /* long getPageLength (); */
    pub getPageLength: unsafe extern "C" fn (this: *const nsITreeBoxObject, _retval: *mut libc::int32_t) -> nsresult,

    /* void ensureRowIsVisible (in long index); */
    pub ensureRowIsVisible: unsafe extern "C" fn (this: *const nsITreeBoxObject, index: libc::int32_t) -> nsresult,

    /* void ensureCellIsVisible (in long row, in nsITreeColumn col); */
    pub ensureCellIsVisible: unsafe extern "C" fn (this: *const nsITreeBoxObject, row: libc::int32_t, col: *const nsITreeColumn) -> nsresult,

    /* void scrollToRow (in long index); */
    pub scrollToRow: unsafe extern "C" fn (this: *const nsITreeBoxObject, index: libc::int32_t) -> nsresult,

    /* void scrollByLines (in long numLines); */
    pub scrollByLines: unsafe extern "C" fn (this: *const nsITreeBoxObject, numLines: libc::int32_t) -> nsresult,

    /* void scrollByPages (in long numPages); */
    pub scrollByPages: unsafe extern "C" fn (this: *const nsITreeBoxObject, numPages: libc::int32_t) -> nsresult,

    /* void scrollToCell (in long row, in nsITreeColumn col); */
    pub scrollToCell: unsafe extern "C" fn (this: *const nsITreeBoxObject, row: libc::int32_t, col: *const nsITreeColumn) -> nsresult,

    /* void scrollToColumn (in nsITreeColumn col); */
    pub scrollToColumn: unsafe extern "C" fn (this: *const nsITreeBoxObject, col: *const nsITreeColumn) -> nsresult,

    /* void scrollToHorizontalPosition (in long horizontalPosition); */
    pub scrollToHorizontalPosition: unsafe extern "C" fn (this: *const nsITreeBoxObject, horizontalPosition: libc::int32_t) -> nsresult,

    /* void invalidate (); */
    pub invalidate: unsafe extern "C" fn (this: *const nsITreeBoxObject) -> nsresult,

    /* void invalidateColumn (in nsITreeColumn col); */
    pub invalidateColumn: unsafe extern "C" fn (this: *const nsITreeBoxObject, col: *const nsITreeColumn) -> nsresult,

    /* void invalidateRow (in long index); */
    pub invalidateRow: unsafe extern "C" fn (this: *const nsITreeBoxObject, index: libc::int32_t) -> nsresult,

    /* void invalidateCell (in long row, in nsITreeColumn col); */
    pub invalidateCell: unsafe extern "C" fn (this: *const nsITreeBoxObject, row: libc::int32_t, col: *const nsITreeColumn) -> nsresult,

    /* void invalidateRange (in long startIndex, in long endIndex); */
    pub invalidateRange: unsafe extern "C" fn (this: *const nsITreeBoxObject, startIndex: libc::int32_t, endIndex: libc::int32_t) -> nsresult,

    /* void invalidateColumnRange (in long startIndex, in long endIndex, in nsITreeColumn col); */
    pub invalidateColumnRange: unsafe extern "C" fn (this: *const nsITreeBoxObject, startIndex: libc::int32_t, endIndex: libc::int32_t, col: *const nsITreeColumn) -> nsresult,

    /* long getRowAt (in long x, in long y); */
    pub getRowAt: unsafe extern "C" fn (this: *const nsITreeBoxObject, x: libc::int32_t, y: libc::int32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* void getCellAt (in long x, in long y, out long row, out nsITreeColumn col, out AString childElt); */
    pub getCellAt: unsafe extern "C" fn (this: *const nsITreeBoxObject, x: libc::int32_t, y: libc::int32_t, row: *mut libc::int32_t, col: *mut *const nsITreeColumn, childElt: *mut nsAString) -> nsresult,

    /* void getCoordsForCellItem (in long row, in nsITreeColumn col, in AString element, out long x, out long y, out long width, out long height); */
    pub getCoordsForCellItem: unsafe extern "C" fn (this: *const nsITreeBoxObject, row: libc::int32_t, col: *const nsITreeColumn, element: *const nsAString, x: *mut libc::int32_t, y: *mut libc::int32_t, width: *mut libc::int32_t, height: *mut libc::int32_t) -> nsresult,

    /* boolean isCellCropped (in long row, in nsITreeColumn col); */
    pub isCellCropped: unsafe extern "C" fn (this: *const nsITreeBoxObject, row: libc::int32_t, col: *const nsITreeColumn, _retval: *mut bool) -> nsresult,

    /* void rowCountChanged (in long index, in long count); */
    pub rowCountChanged: unsafe extern "C" fn (this: *const nsITreeBoxObject, index: libc::int32_t, count: libc::int32_t) -> nsresult,

    /* void beginUpdateBatch (); */
    pub beginUpdateBatch: unsafe extern "C" fn (this: *const nsITreeBoxObject) -> nsresult,

    /* void endUpdateBatch (); */
    pub endUpdateBatch: unsafe extern "C" fn (this: *const nsITreeBoxObject) -> nsresult,

    /* void clearStyleAndImageCaches (); */
    pub clearStyleAndImageCaches: unsafe extern "C" fn (this: *const nsITreeBoxObject) -> nsresult,

    /* void removeImageCacheEntry (in long row, in nsITreeColumn col); */
    pub removeImageCacheEntry: unsafe extern "C" fn (this: *const nsITreeBoxObject, row: libc::int32_t, col: *const nsITreeColumn) -> nsresult,

}


impl nsITreeBoxObject {
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

    /* attribute boolean focused; */
    #[inline]
    pub unsafe fn get_focused(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_focused)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_focused(&self, aFocused: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_focused)(self as *const _, aFocused) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMElement treeBody; */
    #[inline]
    pub unsafe fn get_treeBody(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_treeBody)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long rowHeight; */
    #[inline]
    pub unsafe fn get_rowHeight(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_rowHeight)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long rowWidth; */
    #[inline]
    pub unsafe fn get_rowWidth(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_rowWidth)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long horizontalPosition; */
    #[inline]
    pub unsafe fn get_horizontalPosition(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_horizontalPosition)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIScriptableRegion selectionRegion; */
    #[inline]
    pub unsafe fn get_selectionRegion(&self, ) -> Result<Option<RefPtr<nsIScriptableRegion>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_selectionRegion)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* long getFirstVisibleRow (); */
    #[inline]
    pub unsafe fn getFirstVisibleRow(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getFirstVisibleRow)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getLastVisibleRow (); */
    #[inline]
    pub unsafe fn getLastVisibleRow(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getLastVisibleRow)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getPageLength (); */
    #[inline]
    pub unsafe fn getPageLength(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getPageLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void ensureRowIsVisible (in long index); */
    #[inline]
    pub unsafe fn ensureRowIsVisible(&self, index: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).ensureRowIsVisible)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void ensureCellIsVisible (in long row, in nsITreeColumn col); */
    #[inline]
    pub unsafe fn ensureCellIsVisible(&self, row: libc::int32_t, col: Option<&nsITreeColumn>) -> Result<(), nsresult> {

        match ((*self.vtable).ensureCellIsVisible)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollToRow (in long index); */
    #[inline]
    pub unsafe fn scrollToRow(&self, index: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).scrollToRow)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollByLines (in long numLines); */
    #[inline]
    pub unsafe fn scrollByLines(&self, numLines: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).scrollByLines)(self as *const _, numLines) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollByPages (in long numPages); */
    #[inline]
    pub unsafe fn scrollByPages(&self, numPages: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).scrollByPages)(self as *const _, numPages) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollToCell (in long row, in nsITreeColumn col); */
    #[inline]
    pub unsafe fn scrollToCell(&self, row: libc::int32_t, col: Option<&nsITreeColumn>) -> Result<(), nsresult> {

        match ((*self.vtable).scrollToCell)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollToColumn (in nsITreeColumn col); */
    #[inline]
    pub unsafe fn scrollToColumn(&self, col: Option<&nsITreeColumn>) -> Result<(), nsresult> {

        match ((*self.vtable).scrollToColumn)(self as *const _, col.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollToHorizontalPosition (in long horizontalPosition); */
    #[inline]
    pub unsafe fn scrollToHorizontalPosition(&self, horizontalPosition: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).scrollToHorizontalPosition)(self as *const _, horizontalPosition) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void invalidate (); */
    #[inline]
    pub unsafe fn invalidate(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).invalidate)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void invalidateColumn (in nsITreeColumn col); */
    #[inline]
    pub unsafe fn invalidateColumn(&self, col: Option<&nsITreeColumn>) -> Result<(), nsresult> {

        match ((*self.vtable).invalidateColumn)(self as *const _, col.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void invalidateRow (in long index); */
    #[inline]
    pub unsafe fn invalidateRow(&self, index: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).invalidateRow)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void invalidateCell (in long row, in nsITreeColumn col); */
    #[inline]
    pub unsafe fn invalidateCell(&self, row: libc::int32_t, col: Option<&nsITreeColumn>) -> Result<(), nsresult> {

        match ((*self.vtable).invalidateCell)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void invalidateRange (in long startIndex, in long endIndex); */
    #[inline]
    pub unsafe fn invalidateRange(&self, startIndex: libc::int32_t, endIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).invalidateRange)(self as *const _, startIndex, endIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void invalidateColumnRange (in long startIndex, in long endIndex, in nsITreeColumn col); */
    #[inline]
    pub unsafe fn invalidateColumnRange(&self, startIndex: libc::int32_t, endIndex: libc::int32_t, col: Option<&nsITreeColumn>) -> Result<(), nsresult> {

        match ((*self.vtable).invalidateColumnRange)(self as *const _, startIndex, endIndex, col.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* long getRowAt (in long x, in long y); */
    #[inline]
    pub unsafe fn getRowAt(&self, x: libc::int32_t, y: libc::int32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getRowAt)(self as *const _, x, y, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getCellAt (in long x, in long y, out long row, out nsITreeColumn col, out AString childElt); */
    #[inline]
    pub unsafe fn getCellAt(&self, x: libc::int32_t, y: libc::int32_t) -> Result<(libc::int32_t, Option<RefPtr<nsITreeColumn>>, nsString), nsresult> {
        let mut row: libc::int32_t = ::std::mem::zeroed();
        let mut col = GetterAddrefs::new();
        let mut childElt = nsString::new();
        match ((*self.vtable).getCellAt)(self as *const _, x, y, &mut row as *mut _, col.ptr(), &mut *childElt) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((row, col.refptr(), childElt))
    }

    /* void getCoordsForCellItem (in long row, in nsITreeColumn col, in AString element, out long x, out long y, out long width, out long height); */
    #[inline]
    pub unsafe fn getCoordsForCellItem(&self, row: libc::int32_t, col: Option<&nsITreeColumn>, element: &[u16]) -> Result<(libc::int32_t, libc::int32_t, libc::int32_t, libc::int32_t), nsresult> {
        let element = nsString::from(element);
        let mut x: libc::int32_t = ::std::mem::zeroed();
        let mut y: libc::int32_t = ::std::mem::zeroed();
        let mut width: libc::int32_t = ::std::mem::zeroed();
        let mut height: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getCoordsForCellItem)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _), &*element, &mut x as *mut _, &mut y as *mut _, &mut width as *mut _, &mut height as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((x, y, width, height))
    }

    /* boolean isCellCropped (in long row, in nsITreeColumn col); */
    #[inline]
    pub unsafe fn isCellCropped(&self, row: libc::int32_t, col: Option<&nsITreeColumn>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isCellCropped)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void rowCountChanged (in long index, in long count); */
    #[inline]
    pub unsafe fn rowCountChanged(&self, index: libc::int32_t, count: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).rowCountChanged)(self as *const _, index, count) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void beginUpdateBatch (); */
    #[inline]
    pub unsafe fn beginUpdateBatch(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).beginUpdateBatch)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endUpdateBatch (); */
    #[inline]
    pub unsafe fn endUpdateBatch(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).endUpdateBatch)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clearStyleAndImageCaches (); */
    #[inline]
    pub unsafe fn clearStyleAndImageCaches(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearStyleAndImageCaches)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeImageCacheEntry (in long row, in nsITreeColumn col); */
    #[inline]
    pub unsafe fn removeImageCacheEntry(&self, row: libc::int32_t, col: Option<&nsITreeColumn>) -> Result<(), nsresult> {

        match ((*self.vtable).removeImageCacheEntry)(self as *const _, row, col.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


