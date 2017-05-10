//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFeedResultListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handleResult (in nsIFeedResult result); */
                    Method {
                        name: "handleResult",
                        abi: "C",
                        params: &[Param { name: "result", ty: "*const nsIFeedResult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIFeedProgressListener",
            base: Some("nsIFeedResultListener"),
            methods: Some(&[
                    /* void reportError (in AString errorText, in long lineNumber, in boolean bozo); */
                    Method {
                        name: "reportError",
                        abi: "C",
                        params: &[Param { name: "errorText", ty: "*const nsAString" }, Param { name: "lineNumber", ty: "libc::int32_t" }, Param { name: "bozo", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void handleStartFeed (in nsIFeedResult result); */
                    Method {
                        name: "handleStartFeed",
                        abi: "C",
                        params: &[Param { name: "result", ty: "*const nsIFeedResult" }],
                        ret: "nsresult",
                    },

                    /* void handleFeedAtFirstEntry (in nsIFeedResult result); */
                    Method {
                        name: "handleFeedAtFirstEntry",
                        abi: "C",
                        params: &[Param { name: "result", ty: "*const nsIFeedResult" }],
                        ret: "nsresult",
                    },

                    /* void handleEntry (in nsIFeedEntry entry, in nsIFeedResult result); */
                    Method {
                        name: "handleEntry",
                        abi: "C",
                        params: &[Param { name: "entry", ty: "*const nsIFeedEntry" }, Param { name: "result", ty: "*const nsIFeedResult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

