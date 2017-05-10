//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleTable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleTable",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIAccessibleTableCell",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIAccessibleTable table; */
                    Method {
                        name: "get_table",
                        abi: "C",
                        params: &[Param { name: "aTable", ty: "*mut *const nsIAccessibleTable" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long columnIndex; */
                    Method {
                        name: "get_columnIndex",
                        abi: "C",
                        params: &[Param { name: "aColumnIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long rowIndex; */
                    Method {
                        name: "get_rowIndex",
                        abi: "C",
                        params: &[Param { name: "aRowIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long columnExtent; */
                    Method {
                        name: "get_columnExtent",
                        abi: "C",
                        params: &[Param { name: "aColumnExtent", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long rowExtent; */
                    Method {
                        name: "get_rowExtent",
                        abi: "C",
                        params: &[Param { name: "aRowExtent", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIArray columnHeaderCells; */
                    Method {
                        name: "get_columnHeaderCells",
                        abi: "C",
                        params: &[Param { name: "aColumnHeaderCells", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIArray rowHeaderCells; */
                    Method {
                        name: "get_rowHeaderCells",
                        abi: "C",
                        params: &[Param { name: "aRowHeaderCells", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* boolean isSelected (); */
                    Method {
                        name: "isSelected",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

