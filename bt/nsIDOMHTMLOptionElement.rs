//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLOptionElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLOptionElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute boolean disabled; */
                    Method {
                        name: "get_disabled",
                        abi: "C",
                        params: &[Param { name: "aDisabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_disabled",
                        abi: "C",
                        params: &[Param { name: "aDisabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMHTMLFormElement form; */
                    Method {
                        name: "get_form",
                        abi: "C",
                        params: &[Param { name: "aForm", ty: "*mut *const nsIDOMHTMLFormElement" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString label; */
                    Method {
                        name: "get_label",
                        abi: "C",
                        params: &[Param { name: "aLabel", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_label",
                        abi: "C",
                        params: &[Param { name: "aLabel", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean defaultSelected; */
                    Method {
                        name: "get_defaultSelected",
                        abi: "C",
                        params: &[Param { name: "aDefaultSelected", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_defaultSelected",
                        abi: "C",
                        params: &[Param { name: "aDefaultSelected", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean selected; */
                    Method {
                        name: "get_selected",
                        abi: "C",
                        params: &[Param { name: "aSelected", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_selected",
                        abi: "C",
                        params: &[Param { name: "aSelected", ty: "bool" }],
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

                    /* attribute DOMString text; */
                    Method {
                        name: "get_text",
                        abi: "C",
                        params: &[Param { name: "aText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_text",
                        abi: "C",
                        params: &[Param { name: "aText", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long index; */
                    Method {
                        name: "get_index",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

