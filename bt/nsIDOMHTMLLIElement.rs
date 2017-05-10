//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLLIElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLLIElement",
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

                    /* attribute long value; */
                    Method {
                        name: "get_value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

