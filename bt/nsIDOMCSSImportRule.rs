//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSImportRule.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMCSSImportRule",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute DOMString href; */
                    Method {
                        name: "get_href",
                        abi: "C",
                        params: &[Param { name: "aHref", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMMediaList media; */
                    Method {
                        name: "get_media",
                        abi: "C",
                        params: &[Param { name: "aMedia", ty: "*mut *const nsIDOMMediaList" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMCSSStyleSheet styleSheet; */
                    Method {
                        name: "get_styleSheet",
                        abi: "C",
                        params: &[Param { name: "aStyleSheet", ty: "*mut *const nsIDOMCSSStyleSheet" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

