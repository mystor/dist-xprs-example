//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLMenuElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLMenuElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute boolean compact; */
                    Method {
                        name: "get_compact",
                        abi: "C",
                        params: &[Param { name: "aCompact", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_compact",
                        abi: "C",
                        params: &[Param { name: "aCompact", ty: "bool" }],
                        ret: "nsresult",
                    },

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

                    ]),
        },


        ]; D}

