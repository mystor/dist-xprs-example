//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITableEditor.idl
//


pub mod nsITableEditor_consts {
    pub const eNoSearch: i64 = 0;
    pub const ePreviousColumn: i64 = 1;
    pub const ePreviousRow: i64 = 2;
}


#[repr(C)]
pub struct nsITableEditor {
    vtable: *const nsITableEditorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITableEditor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4805e684, 0x49b9, 0x11d3,
            [0x9c, 0xe4, 0xed, 0x60, 0xbd, 0x6c, 0xb5, 0xbc])
    }
}

unsafe impl RefCounted for nsITableEditor {
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
pub trait nsITableEditorCoerce {
    fn coerce_from(v: &nsITableEditor) -> &Self;
}

impl nsITableEditorCoerce for nsITableEditor {
    #[inline]
    fn coerce_from(v: &nsITableEditor) -> &Self {
        v
    }
}

impl nsITableEditor {
    #[inline]
    pub fn coerce<T: nsITableEditorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITableEditor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITableEditorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITableEditor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITableEditorVTable {
    pub __base: nsISupportsVTable,

    /* void insertTableCell (in long aNumber, in boolean aAfter); */
    pub insertTableCell: unsafe extern "C" fn (this: *const nsITableEditor, aNumber: libc::int32_t, aAfter: bool) -> nsresult,

    /* void insertTableColumn (in long aNumber, in boolean aAfter); */
    pub insertTableColumn: unsafe extern "C" fn (this: *const nsITableEditor, aNumber: libc::int32_t, aAfter: bool) -> nsresult,

    /* void insertTableRow (in long aNumber, in boolean aAfter); */
    pub insertTableRow: unsafe extern "C" fn (this: *const nsITableEditor, aNumber: libc::int32_t, aAfter: bool) -> nsresult,

    /* void deleteTable (); */
    pub deleteTable: unsafe extern "C" fn (this: *const nsITableEditor) -> nsresult,

    /* void deleteTableCellContents (); */
    pub deleteTableCellContents: unsafe extern "C" fn (this: *const nsITableEditor) -> nsresult,

    /* void deleteTableCell (in long aNumber); */
    pub deleteTableCell: unsafe extern "C" fn (this: *const nsITableEditor, aNumber: libc::int32_t) -> nsresult,

    /* void deleteTableColumn (in long aNumber); */
    pub deleteTableColumn: unsafe extern "C" fn (this: *const nsITableEditor, aNumber: libc::int32_t) -> nsresult,

    /* void deleteTableRow (in long aNumber); */
    pub deleteTableRow: unsafe extern "C" fn (this: *const nsITableEditor, aNumber: libc::int32_t) -> nsresult,

    /* void selectTableCell (); */
    pub selectTableCell: unsafe extern "C" fn (this: *const nsITableEditor) -> nsresult,

    /* void selectBlockOfCells (in nsIDOMElement aStartCell, in nsIDOMElement aEndCell); */
    pub selectBlockOfCells: unsafe extern "C" fn (this: *const nsITableEditor, aStartCell: *const nsIDOMElement, aEndCell: *const nsIDOMElement) -> nsresult,

    /* void selectTableRow (); */
    pub selectTableRow: unsafe extern "C" fn (this: *const nsITableEditor) -> nsresult,

    /* void selectTableColumn (); */
    pub selectTableColumn: unsafe extern "C" fn (this: *const nsITableEditor) -> nsresult,

    /* void selectTable (); */
    pub selectTable: unsafe extern "C" fn (this: *const nsITableEditor) -> nsresult,

    /* void selectAllTableCells (); */
    pub selectAllTableCells: unsafe extern "C" fn (this: *const nsITableEditor) -> nsresult,

    /* nsIDOMElement switchTableCellHeaderType (in nsIDOMElement aSourceCell); */
    pub switchTableCellHeaderType: unsafe extern "C" fn (this: *const nsITableEditor, aSourceCell: *const nsIDOMElement, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* void joinTableCells (in boolean aMergeNonContiguousContents); */
    pub joinTableCells: unsafe extern "C" fn (this: *const nsITableEditor, aMergeNonContiguousContents: bool) -> nsresult,

    /* void splitTableCell (); */
    pub splitTableCell: unsafe extern "C" fn (this: *const nsITableEditor) -> nsresult,

    /* void normalizeTable (in nsIDOMElement aTable); */
    pub normalizeTable: unsafe extern "C" fn (this: *const nsITableEditor, aTable: *const nsIDOMElement) -> nsresult,

    /* void getCellIndexes (in nsIDOMElement aCell, out long aRowIndex, out long aColIndex); */
    pub getCellIndexes: unsafe extern "C" fn (this: *const nsITableEditor, aCell: *const nsIDOMElement, aRowIndex: *mut libc::int32_t, aColIndex: *mut libc::int32_t) -> nsresult,

    /* void getTableSize (in nsIDOMElement aTable, out long aRowCount, out long aColCount); */
    pub getTableSize: unsafe extern "C" fn (this: *const nsITableEditor, aTable: *const nsIDOMElement, aRowCount: *mut libc::int32_t, aColCount: *mut libc::int32_t) -> nsresult,

    /* nsIDOMElement getCellAt (in nsIDOMElement aTable, in long aRowIndex, in long aColIndex); */
    pub getCellAt: unsafe extern "C" fn (this: *const nsITableEditor, aTable: *const nsIDOMElement, aRowIndex: libc::int32_t, aColIndex: libc::int32_t, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* void getCellDataAt (in nsIDOMElement aTable, in long aRowIndex, in long aColIndex, out nsIDOMElement aCell, out long aStartRowIndex, out long aStartColIndex, out long aRowSpan, out long aColSpan, out long aActualRowSpan, out long aActualColSpan, out boolean aIsSelected); */
    pub getCellDataAt: unsafe extern "C" fn (this: *const nsITableEditor, aTable: *const nsIDOMElement, aRowIndex: libc::int32_t, aColIndex: libc::int32_t, aCell: *mut *const nsIDOMElement, aStartRowIndex: *mut libc::int32_t, aStartColIndex: *mut libc::int32_t, aRowSpan: *mut libc::int32_t, aColSpan: *mut libc::int32_t, aActualRowSpan: *mut libc::int32_t, aActualColSpan: *mut libc::int32_t, aIsSelected: *mut bool) -> nsresult,

    /* nsIDOMNode getFirstRow (in nsIDOMElement aTableElement); */
    pub getFirstRow: unsafe extern "C" fn (this: *const nsITableEditor, aTableElement: *const nsIDOMElement, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode getNextRow (in nsIDOMNode aTableElement); */
    pub getNextRow: unsafe extern "C" fn (this: *const nsITableEditor, aTableElement: *const nsIDOMNode, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* void setSelectionAfterTableEdit (in nsIDOMElement aTable, in long aRow, in long aCol, in long aDirection, in boolean aSelected); */
    pub setSelectionAfterTableEdit: unsafe extern "C" fn (this: *const nsITableEditor, aTable: *const nsIDOMElement, aRow: libc::int32_t, aCol: libc::int32_t, aDirection: libc::int32_t, aSelected: bool) -> nsresult,

    /* nsIDOMElement getSelectedOrParentTableElement (out AString aTagName, out long aCount); */
    pub getSelectedOrParentTableElement: unsafe extern "C" fn (this: *const nsITableEditor, aTagName: *mut nsAString, aCount: *mut libc::int32_t, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* uint32_t getSelectedCellsType (in nsIDOMElement aElement); */
    pub getSelectedCellsType: unsafe extern "C" fn (this: *const nsITableEditor, aElement: *const nsIDOMElement, _retval: *mut uint32_t) -> nsresult,

    /* nsIDOMElement getFirstSelectedCell (out nsIDOMRange aRange); */
    pub getFirstSelectedCell: unsafe extern "C" fn (this: *const nsITableEditor, aRange: *mut *const nsIDOMRange, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* nsIDOMElement getFirstSelectedCellInTable (out long aRowIndex, out long aColIndex); */
    pub getFirstSelectedCellInTable: unsafe extern "C" fn (this: *const nsITableEditor, aRowIndex: *mut libc::int32_t, aColIndex: *mut libc::int32_t, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* nsIDOMElement getNextSelectedCell (out nsIDOMRange aRange); */
    pub getNextSelectedCell: unsafe extern "C" fn (this: *const nsITableEditor, aRange: *mut *const nsIDOMRange, _retval: *mut *const nsIDOMElement) -> nsresult,

}


impl nsITableEditor {
    /* void insertTableCell (in long aNumber, in boolean aAfter); */
    #[inline]
    pub unsafe fn insertTableCell(&self, aNumber: libc::int32_t, aAfter: bool) -> Result<(), nsresult> {

        match ((*self.vtable).insertTableCell)(self as *const _, aNumber, aAfter) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void insertTableColumn (in long aNumber, in boolean aAfter); */
    #[inline]
    pub unsafe fn insertTableColumn(&self, aNumber: libc::int32_t, aAfter: bool) -> Result<(), nsresult> {

        match ((*self.vtable).insertTableColumn)(self as *const _, aNumber, aAfter) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void insertTableRow (in long aNumber, in boolean aAfter); */
    #[inline]
    pub unsafe fn insertTableRow(&self, aNumber: libc::int32_t, aAfter: bool) -> Result<(), nsresult> {

        match ((*self.vtable).insertTableRow)(self as *const _, aNumber, aAfter) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deleteTable (); */
    #[inline]
    pub unsafe fn deleteTable(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).deleteTable)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deleteTableCellContents (); */
    #[inline]
    pub unsafe fn deleteTableCellContents(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).deleteTableCellContents)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deleteTableCell (in long aNumber); */
    #[inline]
    pub unsafe fn deleteTableCell(&self, aNumber: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).deleteTableCell)(self as *const _, aNumber) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deleteTableColumn (in long aNumber); */
    #[inline]
    pub unsafe fn deleteTableColumn(&self, aNumber: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).deleteTableColumn)(self as *const _, aNumber) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deleteTableRow (in long aNumber); */
    #[inline]
    pub unsafe fn deleteTableRow(&self, aNumber: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).deleteTableRow)(self as *const _, aNumber) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectTableCell (); */
    #[inline]
    pub unsafe fn selectTableCell(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).selectTableCell)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectBlockOfCells (in nsIDOMElement aStartCell, in nsIDOMElement aEndCell); */
    #[inline]
    pub unsafe fn selectBlockOfCells(&self, aStartCell: Option<&nsIDOMElement>, aEndCell: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).selectBlockOfCells)(self as *const _, aStartCell.map_or(::std::ptr::null(), |x| x as *const _), aEndCell.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectTableRow (); */
    #[inline]
    pub unsafe fn selectTableRow(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).selectTableRow)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectTableColumn (); */
    #[inline]
    pub unsafe fn selectTableColumn(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).selectTableColumn)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectTable (); */
    #[inline]
    pub unsafe fn selectTable(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).selectTable)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectAllTableCells (); */
    #[inline]
    pub unsafe fn selectAllTableCells(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).selectAllTableCells)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMElement switchTableCellHeaderType (in nsIDOMElement aSourceCell); */
    #[inline]
    pub unsafe fn switchTableCellHeaderType(&self, aSourceCell: Option<&nsIDOMElement>) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).switchTableCellHeaderType)(self as *const _, aSourceCell.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void joinTableCells (in boolean aMergeNonContiguousContents); */
    #[inline]
    pub unsafe fn joinTableCells(&self, aMergeNonContiguousContents: bool) -> Result<(), nsresult> {

        match ((*self.vtable).joinTableCells)(self as *const _, aMergeNonContiguousContents) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void splitTableCell (); */
    #[inline]
    pub unsafe fn splitTableCell(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).splitTableCell)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void normalizeTable (in nsIDOMElement aTable); */
    #[inline]
    pub unsafe fn normalizeTable(&self, aTable: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).normalizeTable)(self as *const _, aTable.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getCellIndexes (in nsIDOMElement aCell, out long aRowIndex, out long aColIndex); */
    #[inline]
    pub unsafe fn getCellIndexes(&self, aCell: Option<&nsIDOMElement>) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut aRowIndex: libc::int32_t = ::std::mem::zeroed();
        let mut aColIndex: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getCellIndexes)(self as *const _, aCell.map_or(::std::ptr::null(), |x| x as *const _), &mut aRowIndex as *mut _, &mut aColIndex as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aRowIndex, aColIndex))
    }

    /* void getTableSize (in nsIDOMElement aTable, out long aRowCount, out long aColCount); */
    #[inline]
    pub unsafe fn getTableSize(&self, aTable: Option<&nsIDOMElement>) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut aRowCount: libc::int32_t = ::std::mem::zeroed();
        let mut aColCount: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getTableSize)(self as *const _, aTable.map_or(::std::ptr::null(), |x| x as *const _), &mut aRowCount as *mut _, &mut aColCount as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aRowCount, aColCount))
    }

    /* nsIDOMElement getCellAt (in nsIDOMElement aTable, in long aRowIndex, in long aColIndex); */
    #[inline]
    pub unsafe fn getCellAt(&self, aTable: Option<&nsIDOMElement>, aRowIndex: libc::int32_t, aColIndex: libc::int32_t) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getCellAt)(self as *const _, aTable.map_or(::std::ptr::null(), |x| x as *const _), aRowIndex, aColIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void getCellDataAt (in nsIDOMElement aTable, in long aRowIndex, in long aColIndex, out nsIDOMElement aCell, out long aStartRowIndex, out long aStartColIndex, out long aRowSpan, out long aColSpan, out long aActualRowSpan, out long aActualColSpan, out boolean aIsSelected); */
    #[inline]
    pub unsafe fn getCellDataAt(&self, aTable: Option<&nsIDOMElement>, aRowIndex: libc::int32_t, aColIndex: libc::int32_t) -> Result<(Option<RefPtr<nsIDOMElement>>, libc::int32_t, libc::int32_t, libc::int32_t, libc::int32_t, libc::int32_t, libc::int32_t, bool), nsresult> {
        let mut aCell = GetterAddrefs::new();
        let mut aStartRowIndex: libc::int32_t = ::std::mem::zeroed();
        let mut aStartColIndex: libc::int32_t = ::std::mem::zeroed();
        let mut aRowSpan: libc::int32_t = ::std::mem::zeroed();
        let mut aColSpan: libc::int32_t = ::std::mem::zeroed();
        let mut aActualRowSpan: libc::int32_t = ::std::mem::zeroed();
        let mut aActualColSpan: libc::int32_t = ::std::mem::zeroed();
        let mut aIsSelected: bool = ::std::mem::zeroed();
        match ((*self.vtable).getCellDataAt)(self as *const _, aTable.map_or(::std::ptr::null(), |x| x as *const _), aRowIndex, aColIndex, aCell.ptr(), &mut aStartRowIndex as *mut _, &mut aStartColIndex as *mut _, &mut aRowSpan as *mut _, &mut aColSpan as *mut _, &mut aActualRowSpan as *mut _, &mut aActualColSpan as *mut _, &mut aIsSelected as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aCell.refptr(), aStartRowIndex, aStartColIndex, aRowSpan, aColSpan, aActualRowSpan, aActualColSpan, aIsSelected))
    }

    /* nsIDOMNode getFirstRow (in nsIDOMElement aTableElement); */
    #[inline]
    pub unsafe fn getFirstRow(&self, aTableElement: Option<&nsIDOMElement>) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFirstRow)(self as *const _, aTableElement.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNode getNextRow (in nsIDOMNode aTableElement); */
    #[inline]
    pub unsafe fn getNextRow(&self, aTableElement: Option<&nsIDOMNode>) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getNextRow)(self as *const _, aTableElement.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setSelectionAfterTableEdit (in nsIDOMElement aTable, in long aRow, in long aCol, in long aDirection, in boolean aSelected); */
    #[inline]
    pub unsafe fn setSelectionAfterTableEdit(&self, aTable: Option<&nsIDOMElement>, aRow: libc::int32_t, aCol: libc::int32_t, aDirection: libc::int32_t, aSelected: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setSelectionAfterTableEdit)(self as *const _, aTable.map_or(::std::ptr::null(), |x| x as *const _), aRow, aCol, aDirection, aSelected) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMElement getSelectedOrParentTableElement (out AString aTagName, out long aCount); */
    #[inline]
    pub unsafe fn getSelectedOrParentTableElement(&self, ) -> Result<(nsString, libc::int32_t, Option<RefPtr<nsIDOMElement>>), nsresult> {
        let mut aTagName = nsString::new();
        let mut aCount: libc::int32_t = ::std::mem::zeroed();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSelectedOrParentTableElement)(self as *const _, &mut *aTagName, &mut aCount as *mut _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aTagName, aCount, _retval.refptr()))
    }

    /* uint32_t getSelectedCellsType (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn getSelectedCellsType(&self, aElement: Option<&nsIDOMElement>) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getSelectedCellsType)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMElement getFirstSelectedCell (out nsIDOMRange aRange); */
    #[inline]
    pub unsafe fn getFirstSelectedCell(&self, ) -> Result<(Option<RefPtr<nsIDOMRange>>, Option<RefPtr<nsIDOMElement>>), nsresult> {
        let mut aRange = GetterAddrefs::new();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFirstSelectedCell)(self as *const _, aRange.ptr(), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aRange.refptr(), _retval.refptr()))
    }

    /* nsIDOMElement getFirstSelectedCellInTable (out long aRowIndex, out long aColIndex); */
    #[inline]
    pub unsafe fn getFirstSelectedCellInTable(&self, ) -> Result<(libc::int32_t, libc::int32_t, Option<RefPtr<nsIDOMElement>>), nsresult> {
        let mut aRowIndex: libc::int32_t = ::std::mem::zeroed();
        let mut aColIndex: libc::int32_t = ::std::mem::zeroed();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFirstSelectedCellInTable)(self as *const _, &mut aRowIndex as *mut _, &mut aColIndex as *mut _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aRowIndex, aColIndex, _retval.refptr()))
    }

    /* nsIDOMElement getNextSelectedCell (out nsIDOMRange aRange); */
    #[inline]
    pub unsafe fn getNextSelectedCell(&self, ) -> Result<(Option<RefPtr<nsIDOMRange>>, Option<RefPtr<nsIDOMElement>>), nsresult> {
        let mut aRange = GetterAddrefs::new();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getNextSelectedCell)(self as *const _, aRange.ptr(), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aRange.refptr(), _retval.refptr()))
    }

}


