//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSKeyframeRule.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMCSSKeyframeRule",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString keyText; */
                    Method {
                        name: "get_keyText",
                        abi: "C",
                        params: &[Param { name: "aKeyText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_keyText",
                        abi: "C",
                        params: &[Param { name: "aKeyText", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMCSSStyleDeclaration style; */
                    Method {
                        name: "get_style",
                        abi: "C",
                        params: &[Param { name: "aStyle", ty: "*mut *const nsIDOMCSSStyleDeclaration" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

