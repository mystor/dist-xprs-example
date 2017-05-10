//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLFormElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLFormElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString acceptCharset; */
                    Method {
                        name: "get_acceptCharset",
                        abi: "C",
                        params: &[Param { name: "aAcceptCharset", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_acceptCharset",
                        abi: "C",
                        params: &[Param { name: "aAcceptCharset", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString action; */
                    Method {
                        name: "get_action",
                        abi: "C",
                        params: &[Param { name: "aAction", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_action",
                        abi: "C",
                        params: &[Param { name: "aAction", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString autocomplete; */
                    Method {
                        name: "get_autocomplete",
                        abi: "C",
                        params: &[Param { name: "aAutocomplete", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_autocomplete",
                        abi: "C",
                        params: &[Param { name: "aAutocomplete", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString enctype; */
                    Method {
                        name: "get_enctype",
                        abi: "C",
                        params: &[Param { name: "aEnctype", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_enctype",
                        abi: "C",
                        params: &[Param { name: "aEnctype", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString encoding; */
                    Method {
                        name: "get_encoding",
                        abi: "C",
                        params: &[Param { name: "aEncoding", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_encoding",
                        abi: "C",
                        params: &[Param { name: "aEncoding", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString method; */
                    Method {
                        name: "get_method",
                        abi: "C",
                        params: &[Param { name: "aMethod", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_method",
                        abi: "C",
                        params: &[Param { name: "aMethod", ty: "*const nsAString" }],
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

                    /* attribute boolean noValidate; */
                    Method {
                        name: "get_noValidate",
                        abi: "C",
                        params: &[Param { name: "aNoValidate", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_noValidate",
                        abi: "C",
                        params: &[Param { name: "aNoValidate", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString target; */
                    Method {
                        name: "get_target",
                        abi: "C",
                        params: &[Param { name: "aTarget", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_target",
                        abi: "C",
                        params: &[Param { name: "aTarget", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMHTMLCollection elements; */
                    Method {
                        name: "get_elements",
                        abi: "C",
                        params: &[Param { name: "aElements", ty: "*mut *const nsIDOMHTMLCollection" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void submit (); */
                    Method {
                        name: "submit",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void reset (); */
                    Method {
                        name: "reset",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean checkValidity (); */
                    Method {
                        name: "checkValidity",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

