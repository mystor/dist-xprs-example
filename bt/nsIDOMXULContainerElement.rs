//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULContainerElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULContainerItemElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMXULContainerElement parentContainer; */
                    Method {
                        name: "get_parentContainer",
                        abi: "C",
                        params: &[Param { name: "aParentContainer", ty: "*mut *const nsIDOMXULContainerElement" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDOMXULContainerElement",
            base: Some("nsIDOMXULContainerItemElement"),
            methods: Some(&[
                    /* nsIDOMXULElement appendItem (in DOMString aLabel, in DOMString aValue); */
                    Method {
                        name: "appendItem",
                        abi: "C",
                        params: &[Param { name: "aLabel", ty: "*const nsAString" }, Param { name: "aValue", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMXULElement" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMXULElement insertItemAt (in long aIndex, in DOMString aLabel, in DOMString aValue); */
                    Method {
                        name: "insertItemAt",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::int32_t" }, Param { name: "aLabel", ty: "*const nsAString" }, Param { name: "aValue", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMXULElement" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMXULElement removeItemAt (in long aIndex); */
                    Method {
                        name: "removeItemAt",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMXULElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long itemCount; */
                    Method {
                        name: "get_itemCount",
                        abi: "C",
                        params: &[Param { name: "aItemCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* long getIndexOfItem (in nsIDOMXULElement aItem); */
                    Method {
                        name: "getIndexOfItem",
                        abi: "C",
                        params: &[Param { name: "aItem", ty: "*const nsIDOMXULElement" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMXULElement getItemAtIndex (in long aIndex); */
                    Method {
                        name: "getItemAtIndex",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMXULElement" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

