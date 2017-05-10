//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLOListElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLOListElement",
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

                    /* attribute boolean reversed; */
                    Method {
                        name: "get_reversed",
                        abi: "C",
                        params: &[Param { name: "aReversed", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_reversed",
                        abi: "C",
                        params: &[Param { name: "aReversed", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute long start; */
                    Method {
                        name: "get_start",
                        abi: "C",
                        params: &[Param { name: "aStart", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_start",
                        abi: "C",
                        params: &[Param { name: "aStart", ty: "libc::int32_t" }],
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

                    ]),
        },


        ]; D}

