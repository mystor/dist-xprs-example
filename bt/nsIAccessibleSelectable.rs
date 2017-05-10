//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleSelectable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleSelectable",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIArray selectedItems; */
                    Method {
                        name: "get_selectedItems",
                        abi: "C",
                        params: &[Param { name: "aSelectedItems", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long selectedItemCount; */
                    Method {
                        name: "get_selectedItemCount",
                        abi: "C",
                        params: &[Param { name: "aSelectedItemCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessible getSelectedItemAt (in unsigned long index); */
                    Method {
                        name: "getSelectedItemAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* boolean isItemSelected (in unsigned long index); */
                    Method {
                        name: "isItemSelected",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void addItemToSelection (in unsigned long index); */
                    Method {
                        name: "addItemToSelection",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void removeItemFromSelection (in unsigned long index); */
                    Method {
                        name: "removeItemFromSelection",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* boolean selectAll (); */
                    Method {
                        name: "selectAll",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void unselectAll (); */
                    Method {
                        name: "unselectAll",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

