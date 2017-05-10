//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBrowserSearchService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISearchSubmission",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIInputStream postData; */
                    Method {
                        name: "get_postData",
                        abi: "C",
                        params: &[Param { name: "aPostData", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI uri; */
                    Method {
                        name: "get_uri",
                        abi: "C",
                        params: &[Param { name: "aUri", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISearchEngine",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsISearchParseSubmissionResult",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsISearchEngine engine; */
                    Method {
                        name: "get_engine",
                        abi: "C",
                        params: &[Param { name: "aEngine", ty: "*mut *const nsISearchEngine" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString terms; */
                    Method {
                        name: "get_terms",
                        abi: "C",
                        params: &[Param { name: "aTerms", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long termsOffset; */
                    Method {
                        name: "get_termsOffset",
                        abi: "C",
                        params: &[Param { name: "aTermsOffset", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long termsLength; */
                    Method {
                        name: "get_termsLength",
                        abi: "C",
                        params: &[Param { name: "aTermsLength", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISearchInstallCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onSuccess (in nsISearchEngine engine); */
                    Method {
                        name: "onSuccess",
                        abi: "C",
                        params: &[Param { name: "engine", ty: "*const nsISearchEngine" }],
                        ret: "nsresult",
                    },

                    /* void onError (in unsigned long errorCode); */
                    Method {
                        name: "onError",
                        abi: "C",
                        params: &[Param { name: "errorCode", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIBrowserSearchInitObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onInitComplete (in nsresult aStatus); */
                    Method {
                        name: "onInitComplete",
                        abi: "C",
                        params: &[Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIBrowserSearchService",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

