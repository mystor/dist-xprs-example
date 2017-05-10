//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULSelectCntrlEl.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULSelectControlElement",
            base: Some("nsIDOMXULControlElement"),
            methods: Some(&[
                    /* attribute nsIDOMXULSelectControlItemElement selectedItem; */
                    Method {
                        name: "get_selectedItem",
                        abi: "C",
                        params: &[Param { name: "aSelectedItem", ty: "*mut *const nsIDOMXULSelectControlItemElement" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_selectedItem",
                        abi: "C",
                        params: &[Param { name: "aSelectedItem", ty: "*const nsIDOMXULSelectControlItemElement" }],
                        ret: "nsresult",
                    },

                    /* attribute long selectedIndex; */
                    Method {
                        name: "get_selectedIndex",
                        abi: "C",
                        params: &[Param { name: "aSelectedIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_selectedIndex",
                        abi: "C",
                        params: &[Param { name: "aSelectedIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString value; */
                    Method {
                        name: "get_value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMXULSelectControlItemElement appendItem (in DOMString label, in DOMString value); */
                    Method {
                        name: "appendItem",
                        abi: "C",
                        params: &[Param { name: "label", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMXULSelectControlItemElement" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMXULSelectControlItemElement insertItemAt (in long index, in DOMString label, in DOMString value); */
                    Method {
                        name: "insertItemAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "label", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMXULSelectControlItemElement" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMXULSelectControlItemElement removeItemAt (in long index); */
                    Method {
                        name: "removeItemAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMXULSelectControlItemElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long itemCount; */
                    Method {
                        name: "get_itemCount",
                        abi: "C",
                        params: &[Param { name: "aItemCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* long getIndexOfItem (in nsIDOMXULSelectControlItemElement item); */
                    Method {
                        name: "getIndexOfItem",
                        abi: "C",
                        params: &[Param { name: "item", ty: "*const nsIDOMXULSelectControlItemElement" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMXULSelectControlItemElement getItemAtIndex (in long index); */
                    Method {
                        name: "getItemAtIndex",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMXULSelectControlItemElement" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

