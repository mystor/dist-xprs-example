//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULMultSelectCntrlEl.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULMultiSelectControlElement",
            base: Some("nsIDOMXULSelectControlElement"),
            methods: Some(&[
                    /* attribute DOMString selType; */
                    Method {
                        name: "get_selType",
                        abi: "C",
                        params: &[Param { name: "aSelType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_selType",
                        abi: "C",
                        params: &[Param { name: "aSelType", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIDOMXULSelectControlItemElement currentItem; */
                    Method {
                        name: "get_currentItem",
                        abi: "C",
                        params: &[Param { name: "aCurrentItem", ty: "*mut *const nsIDOMXULSelectControlItemElement" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_currentItem",
                        abi: "C",
                        params: &[Param { name: "aCurrentItem", ty: "*const nsIDOMXULSelectControlItemElement" }],
                        ret: "nsresult",
                    },

                    /* attribute long currentIndex; */
                    Method {
                        name: "get_currentIndex",
                        abi: "C",
                        params: &[Param { name: "aCurrentIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_currentIndex",
                        abi: "C",
                        params: &[Param { name: "aCurrentIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMNodeList selectedItems; */
                    Method {
                        name: "get_selectedItems",
                        abi: "C",
                        params: &[Param { name: "aSelectedItems", ty: "*mut *const nsIDOMNodeList" }],
                        ret: "nsresult",
                    },

                    /* void addItemToSelection (in nsIDOMXULSelectControlItemElement item); */
                    Method {
                        name: "addItemToSelection",
                        abi: "C",
                        params: &[Param { name: "item", ty: "*const nsIDOMXULSelectControlItemElement" }],
                        ret: "nsresult",
                    },

                    /* void removeItemFromSelection (in nsIDOMXULSelectControlItemElement item); */
                    Method {
                        name: "removeItemFromSelection",
                        abi: "C",
                        params: &[Param { name: "item", ty: "*const nsIDOMXULSelectControlItemElement" }],
                        ret: "nsresult",
                    },

                    /* void toggleItemSelection (in nsIDOMXULSelectControlItemElement item); */
                    Method {
                        name: "toggleItemSelection",
                        abi: "C",
                        params: &[Param { name: "item", ty: "*const nsIDOMXULSelectControlItemElement" }],
                        ret: "nsresult",
                    },

                    /* void selectItem (in nsIDOMXULSelectControlItemElement item); */
                    Method {
                        name: "selectItem",
                        abi: "C",
                        params: &[Param { name: "item", ty: "*const nsIDOMXULSelectControlItemElement" }],
                        ret: "nsresult",
                    },

                    /* void selectItemRange (in nsIDOMXULSelectControlItemElement startItem, in nsIDOMXULSelectControlItemElement item); */
                    Method {
                        name: "selectItemRange",
                        abi: "C",
                        params: &[Param { name: "startItem", ty: "*const nsIDOMXULSelectControlItemElement" }, Param { name: "item", ty: "*const nsIDOMXULSelectControlItemElement" }],
                        ret: "nsresult",
                    },

                    /* void selectAll (); */
                    Method {
                        name: "selectAll",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void invertSelection (); */
                    Method {
                        name: "invertSelection",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void clearSelection (); */
                    Method {
                        name: "clearSelection",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute long selectedCount; */
                    Method {
                        name: "get_selectedCount",
                        abi: "C",
                        params: &[Param { name: "aSelectedCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* [binaryname(MultiGetSelectedItem)] nsIDOMXULSelectControlItemElement getSelectedItem (in long index); */
                    Method {
                        name: "MultiGetSelectedItem",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMXULSelectControlItemElement" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

