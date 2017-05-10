//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITreeBoxObject.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITreeBoxObject",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsITreeColumns columns; */
                    Method {
                        name: "get_columns",
                        abi: "C",
                        params: &[Param { name: "aColumns", ty: "*mut *const nsITreeColumns" }],
                        ret: "nsresult",
                    },

                    /* attribute nsITreeView view; */
                    Method {
                        name: "get_view",
                        abi: "C",
                        params: &[Param { name: "aView", ty: "*mut *const nsITreeView" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_view",
                        abi: "C",
                        params: &[Param { name: "aView", ty: "*const nsITreeView" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean focused; */
                    Method {
                        name: "get_focused",
                        abi: "C",
                        params: &[Param { name: "aFocused", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_focused",
                        abi: "C",
                        params: &[Param { name: "aFocused", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMElement treeBody; */
                    Method {
                        name: "get_treeBody",
                        abi: "C",
                        params: &[Param { name: "aTreeBody", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long rowHeight; */
                    Method {
                        name: "get_rowHeight",
                        abi: "C",
                        params: &[Param { name: "aRowHeight", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long rowWidth; */
                    Method {
                        name: "get_rowWidth",
                        abi: "C",
                        params: &[Param { name: "aRowWidth", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long horizontalPosition; */
                    Method {
                        name: "get_horizontalPosition",
                        abi: "C",
                        params: &[Param { name: "aHorizontalPosition", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIScriptableRegion selectionRegion; */
                    Method {
                        name: "get_selectionRegion",
                        abi: "C",
                        params: &[Param { name: "aSelectionRegion", ty: "*mut *const nsIScriptableRegion" }],
                        ret: "nsresult",
                    },

                    /* long getFirstVisibleRow (); */
                    Method {
                        name: "getFirstVisibleRow",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* long getLastVisibleRow (); */
                    Method {
                        name: "getLastVisibleRow",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* long getPageLength (); */
                    Method {
                        name: "getPageLength",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void ensureRowIsVisible (in long index); */
                    Method {
                        name: "ensureRowIsVisible",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void ensureCellIsVisible (in long row, in nsITreeColumn col); */
                    Method {
                        name: "ensureCellIsVisible",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* void scrollToRow (in long index); */
                    Method {
                        name: "scrollToRow",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void scrollByLines (in long numLines); */
                    Method {
                        name: "scrollByLines",
                        abi: "C",
                        params: &[Param { name: "numLines", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void scrollByPages (in long numPages); */
                    Method {
                        name: "scrollByPages",
                        abi: "C",
                        params: &[Param { name: "numPages", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void scrollToCell (in long row, in nsITreeColumn col); */
                    Method {
                        name: "scrollToCell",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* void scrollToColumn (in nsITreeColumn col); */
                    Method {
                        name: "scrollToColumn",
                        abi: "C",
                        params: &[Param { name: "col", ty: "*const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* void scrollToHorizontalPosition (in long horizontalPosition); */
                    Method {
                        name: "scrollToHorizontalPosition",
                        abi: "C",
                        params: &[Param { name: "horizontalPosition", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void invalidate (); */
                    Method {
                        name: "invalidate",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void invalidateColumn (in nsITreeColumn col); */
                    Method {
                        name: "invalidateColumn",
                        abi: "C",
                        params: &[Param { name: "col", ty: "*const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* void invalidateRow (in long index); */
                    Method {
                        name: "invalidateRow",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void invalidateCell (in long row, in nsITreeColumn col); */
                    Method {
                        name: "invalidateCell",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* void invalidateRange (in long startIndex, in long endIndex); */
                    Method {
                        name: "invalidateRange",
                        abi: "C",
                        params: &[Param { name: "startIndex", ty: "libc::int32_t" }, Param { name: "endIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void invalidateColumnRange (in long startIndex, in long endIndex, in nsITreeColumn col); */
                    Method {
                        name: "invalidateColumnRange",
                        abi: "C",
                        params: &[Param { name: "startIndex", ty: "libc::int32_t" }, Param { name: "endIndex", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* long getRowAt (in long x, in long y); */
                    Method {
                        name: "getRowAt",
                        abi: "C",
                        params: &[Param { name: "x", ty: "libc::int32_t" }, Param { name: "y", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void getCellAt (in long x, in long y, out long row, out nsITreeColumn col, out AString childElt); */
                    Method {
                        name: "getCellAt",
                        abi: "C",
                        params: &[Param { name: "x", ty: "libc::int32_t" }, Param { name: "y", ty: "libc::int32_t" }, Param { name: "row", ty: "*mut libc::int32_t" }, Param { name: "col", ty: "*mut *const nsITreeColumn" }, Param { name: "childElt", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void getCoordsForCellItem (in long row, in nsITreeColumn col, in AString element, out long x, out long y, out long width, out long height); */
                    Method {
                        name: "getCoordsForCellItem",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }, Param { name: "element", ty: "*const nsAString" }, Param { name: "x", ty: "*mut libc::int32_t" }, Param { name: "y", ty: "*mut libc::int32_t" }, Param { name: "width", ty: "*mut libc::int32_t" }, Param { name: "height", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* boolean isCellCropped (in long row, in nsITreeColumn col); */
                    Method {
                        name: "isCellCropped",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void rowCountChanged (in long index, in long count); */
                    Method {
                        name: "rowCountChanged",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "count", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void beginUpdateBatch (); */
                    Method {
                        name: "beginUpdateBatch",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void endUpdateBatch (); */
                    Method {
                        name: "endUpdateBatch",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void clearStyleAndImageCaches (); */
                    Method {
                        name: "clearStyleAndImageCaches",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void removeImageCacheEntry (in long row, in nsITreeColumn col); */
                    Method {
                        name: "removeImageCacheEntry",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

