//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpChannel",
            base: Some("nsIChannel"),
            methods: Some(&[
                    /* [must_use] attribute ACString requestMethod; */
                    Method {
                        name: "get_requestMethod",
                        abi: "C",
                        params: &[Param { name: "aRequestMethod", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_requestMethod",
                        abi: "C",
                        params: &[Param { name: "aRequestMethod", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute nsIURI referrer; */
                    Method {
                        name: "get_referrer",
                        abi: "C",
                        params: &[Param { name: "aReferrer", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_referrer",
                        abi: "C",
                        params: &[Param { name: "aReferrer", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute unsigned long referrerPolicy; */
                    Method {
                        name: "get_referrerPolicy",
                        abi: "C",
                        params: &[Param { name: "aReferrerPolicy", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void setReferrerWithPolicy (in nsIURI referrer, in unsigned long referrerPolicy); */
                    Method {
                        name: "setReferrerWithPolicy",
                        abi: "C",
                        params: &[Param { name: "referrer", ty: "*const nsIURI" }, Param { name: "referrerPolicy", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute ACString protocolVersion; */
                    Method {
                        name: "get_protocolVersion",
                        abi: "C",
                        params: &[Param { name: "aProtocolVersion", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute uint64_t transferSize; */
                    Method {
                        name: "get_transferSize",
                        abi: "C",
                        params: &[Param { name: "aTransferSize", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute uint64_t decodedBodySize; */
                    Method {
                        name: "get_decodedBodySize",
                        abi: "C",
                        params: &[Param { name: "aDecodedBodySize", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute uint64_t encodedBodySize; */
                    Method {
                        name: "get_encodedBodySize",
                        abi: "C",
                        params: &[Param { name: "aEncodedBodySize", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] ACString getRequestHeader (in ACString aHeader); */
                    Method {
                        name: "getRequestHeader",
                        abi: "C",
                        params: &[Param { name: "aHeader", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void setRequestHeader (in ACString aHeader, in ACString aValue, in boolean aMerge); */
                    Method {
                        name: "setRequestHeader",
                        abi: "C",
                        params: &[Param { name: "aHeader", ty: "*const nsACString" }, Param { name: "aValue", ty: "*const nsACString" }, Param { name: "aMerge", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void setEmptyRequestHeader (in ACString aHeader); */
                    Method {
                        name: "setEmptyRequestHeader",
                        abi: "C",
                        params: &[Param { name: "aHeader", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void visitRequestHeaders (in nsIHttpHeaderVisitor aVisitor); */
                    Method {
                        name: "visitRequestHeaders",
                        abi: "C",
                        params: &[Param { name: "aVisitor", ty: "*const nsIHttpHeaderVisitor" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void visitNonDefaultRequestHeaders (in nsIHttpHeaderVisitor aVisitor); */
                    Method {
                        name: "visitNonDefaultRequestHeaders",
                        abi: "C",
                        params: &[Param { name: "aVisitor", ty: "*const nsIHttpHeaderVisitor" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute boolean allowPipelining; */
                    Method {
                        name: "get_allowPipelining",
                        abi: "C",
                        params: &[Param { name: "aAllowPipelining", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_allowPipelining",
                        abi: "C",
                        params: &[Param { name: "aAllowPipelining", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute boolean allowSTS; */
                    Method {
                        name: "get_allowSTS",
                        abi: "C",
                        params: &[Param { name: "aAllowSTS", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_allowSTS",
                        abi: "C",
                        params: &[Param { name: "aAllowSTS", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute unsigned long redirectionLimit; */
                    Method {
                        name: "get_redirectionLimit",
                        abi: "C",
                        params: &[Param { name: "aRedirectionLimit", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_redirectionLimit",
                        abi: "C",
                        params: &[Param { name: "aRedirectionLimit", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute unsigned long responseStatus; */
                    Method {
                        name: "get_responseStatus",
                        abi: "C",
                        params: &[Param { name: "aResponseStatus", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute ACString responseStatusText; */
                    Method {
                        name: "get_responseStatusText",
                        abi: "C",
                        params: &[Param { name: "aResponseStatusText", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute boolean requestSucceeded; */
                    Method {
                        name: "get_requestSucceeded",
                        abi: "C",
                        params: &[Param { name: "aRequestSucceeded", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute boolean isMainDocumentChannel; */
                    Method {
                        name: "get_isMainDocumentChannel",
                        abi: "C",
                        params: &[Param { name: "aIsMainDocumentChannel", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_isMainDocumentChannel",
                        abi: "C",
                        params: &[Param { name: "aIsMainDocumentChannel", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] ACString getResponseHeader (in ACString header); */
                    Method {
                        name: "getResponseHeader",
                        abi: "C",
                        params: &[Param { name: "header", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void setResponseHeader (in ACString header, in ACString value, in boolean merge); */
                    Method {
                        name: "setResponseHeader",
                        abi: "C",
                        params: &[Param { name: "header", ty: "*const nsACString" }, Param { name: "value", ty: "*const nsACString" }, Param { name: "merge", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void visitResponseHeaders (in nsIHttpHeaderVisitor aVisitor); */
                    Method {
                        name: "visitResponseHeaders",
                        abi: "C",
                        params: &[Param { name: "aVisitor", ty: "*const nsIHttpHeaderVisitor" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void getOriginalResponseHeader (in ACString aHeader, in nsIHttpHeaderVisitor aVisitor); */
                    Method {
                        name: "getOriginalResponseHeader",
                        abi: "C",
                        params: &[Param { name: "aHeader", ty: "*const nsACString" }, Param { name: "aVisitor", ty: "*const nsIHttpHeaderVisitor" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void visitOriginalResponseHeaders (in nsIHttpHeaderVisitor aVisitor); */
                    Method {
                        name: "visitOriginalResponseHeaders",
                        abi: "C",
                        params: &[Param { name: "aVisitor", ty: "*const nsIHttpHeaderVisitor" }],
                        ret: "nsresult",
                    },

                    /* [must_use] boolean isNoStoreResponse (); */
                    Method {
                        name: "isNoStoreResponse",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] boolean isNoCacheResponse (); */
                    Method {
                        name: "isNoCacheResponse",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] boolean isPrivateResponse (); */
                    Method {
                        name: "isPrivateResponse",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void redirectTo (in nsIURI aTargetURI); */
                    Method {
                        name: "redirectTo",
                        abi: "C",
                        params: &[Param { name: "aTargetURI", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* [must_use,noscript] attribute uint64_t requestContextID; */
                    Method {
                        name: "get_requestContextID",
                        abi: "C",
                        params: &[Param { name: "aRequestContextID", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_requestContextID",
                        abi: "C",
                        params: &[Param { name: "aRequestContextID", ty: "uint64_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute uint64_t channelId; */
                    Method {
                        name: "get_channelId",
                        abi: "C",
                        params: &[Param { name: "aChannelId", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_channelId",
                        abi: "C",
                        params: &[Param { name: "aChannelId", ty: "uint64_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute uint64_t topLevelContentWindowId; */
                    Method {
                        name: "get_topLevelContentWindowId",
                        abi: "C",
                        params: &[Param { name: "aTopLevelContentWindowId", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_topLevelContentWindowId",
                        abi: "C",
                        params: &[Param { name: "aTopLevelContentWindowId", ty: "uint64_t" }],
                        ret: "nsresult",
                    },

                    /* [infallible] readonly attribute boolean isTrackingResource; */
                    Method {
                        name: "get_isTrackingResource",
                        abi: "C",
                        params: &[Param { name: "aIsTrackingResource", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute uint64_t topLevelOuterContentWindowId; */
                    Method {
                        name: "get_topLevelOuterContentWindowId",
                        abi: "C",
                        params: &[Param { name: "aTopLevelOuterContentWindowId", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_topLevelOuterContentWindowId",
                        abi: "C",
                        params: &[Param { name: "aTopLevelOuterContentWindowId", ty: "uint64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

