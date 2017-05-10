//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleTable.idl
//


#[repr(C)]
pub struct nsIAccessibleTable {
    vtable: *const nsIAccessibleTableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleTable {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcb0bf7b9, 0x117e, 0x40e2,
            [0x9e, 0x46, 0x18, 0x9c, 0x3d, 0x43, 0xce, 0x4a])
    }
}

unsafe impl RefCounted for nsIAccessibleTable {
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
pub trait nsIAccessibleTableCoerce {
    fn coerce_from(v: &nsIAccessibleTable) -> &Self;
}

impl nsIAccessibleTableCoerce for nsIAccessibleTable {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTable) -> &Self {
        v
    }
}

impl nsIAccessibleTable {
    #[inline]
    pub fn coerce<T: nsIAccessibleTableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleTable {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleTableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTable) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleTableVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIAccessible caption; */
    pub get_caption: unsafe extern "C" fn (this: *const nsIAccessibleTable, aCaption: *mut *const nsIAccessible) -> nsresult,

    /* readonly attribute AString summary; */
    pub get_summary: unsafe extern "C" fn (this: *const nsIAccessibleTable, aSummary: *mut nsAString) -> nsresult,

    /* readonly attribute long columnCount; */
    pub get_columnCount: unsafe extern "C" fn (this: *const nsIAccessibleTable, aColumnCount: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long rowCount; */
    pub get_rowCount: unsafe extern "C" fn (this: *const nsIAccessibleTable, aRowCount: *mut libc::int32_t) -> nsresult,

    /* nsIAccessible getCellAt (in long rowIndex, in long columnIndex); */
    pub getCellAt: unsafe extern "C" fn (this: *const nsIAccessibleTable, rowIndex: libc::int32_t, columnIndex: libc::int32_t, _retval: *mut *const nsIAccessible) -> nsresult,

    /* long getCellIndexAt (in long rowIndex, in long columnIndex); */
    pub getCellIndexAt: unsafe extern "C" fn (this: *const nsIAccessibleTable, rowIndex: libc::int32_t, columnIndex: libc::int32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* long getColumnIndexAt (in long cellIndex); */
    pub getColumnIndexAt: unsafe extern "C" fn (this: *const nsIAccessibleTable, cellIndex: libc::int32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* long getRowIndexAt (in long cellIndex); */
    pub getRowIndexAt: unsafe extern "C" fn (this: *const nsIAccessibleTable, cellIndex: libc::int32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* void getRowAndColumnIndicesAt (in long cellIndex, out long rowIndex, out long columnIndex); */
    pub getRowAndColumnIndicesAt: unsafe extern "C" fn (this: *const nsIAccessibleTable, cellIndex: libc::int32_t, rowIndex: *mut libc::int32_t, columnIndex: *mut libc::int32_t) -> nsresult,

    /* long getColumnExtentAt (in long row, in long column); */
    pub getColumnExtentAt: unsafe extern "C" fn (this: *const nsIAccessibleTable, row: libc::int32_t, column: libc::int32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* long getRowExtentAt (in long row, in long column); */
    pub getRowExtentAt: unsafe extern "C" fn (this: *const nsIAccessibleTable, row: libc::int32_t, column: libc::int32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* AString getColumnDescription (in long columnIndex); */
    pub getColumnDescription: unsafe extern "C" fn (this: *const nsIAccessibleTable, columnIndex: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getRowDescription (in long rowIndex); */
    pub getRowDescription: unsafe extern "C" fn (this: *const nsIAccessibleTable, rowIndex: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* boolean isColumnSelected (in long columnIndex); */
    pub isColumnSelected: unsafe extern "C" fn (this: *const nsIAccessibleTable, columnIndex: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* boolean isRowSelected (in long rowIndex); */
    pub isRowSelected: unsafe extern "C" fn (this: *const nsIAccessibleTable, rowIndex: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* boolean isCellSelected (in long rowIndex, in long columnIndex); */
    pub isCellSelected: unsafe extern "C" fn (this: *const nsIAccessibleTable, rowIndex: libc::int32_t, columnIndex: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* readonly attribute unsigned long selectedCellCount; */
    pub get_selectedCellCount: unsafe extern "C" fn (this: *const nsIAccessibleTable, aSelectedCellCount: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long selectedColumnCount; */
    pub get_selectedColumnCount: unsafe extern "C" fn (this: *const nsIAccessibleTable, aSelectedColumnCount: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long selectedRowCount; */
    pub get_selectedRowCount: unsafe extern "C" fn (this: *const nsIAccessibleTable, aSelectedRowCount: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIArray selectedCells; */
    pub get_selectedCells: unsafe extern "C" fn (this: *const nsIAccessibleTable, aSelectedCells: *mut *const nsIArray) -> nsresult,

    /* void getSelectedCellIndices (out unsigned long cellsArraySize, [array, size_is (cellsArraySize), retval] out long cellsArray); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSelectedCellIndices: *const ::libc::c_void,

    /* void getSelectedColumnIndices (out unsigned long columnsArraySize, [array, size_is (columnsArraySize), retval] out long columnsArray); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSelectedColumnIndices: *const ::libc::c_void,

    /* void getSelectedRowIndices (out unsigned long rowsArraySize, [array, size_is (rowsArraySize), retval] out long rowsArray); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSelectedRowIndices: *const ::libc::c_void,

    /* void selectRow (in long rowIndex); */
    pub selectRow: unsafe extern "C" fn (this: *const nsIAccessibleTable, rowIndex: libc::int32_t) -> nsresult,

    /* void selectColumn (in long columnIndex); */
    pub selectColumn: unsafe extern "C" fn (this: *const nsIAccessibleTable, columnIndex: libc::int32_t) -> nsresult,

    /* void unselectRow (in long rowIndex); */
    pub unselectRow: unsafe extern "C" fn (this: *const nsIAccessibleTable, rowIndex: libc::int32_t) -> nsresult,

    /* void unselectColumn (in long columnIndex); */
    pub unselectColumn: unsafe extern "C" fn (this: *const nsIAccessibleTable, columnIndex: libc::int32_t) -> nsresult,

    /* boolean isProbablyForLayout (); */
    pub isProbablyForLayout: unsafe extern "C" fn (this: *const nsIAccessibleTable, _retval: *mut bool) -> nsresult,

}


impl nsIAccessibleTable {
    /* readonly attribute nsIAccessible caption; */
    #[inline]
    pub unsafe fn get_caption(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_caption)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AString summary; */
    #[inline]
    pub unsafe fn get_summary(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_summary)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long columnCount; */
    #[inline]
    pub unsafe fn get_columnCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_columnCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

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

    /* nsIAccessible getCellAt (in long rowIndex, in long columnIndex); */
    #[inline]
    pub unsafe fn getCellAt(&self, rowIndex: libc::int32_t, columnIndex: libc::int32_t) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getCellAt)(self as *const _, rowIndex, columnIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* long getCellIndexAt (in long rowIndex, in long columnIndex); */
    #[inline]
    pub unsafe fn getCellIndexAt(&self, rowIndex: libc::int32_t, columnIndex: libc::int32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getCellIndexAt)(self as *const _, rowIndex, columnIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getColumnIndexAt (in long cellIndex); */
    #[inline]
    pub unsafe fn getColumnIndexAt(&self, cellIndex: libc::int32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getColumnIndexAt)(self as *const _, cellIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getRowIndexAt (in long cellIndex); */
    #[inline]
    pub unsafe fn getRowIndexAt(&self, cellIndex: libc::int32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getRowIndexAt)(self as *const _, cellIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getRowAndColumnIndicesAt (in long cellIndex, out long rowIndex, out long columnIndex); */
    #[inline]
    pub unsafe fn getRowAndColumnIndicesAt(&self, cellIndex: libc::int32_t) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut rowIndex: libc::int32_t = ::std::mem::zeroed();
        let mut columnIndex: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getRowAndColumnIndicesAt)(self as *const _, cellIndex, &mut rowIndex as *mut _, &mut columnIndex as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((rowIndex, columnIndex))
    }

    /* long getColumnExtentAt (in long row, in long column); */
    #[inline]
    pub unsafe fn getColumnExtentAt(&self, row: libc::int32_t, column: libc::int32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getColumnExtentAt)(self as *const _, row, column, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getRowExtentAt (in long row, in long column); */
    #[inline]
    pub unsafe fn getRowExtentAt(&self, row: libc::int32_t, column: libc::int32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getRowExtentAt)(self as *const _, row, column, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getColumnDescription (in long columnIndex); */
    #[inline]
    pub unsafe fn getColumnDescription(&self, columnIndex: libc::int32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getColumnDescription)(self as *const _, columnIndex, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getRowDescription (in long rowIndex); */
    #[inline]
    pub unsafe fn getRowDescription(&self, rowIndex: libc::int32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getRowDescription)(self as *const _, rowIndex, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isColumnSelected (in long columnIndex); */
    #[inline]
    pub unsafe fn isColumnSelected(&self, columnIndex: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isColumnSelected)(self as *const _, columnIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isRowSelected (in long rowIndex); */
    #[inline]
    pub unsafe fn isRowSelected(&self, rowIndex: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isRowSelected)(self as *const _, rowIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isCellSelected (in long rowIndex, in long columnIndex); */
    #[inline]
    pub unsafe fn isCellSelected(&self, rowIndex: libc::int32_t, columnIndex: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isCellSelected)(self as *const _, rowIndex, columnIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long selectedCellCount; */
    #[inline]
    pub unsafe fn get_selectedCellCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_selectedCellCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long selectedColumnCount; */
    #[inline]
    pub unsafe fn get_selectedColumnCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_selectedColumnCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long selectedRowCount; */
    #[inline]
    pub unsafe fn get_selectedRowCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_selectedRowCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIArray selectedCells; */
    #[inline]
    pub unsafe fn get_selectedCells(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_selectedCells)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void getSelectedCellIndices (out unsigned long cellsArraySize, [array, size_is (cellsArraySize), retval] out long cellsArray); */


    /* void getSelectedColumnIndices (out unsigned long columnsArraySize, [array, size_is (columnsArraySize), retval] out long columnsArray); */


    /* void getSelectedRowIndices (out unsigned long rowsArraySize, [array, size_is (rowsArraySize), retval] out long rowsArray); */


    /* void selectRow (in long rowIndex); */
    #[inline]
    pub unsafe fn selectRow(&self, rowIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).selectRow)(self as *const _, rowIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectColumn (in long columnIndex); */
    #[inline]
    pub unsafe fn selectColumn(&self, columnIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).selectColumn)(self as *const _, columnIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unselectRow (in long rowIndex); */
    #[inline]
    pub unsafe fn unselectRow(&self, rowIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).unselectRow)(self as *const _, rowIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unselectColumn (in long columnIndex); */
    #[inline]
    pub unsafe fn unselectColumn(&self, columnIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).unselectColumn)(self as *const _, columnIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isProbablyForLayout (); */
    #[inline]
    pub unsafe fn isProbablyForLayout(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isProbablyForLayout)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIAccessibleTableCell {
    vtable: *const nsIAccessibleTableCellVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleTableCell {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x654e296d, 0xfae6, 0x452b,
            [0x98, 0x7d, 0x74, 0x6b, 0x20, 0xb9, 0x51, 0x4b])
    }
}

unsafe impl RefCounted for nsIAccessibleTableCell {
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
pub trait nsIAccessibleTableCellCoerce {
    fn coerce_from(v: &nsIAccessibleTableCell) -> &Self;
}

impl nsIAccessibleTableCellCoerce for nsIAccessibleTableCell {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTableCell) -> &Self {
        v
    }
}

impl nsIAccessibleTableCell {
    #[inline]
    pub fn coerce<T: nsIAccessibleTableCellCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleTableCell {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleTableCellCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTableCell) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleTableCellVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIAccessibleTable table; */
    pub get_table: unsafe extern "C" fn (this: *const nsIAccessibleTableCell, aTable: *mut *const nsIAccessibleTable) -> nsresult,

    /* readonly attribute long columnIndex; */
    pub get_columnIndex: unsafe extern "C" fn (this: *const nsIAccessibleTableCell, aColumnIndex: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long rowIndex; */
    pub get_rowIndex: unsafe extern "C" fn (this: *const nsIAccessibleTableCell, aRowIndex: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long columnExtent; */
    pub get_columnExtent: unsafe extern "C" fn (this: *const nsIAccessibleTableCell, aColumnExtent: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long rowExtent; */
    pub get_rowExtent: unsafe extern "C" fn (this: *const nsIAccessibleTableCell, aRowExtent: *mut libc::int32_t) -> nsresult,

    /* readonly attribute nsIArray columnHeaderCells; */
    pub get_columnHeaderCells: unsafe extern "C" fn (this: *const nsIAccessibleTableCell, aColumnHeaderCells: *mut *const nsIArray) -> nsresult,

    /* readonly attribute nsIArray rowHeaderCells; */
    pub get_rowHeaderCells: unsafe extern "C" fn (this: *const nsIAccessibleTableCell, aRowHeaderCells: *mut *const nsIArray) -> nsresult,

    /* boolean isSelected (); */
    pub isSelected: unsafe extern "C" fn (this: *const nsIAccessibleTableCell, _retval: *mut bool) -> nsresult,

}


impl nsIAccessibleTableCell {
    /* readonly attribute nsIAccessibleTable table; */
    #[inline]
    pub unsafe fn get_table(&self, ) -> Result<Option<RefPtr<nsIAccessibleTable>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_table)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long columnIndex; */
    #[inline]
    pub unsafe fn get_columnIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_columnIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long rowIndex; */
    #[inline]
    pub unsafe fn get_rowIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_rowIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long columnExtent; */
    #[inline]
    pub unsafe fn get_columnExtent(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_columnExtent)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long rowExtent; */
    #[inline]
    pub unsafe fn get_rowExtent(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_rowExtent)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIArray columnHeaderCells; */
    #[inline]
    pub unsafe fn get_columnHeaderCells(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_columnHeaderCells)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIArray rowHeaderCells; */
    #[inline]
    pub unsafe fn get_rowHeaderCells(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_rowHeaderCells)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean isSelected (); */
    #[inline]
    pub unsafe fn isSelected(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isSelected)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


