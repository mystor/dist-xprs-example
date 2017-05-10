//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLMetaElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLMetaElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString content; */
                    Method {
                        name: "get_content",
                        abi: "C",
                        params: &[Param { name: "aContent", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_content",
                        abi: "C",
                        params: &[Param { name: "aContent", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString httpEquiv; */
                    Method {
                        name: "get_httpEquiv",
                        abi: "C",
                        params: &[Param { name: "aHttpEquiv", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_httpEquiv",
                        abi: "C",
                        params: &[Param { name: "aHttpEquiv", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString scheme; */
                    Method {
                        name: "get_scheme",
                        abi: "C",
                        params: &[Param { name: "aScheme", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_scheme",
                        abi: "C",
                        params: &[Param { name: "aScheme", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

