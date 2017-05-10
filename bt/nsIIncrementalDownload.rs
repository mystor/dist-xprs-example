//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIncrementalDownload.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIncrementalDownload",
            base: Some("nsIRequest"),
            methods: Some(&[
                    /* void init (in nsIURI uri, in nsIFile destination, in long chunkSize, in long intervalInSeconds); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "destination", ty: "*const nsIFile" }, Param { name: "chunkSize", ty: "libc::int32_t" }, Param { name: "intervalInSeconds", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI URI; */
                    Method {
                        name: "get_URI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI finalURI; */
                    Method {
                        name: "get_finalURI",
                        abi: "C",
                        params: &[Param { name: "aFinalURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIFile destination; */
                    Method {
                        name: "get_destination",
                        abi: "C",
                        params: &[Param { name: "aDestination", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long totalSize; */
                    Method {
                        name: "get_totalSize",
                        abi: "C",
                        params: &[Param { name: "aTotalSize", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long currentSize; */
                    Method {
                        name: "get_currentSize",
                        abi: "C",
                        params: &[Param { name: "aCurrentSize", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* void start (in nsIRequestObserver observer, in nsISupports ctxt); */
                    Method {
                        name: "start",
                        abi: "C",
                        params: &[Param { name: "observer", ty: "*const nsIRequestObserver" }, Param { name: "ctxt", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

