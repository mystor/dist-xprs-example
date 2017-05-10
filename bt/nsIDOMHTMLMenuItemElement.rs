//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLMenuItemElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLMenuItemElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*const nsAString" }],
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

                    /* attribute DOMString icon; */
                    Method {
                        name: "get_icon",
                        abi: "C",
                        params: &[Param { name: "aIcon", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_icon",
                        abi: "C",
                        params: &[Param { name: "aIcon", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

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

                    /* attribute boolean defaultChecked; */
                    Method {
                        name: "get_defaultChecked",
                        abi: "C",
                        params: &[Param { name: "aDefaultChecked", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_defaultChecked",
                        abi: "C",
                        params: &[Param { name: "aDefaultChecked", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean checked; */
                    Method {
                        name: "get_checked",
                        abi: "C",
                        params: &[Param { name: "aChecked", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_checked",
                        abi: "C",
                        params: &[Param { name: "aChecked", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString radiogroup; */
                    Method {
                        name: "get_radiogroup",
                        abi: "C",
                        params: &[Param { name: "aRadiogroup", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_radiogroup",
                        abi: "C",
                        params: &[Param { name: "aRadiogroup", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

