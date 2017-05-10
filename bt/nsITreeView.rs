//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITreeView.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITreeView",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute long rowCount; */
                    Method {
                        name: "get_rowCount",
                        abi: "C",
                        params: &[Param { name: "aRowCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute nsITreeSelection selection; */
                    Method {
                        name: "get_selection",
                        abi: "C",
                        params: &[Param { name: "aSelection", ty: "*mut *const nsITreeSelection" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_selection",
                        abi: "C",
                        params: &[Param { name: "aSelection", ty: "*const nsITreeSelection" }],
                        ret: "nsresult",
                    },

                    /* AString getRowProperties (in long index); */
                    Method {
                        name: "getRowProperties",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getCellProperties (in long row, in nsITreeColumn col); */
                    Method {
                        name: "getCellProperties",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getColumnProperties (in nsITreeColumn col); */
                    Method {
                        name: "getColumnProperties",
                        abi: "C",
                        params: &[Param { name: "col", ty: "*const nsITreeColumn" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* boolean isContainer (in long index); */
                    Method {
                        name: "isContainer",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isContainerOpen (in long index); */
                    Method {
                        name: "isContainerOpen",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isContainerEmpty (in long index); */
                    Method {
                        name: "isContainerEmpty",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isSeparator (in long index); */
                    Method {
                        name: "isSeparator",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isSorted (); */
                    Method {
                        name: "isSorted",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean canDrop (in long index, in long orientation, in nsIDOMDataTransfer dataTransfer); */
                    Method {
                        name: "canDrop",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "orientation", ty: "libc::int32_t" }, Param { name: "dataTransfer", ty: "*const nsIDOMDataTransfer" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void drop (in long row, in long orientation, in nsIDOMDataTransfer dataTransfer); */
                    Method {
                        name: "drop",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "orientation", ty: "libc::int32_t" }, Param { name: "dataTransfer", ty: "*const nsIDOMDataTransfer" }],
                        ret: "nsresult",
                    },

                    /* long getParentIndex (in long rowIndex); */
                    Method {
                        name: "getParentIndex",
                        abi: "C",
                        params: &[Param { name: "rowIndex", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* boolean hasNextSibling (in long rowIndex, in long afterIndex); */
                    Method {
                        name: "hasNextSibling",
                        abi: "C",
                        params: &[Param { name: "rowIndex", ty: "libc::int32_t" }, Param { name: "afterIndex", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* long getLevel (in long index); */
                    Method {
                        name: "getLevel",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* AString getImageSrc (in long row, in nsITreeColumn col); */
                    Method {
                        name: "getImageSrc",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* long getProgressMode (in long row, in nsITreeColumn col); */
                    Method {
                        name: "getProgressMode",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* AString getCellValue (in long row, in nsITreeColumn col); */
                    Method {
                        name: "getCellValue",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getCellText (in long row, in nsITreeColumn col); */
                    Method {
                        name: "getCellText",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setTree (in nsITreeBoxObject tree); */
                    Method {
                        name: "setTree",
                        abi: "C",
                        params: &[Param { name: "tree", ty: "*const nsITreeBoxObject" }],
                        ret: "nsresult",
                    },

                    /* void toggleOpenState (in long index); */
                    Method {
                        name: "toggleOpenState",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void cycleHeader (in nsITreeColumn col); */
                    Method {
                        name: "cycleHeader",
                        abi: "C",
                        params: &[Param { name: "col", ty: "*const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* void selectionChanged (); */
                    Method {
                        name: "selectionChanged",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void cycleCell (in long row, in nsITreeColumn col); */
                    Method {
                        name: "cycleCell",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* boolean isEditable (in long row, in nsITreeColumn col); */
                    Method {
                        name: "isEditable",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isSelectable (in long row, in nsITreeColumn col); */
                    Method {
                        name: "isSelectable",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void setCellValue (in long row, in nsITreeColumn col, in AString value); */
                    Method {
                        name: "setCellValue",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }, Param { name: "value", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setCellText (in long row, in nsITreeColumn col, in AString value); */
                    Method {
                        name: "setCellText",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }, Param { name: "value", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void performAction (in wstring action); */
                    Method {
                        name: "performAction",
                        abi: "C",
                        params: &[Param { name: "action", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void performActionOnRow (in wstring action, in long row); */
                    Method {
                        name: "performActionOnRow",
                        abi: "C",
                        params: &[Param { name: "action", ty: "*const libc::int16_t" }, Param { name: "row", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void performActionOnCell (in wstring action, in long row, in nsITreeColumn col); */
                    Method {
                        name: "performActionOnCell",
                        abi: "C",
                        params: &[Param { name: "action", ty: "*const libc::int16_t" }, Param { name: "row", ty: "libc::int32_t" }, Param { name: "col", ty: "*const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINativeTreeView",
            base: Some("nsITreeView"),
            methods: Some(&[
                    /* [noscript] void ensureNative (); */
                    Method {
                        name: "ensureNative",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

