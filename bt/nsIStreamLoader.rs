//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamLoader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStreamLoaderObserver",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIStreamLoader",
            base: Some("nsIStreamListener"),
            methods: Some(&[
                    /* void init (in nsIStreamLoaderObserver aStreamObserver, [optional] in nsIRequestObserver aRequestObserver); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aStreamObserver", ty: "*const nsIStreamLoaderObserver" }, Param { name: "aRequestObserver", ty: "*const nsIRequestObserver" }],
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

