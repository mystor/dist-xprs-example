//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSValue.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMCSSValue",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString cssText; */
                    Method {
                        name: "get_cssText",
                        abi: "C",
                        params: &[Param { name: "aCssText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_cssText",
                        abi: "C",
                        params: &[Param { name: "aCssText", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned short cssValueType; */
                    Method {
                        name: "get_cssValueType",
                        abi: "C",
                        params: &[Param { name: "aCssValueType", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

