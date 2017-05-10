//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLScriptElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLScriptElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString src; */
                    Method {
                        name: "get_src",
                        abi: "C",
                        params: &[Param { name: "aSrc", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_src",
                        abi: "C",
                        params: &[Param { name: "aSrc", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean async; */
                    Method {
                        name: "get_async",
                        abi: "C",
                        params: &[Param { name: "aAsync", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_async",
                        abi: "C",
                        params: &[Param { name: "aAsync", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean defer; */
                    Method {
                        name: "get_defer",
                        abi: "C",
                        params: &[Param { name: "aDefer", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_defer",
                        abi: "C",
                        params: &[Param { name: "aDefer", ty: "bool" }],
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

                    /* attribute DOMString charset; */
                    Method {
                        name: "get_charset",
                        abi: "C",
                        params: &[Param { name: "aCharset", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_charset",
                        abi: "C",
                        params: &[Param { name: "aCharset", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString text; */
                    Method {
                        name: "get_text",
                        abi: "C",
                        params: &[Param { name: "aText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_text",
                        abi: "C",
                        params: &[Param { name: "aText", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString htmlFor; */
                    Method {
                        name: "get_htmlFor",
                        abi: "C",
                        params: &[Param { name: "aHtmlFor", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_htmlFor",
                        abi: "C",
                        params: &[Param { name: "aHtmlFor", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString event; */
                    Method {
                        name: "get_event",
                        abi: "C",
                        params: &[Param { name: "aEvent", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_event",
                        abi: "C",
                        params: &[Param { name: "aEvent", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

