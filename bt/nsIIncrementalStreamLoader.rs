//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIncrementalStreamLoader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIncrementalStreamLoaderObserver",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIIncrementalStreamLoader",
            base: Some("nsIStreamListener"),
            methods: Some(&[
                    /* void init (in nsIIncrementalStreamLoaderObserver aObserver); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIIncrementalStreamLoaderObserver" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long numBytesRead; */
                    Method {
                        name: "get_numBytesRead",
                        abi: "C",
                        params: &[Param { name: "aNumBytesRead", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIRequest request; */
                    Method {
                        name: "get_request",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*mut *const nsIRequest" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

