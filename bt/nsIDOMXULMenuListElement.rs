//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULMenuListElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULMenuListElement",
            base: Some("nsIDOMXULSelectControlElement"),
            methods: Some(&[
                    /* attribute boolean editable; */
                    Method {
                        name: "get_editable",
                        abi: "C",
                        params: &[Param { name: "aEditable", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_editable",
                        abi: "C",
                        params: &[Param { name: "aEditable", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean open; */
                    Method {
                        name: "get_open",
                        abi: "C",
                        params: &[Param { name: "aOpen", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_open",
                        abi: "C",
                        params: &[Param { name: "aOpen", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString label; */
                    Method {
                        name: "get_label",
                        abi: "C",
                        params: &[Param { name: "aLabel", ty: "*mut nsAString" }],
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

                    /* readonly attribute nsIDOMNode inputField; */
                    Method {
                        name: "get_inputField",
                        abi: "C",
                        params: &[Param { name: "aInputField", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

