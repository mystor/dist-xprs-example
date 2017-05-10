//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleTableChangeEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleTableChangeEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Some(&[
                    /* readonly attribute long rowOrColIndex; */
                    Method {
                        name: "get_rowOrColIndex",
                        abi: "C",
                        params: &[Param { name: "aRowOrColIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long RowsOrColsCount; */
                    Method {
                        name: "get_RowsOrColsCount",
                        abi: "C",
                        params: &[Param { name: "aRowsOrColsCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

