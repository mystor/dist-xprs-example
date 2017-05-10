//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULDescriptionElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULDescriptionElement",
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

                    /* attribute boolean crop; */
                    Method {
                        name: "get_crop",
                        abi: "C",
                        params: &[Param { name: "aCrop", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_crop",
                        abi: "C",
                        params: &[Param { name: "aCrop", ty: "bool" }],
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

                    ]),
        },


        ]; D}

