//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIAsyncHistory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIVisitInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute long long visitId; */
                    Method {
                        name: "get_visitId",
                        abi: "C",
                        params: &[Param { name: "aVisitId", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute PRTime visitDate; */
                    Method {
                        name: "get_visitDate",
                        abi: "C",
                        params: &[Param { name: "aVisitDate", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long transitionType; */
                    Method {
                        name: "get_transitionType",
                        abi: "C",
                        params: &[Param { name: "aTransitionType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI referrerURI; */
                    Method {
                        name: "get_referrerURI",
                        abi: "C",
                        params: &[Param { name: "aReferrerURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "mozIPlaceInfo",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "mozIVisitInfoCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handleError (in nsresult aResultCode, in mozIPlaceInfo aPlaceInfo); */
                    Method {
                        name: "handleError",
                        abi: "C",
                        params: &[Param { name: "aResultCode", ty: "nsresult" }, Param { name: "aPlaceInfo", ty: "*const mozIPlaceInfo" }],
                        ret: "nsresult",
                    },

                    /* void handleResult (in mozIPlaceInfo aPlaceInfo); */
                    Method {
                        name: "handleResult",
                        abi: "C",
                        params: &[Param { name: "aPlaceInfo", ty: "*const mozIPlaceInfo" }],
                        ret: "nsresult",
                    },

                    /* void handleCompletion (in unsigned long aUpdatedItems); */
                    Method {
                        name: "handleCompletion",
                        abi: "C",
                        params: &[Param { name: "aUpdatedItems", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute bool ignoreResults; */
                    Method {
                        name: "get_ignoreResults",
                        abi: "C",
                        params: &[Param { name: "aIgnoreResults", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute bool ignoreErrors; */
                    Method {
                        name: "get_ignoreErrors",
                        abi: "C",
                        params: &[Param { name: "aIgnoreErrors", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "mozIVisitedStatusCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void isVisited (in nsIURI aURI, in boolean aVisitedStatus); */
                    Method {
                        name: "isVisited",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aVisitedStatus", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "mozIAsyncHistory",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

