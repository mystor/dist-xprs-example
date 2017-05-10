//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebContentConverterRegistrar.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebContentHandlerInfo",
            base: Some("nsIHandlerApp"),
            methods: Some(&[
                    /* readonly attribute AString contentType; */
                    Method {
                        name: "get_contentType",
                        abi: "C",
                        params: &[Param { name: "aContentType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString uri; */
                    Method {
                        name: "get_uri",
                        abi: "C",
                        params: &[Param { name: "aUri", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getHandlerURI (in AString uri); */
                    Method {
                        name: "getHandlerURI",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIWebContentConverterService",
            base: Some("nsIWebContentHandlerRegistrar"),
            methods: None,
        },


        ]; D}

