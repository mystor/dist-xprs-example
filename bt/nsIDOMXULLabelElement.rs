//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULLabelElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULLabelElement",
            base: Some("nsIDOMXULDescriptionElement"),
            methods: Some(&[
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

                    /* attribute DOMString control; */
                    Method {
                        name: "get_control",
                        abi: "C",
                        params: &[Param { name: "aControl", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_control",
                        abi: "C",
                        params: &[Param { name: "aControl", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

