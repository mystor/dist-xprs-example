//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedResultService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFeedResultService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute boolean forcePreviewPage; */
                    Method {
                        name: "get_forcePreviewPage",
                        abi: "C",
                        params: &[Param { name: "aForcePreviewPage", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_forcePreviewPage",
                        abi: "C",
                        params: &[Param { name: "aForcePreviewPage", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void addToClientReader (in AUTF8String uri, in AString title, in AString subtitle, in unsigned long feedType, [optional] in AString feedReader); */
                    Method {
                        name: "addToClientReader",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsACString" }, Param { name: "title", ty: "*const nsAString" }, Param { name: "subtitle", ty: "*const nsAString" }, Param { name: "feedType", ty: "libc::uint32_t" }, Param { name: "feedReader", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void addFeedResult (in nsIFeedResult feedResult); */
                    Method {
                        name: "addFeedResult",
                        abi: "C",
                        params: &[Param { name: "feedResult", ty: "*const nsIFeedResult" }],
                        ret: "nsresult",
                    },

                    /* nsIFeedResult getFeedResult (in nsIURI uri); */
                    Method {
                        name: "getFeedResult",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsIFeedResult" }],
                        ret: "nsresult",
                    },

                    /* void removeFeedResult (in nsIURI uri); */
                    Method {
                        name: "removeFeedResult",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

