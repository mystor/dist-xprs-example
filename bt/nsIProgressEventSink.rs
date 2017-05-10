//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProgressEventSink.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProgressEventSink",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onProgress (in nsIRequest aRequest, in nsISupports aContext, in long long aProgress, in long long aProgressMax); */
                    Method {
                        name: "onProgress",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aProgress", ty: "libc::int64_t" }, Param { name: "aProgressMax", ty: "libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* void onStatus (in nsIRequest aRequest, in nsISupports aContext, in nsresult aStatus, in wstring aStatusArg); */
                    Method {
                        name: "onStatus",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aStatus", ty: "nsresult" }, Param { name: "aStatusArg", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

