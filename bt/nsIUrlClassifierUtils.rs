//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUrlClassifierUtils.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUrlClassifierParseFindFullHashCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onCompleteHashFound (in ACString aCompleteHash, in ACString aTableNames, in unsigned long aPerHashCacheDuration); */
                    Method {
                        name: "onCompleteHashFound",
                        abi: "C",
                        params: &[Param { name: "aCompleteHash", ty: "*const nsACString" }, Param { name: "aTableNames", ty: "*const nsACString" }, Param { name: "aPerHashCacheDuration", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void onResponseParsed (in unsigned long aMinWaitDuration, in unsigned long aNegCacheDuration); */
                    Method {
                        name: "onResponseParsed",
                        abi: "C",
                        params: &[Param { name: "aMinWaitDuration", ty: "libc::uint32_t" }, Param { name: "aNegCacheDuration", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIUrlClassifierUtils",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

