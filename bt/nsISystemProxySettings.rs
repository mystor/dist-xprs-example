//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISystemProxySettings.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISystemProxySettings",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute bool mainThreadOnly; */
                    Method {
                        name: "get_mainThreadOnly",
                        abi: "C",
                        params: &[Param { name: "aMainThreadOnly", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String PACURI; */
                    Method {
                        name: "get_PACURI",
                        abi: "C",
                        params: &[Param { name: "aPACURI", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getProxyForURI (in AUTF8String testSpec, in AUTF8String testScheme, in AUTF8String testHost, in int32_t testPort); */
                    Method {
                        name: "getProxyForURI",
                        abi: "C",
                        params: &[Param { name: "testSpec", ty: "*const nsACString" }, Param { name: "testScheme", ty: "*const nsACString" }, Param { name: "testHost", ty: "*const nsACString" }, Param { name: "testPort", ty: "int32_t" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

