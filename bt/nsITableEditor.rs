//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITableEditor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITableEditor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void insertTableCell (in long aNumber, in boolean aAfter); */
                    Method {
                        name: "insertTableCell",
                        abi: "C",
                        params: &[Param { name: "aNumber", ty: "libc::int32_t" }, Param { name: "aAfter", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void insertTableColumn (in long aNumber, in boolean aAfter); */
                    Method {
                        name: "insertTableColumn",
                        abi: "C",
                        params: &[Param { name: "aNumber", ty: "libc::int32_t" }, Param { name: "aAfter", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void insertTableRow (in long aNumber, in boolean aAfter); */
                    Method {
                        name: "insertTableRow",
                        abi: "C",
                        params: &[Param { name: "aNumber", ty: "libc::int32_t" }, Param { name: "aAfter", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void deleteTable (); */
                    Method {
                        name: "deleteTable",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void deleteTableCellContents (); */
                    Method {
                        name: "deleteTableCellContents",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void deleteTableCell (in long aNumber); */
                    Method {
                        name: "deleteTableCell",
                        abi: "C",
                        params: &[Param { name: "aNumber", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void deleteTableColumn (in long aNumber); */
                    Method {
                        name: "deleteTableColumn",
                        abi: "C",
                        params: &[Param { name: "aNumber", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void deleteTableRow (in long aNumber); */
                    Method {
                        name: "deleteTableRow",
                        abi: "C",
                        params: &[Param { name: "aNumber", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void selectTableCell (); */
                    Method {
                        name: "selectTableCell",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void selectBlockOfCells (in nsIDOMElement aStartCell, in nsIDOMElement aEndCell); */
                    Method {
                        name: "selectBlockOfCells",
                        abi: "C",
                        params: &[Param { name: "aStartCell", ty: "*const nsIDOMElement" }, Param { name: "aEndCell", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void selectTableRow (); */
                    Method {
                        name: "selectTableRow",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void selectTableColumn (); */
                    Method {
                        name: "selectTableColumn",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void selectTable (); */
                    Method {
                        name: "selectTable",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void selectAllTableCells (); */
                    Method {
                        name: "selectAllTableCells",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* nsIDOMElement switchTableCellHeaderType (in nsIDOMElement aSourceCell); */
                    Method {
                        name: "switchTableCellHeaderType",
                        abi: "C",
                        params: &[Param { name: "aSourceCell", ty: "*const nsIDOMElement" }, Param { name: "_retval", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void joinTableCells (in boolean aMergeNonContiguousContents); */
                    Method {
                        name: "joinTableCells",
                        abi: "C",
                        params: &[Param { name: "aMergeNonContiguousContents", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void splitTableCell (); */
                    Method {
                        name: "splitTableCell",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void normalizeTable (in nsIDOMElement aTable); */
                    Method {
                        name: "normalizeTable",
                        abi: "C",
                        params: &[Param { name: "aTable", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void getCellIndexes (in nsIDOMElement aCell, out long aRowIndex, out long aColIndex); */
                    Method {
                        name: "getCellIndexes",
                        abi: "C",
                        params: &[Param { name: "aCell", ty: "*const nsIDOMElement" }, Param { name: "aRowIndex", ty: "*mut libc::int32_t" }, Param { name: "aColIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void getTableSize (in nsIDOMElement aTable, out long aRowCount, out long aColCount); */
                    Method {
                        name: "getTableSize",
                        abi: "C",
                        params: &[Param { name: "aTable", ty: "*const nsIDOMElement" }, Param { name: "aRowCount", ty: "*mut libc::int32_t" }, Param { name: "aColCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMElement getCellAt (in nsIDOMElement aTable, in long aRowIndex, in long aColIndex); */
                    Method {
                        name: "getCellAt",
                        abi: "C",
                        params: &[Param { name: "aTable", ty: "*const nsIDOMElement" }, Param { name: "aRowIndex", ty: "libc::int32_t" }, Param { name: "aColIndex", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void getCellDataAt (in nsIDOMElement aTable, in long aRowIndex, in long aColIndex, out nsIDOMElement aCell, out long aStartRowIndex, out long aStartColIndex, out long aRowSpan, out long aColSpan, out long aActualRowSpan, out long aActualColSpan, out boolean aIsSelected); */
                    Method {
                        name: "getCellDataAt",
                        abi: "C",
                        params: &[Param { name: "aTable", ty: "*const nsIDOMElement" }, Param { name: "aRowIndex", ty: "libc::int32_t" }, Param { name: "aColIndex", ty: "libc::int32_t" }, Param { name: "aCell", ty: "*mut *const nsIDOMElement" }, Param { name: "aStartRowIndex", ty: "*mut libc::int32_t" }, Param { name: "aStartColIndex", ty: "*mut libc::int32_t" }, Param { name: "aRowSpan", ty: "*mut libc::int32_t" }, Param { name: "aColSpan", ty: "*mut libc::int32_t" }, Param { name: "aActualRowSpan", ty: "*mut libc::int32_t" }, Param { name: "aActualColSpan", ty: "*mut libc::int32_t" }, Param { name: "aIsSelected", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode getFirstRow (in nsIDOMElement aTableElement); */
                    Method {
                        name: "getFirstRow",
                        abi: "C",
                        params: &[Param { name: "aTableElement", ty: "*const nsIDOMElement" }, Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode getNextRow (in nsIDOMNode aTableElement); */
                    Method {
                        name: "getNextRow",
                        abi: "C",
                        params: &[Param { name: "aTableElement", ty: "*const nsIDOMNode" }, Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void setSelectionAfterTableEdit (in nsIDOMElement aTable, in long aRow, in long aCol, in long aDirection, in boolean aSelected); */
                    Method {
                        name: "setSelectionAfterTableEdit",
                        abi: "C",
                        params: &[Param { name: "aTable", ty: "*const nsIDOMElement" }, Param { name: "aRow", ty: "libc::int32_t" }, Param { name: "aCol", ty: "libc::int32_t" }, Param { name: "aDirection", ty: "libc::int32_t" }, Param { name: "aSelected", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMElement getSelectedOrParentTableElement (out AString aTagName, out long aCount); */
                    Method {
                        name: "getSelectedOrParentTableElement",
                        abi: "C",
                        params: &[Param { name: "aTagName", ty: "*mut nsAString" }, Param { name: "aCount", ty: "*mut libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* uint32_t getSelectedCellsType (in nsIDOMElement aElement); */
                    Method {
                        name: "getSelectedCellsType",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIDOMElement" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMElement getFirstSelectedCell (out nsIDOMRange aRange); */
                    Method {
                        name: "getFirstSelectedCell",
                        abi: "C",
                        params: &[Param { name: "aRange", ty: "*mut *const nsIDOMRange" }, Param { name: "_retval", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMElement getFirstSelectedCellInTable (out long aRowIndex, out long aColIndex); */
                    Method {
                        name: "getFirstSelectedCellInTable",
                        abi: "C",
                        params: &[Param { name: "aRowIndex", ty: "*mut libc::int32_t" }, Param { name: "aColIndex", ty: "*mut libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMElement getNextSelectedCell (out nsIDOMRange aRange); */
                    Method {
                        name: "getNextSelectedCell",
                        abi: "C",
                        params: &[Param { name: "aRange", ty: "*mut *const nsIDOMRange" }, Param { name: "_retval", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

