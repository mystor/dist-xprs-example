//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISSLStatusProvider.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISSLStatusProvider",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsISSLStatus SSLStatus; */
                    Method {
                        name: "get_SSLStatus",
                        abi: "C",
                        params: &[Param { name: "aSSLStatus", ty: "*mut *const nsISSLStatus" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

