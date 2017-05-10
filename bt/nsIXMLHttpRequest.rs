//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXMLHttpRequest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXMLHttpRequestEventTarget",
            base: Some("nsIDOMEventTarget"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "nsIXMLHttpRequestUpload",
            base: Some("nsIXMLHttpRequestEventTarget"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "nsIXMLHttpRequest",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIXHRSendable",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void getSendInfo (out nsIInputStream body, out uint64_t contentLength, out ACString contentTypeWithCharset, out ACString charset); */
                    Method {
                        name: "getSendInfo",
                        abi: "C",
                        params: &[Param { name: "body", ty: "*mut *const nsIInputStream" }, Param { name: "contentLength", ty: "*mut uint64_t" }, Param { name: "contentTypeWithCharset", ty: "*mut nsACString" }, Param { name: "charset", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIJSXMLHttpRequest",
            base: Some("nsISupports"),
            methods: Some(&[
                    ]),
        },


        ]; D}

