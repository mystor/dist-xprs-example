//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULSelectCntrlItemEl.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULSelectControlItemElement",
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

                    /* attribute DOMString crop; */
                    Method {
                        name: "get_crop",
                        abi: "C",
                        params: &[Param { name: "aCrop", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_crop",
                        abi: "C",
                        params: &[Param { name: "aCrop", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString image; */
                    Method {
                        name: "get_image",
                        abi: "C",
                        params: &[Param { name: "aImage", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_image",
                        abi: "C",
                        params: &[Param { name: "aImage", ty: "*const nsAString" }],
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

                    /* attribute DOMString accessKey; */
                    Method {
                        name: "get_accessKey",
                        abi: "C",
                        params: &[Param { name: "aAccessKey", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_accessKey",
                        abi: "C",
                        params: &[Param { name: "aAccessKey", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString command; */
                    Method {
                        name: "get_command",
                        abi: "C",
                        params: &[Param { name: "aCommand", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_command",
                        abi: "C",
                        params: &[Param { name: "aCommand", ty: "*const nsAString" }],
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

                    /* readonly attribute boolean selected; */
                    Method {
                        name: "get_selected",
                        abi: "C",
                        params: &[Param { name: "aSelected", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMXULSelectControlElement control; */
                    Method {
                        name: "get_control",
                        abi: "C",
                        params: &[Param { name: "aControl", ty: "*mut *const nsIDOMXULSelectControlElement" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

