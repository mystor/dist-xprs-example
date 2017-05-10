//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSStyleRule.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMCSSStyleRule",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString selectorText; */
                    Method {
                        name: "get_selectorText",
                        abi: "C",
                        params: &[Param { name: "aSelectorText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_selectorText",
                        abi: "C",
                        params: &[Param { name: "aSelectorText", ty: "*const nsAString" }],
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

