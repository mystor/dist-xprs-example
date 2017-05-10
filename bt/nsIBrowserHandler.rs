//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBrowserHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIBrowserHandler",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute AUTF8String startPage; */
                    Method {
                        name: "get_startPage",
                        abi: "C",
                        params: &[Param { name: "aStartPage", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_startPage",
                        abi: "C",
                        params: &[Param { name: "aStartPage", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String defaultArgs; */
                    Method {
                        name: "get_defaultArgs",
                        abi: "C",
                        params: &[Param { name: "aDefaultArgs", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_defaultArgs",
                        abi: "C",
                        params: &[Param { name: "aDefaultArgs", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getFeatures (in nsICommandLine aCmdLine); */
                    Method {
                        name: "getFeatures",
                        abi: "C",
                        params: &[Param { name: "aCmdLine", ty: "*const nsICommandLine" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

