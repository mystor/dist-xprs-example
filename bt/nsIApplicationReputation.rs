//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIApplicationReputation.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIApplicationReputationService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void queryReputation (in nsIApplicationReputationQuery aQuery, in nsIApplicationReputationCallback aCallback); */
                    Method {
                        name: "queryReputation",
                        abi: "C",
                        params: &[Param { name: "aQuery", ty: "*const nsIApplicationReputationQuery" }, Param { name: "aCallback", ty: "*const nsIApplicationReputationCallback" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIApplicationReputationQuery",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIURI sourceURI; */
                    Method {
                        name: "get_sourceURI",
                        abi: "C",
                        params: &[Param { name: "aSourceURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI referrerURI; */
                    Method {
                        name: "get_referrerURI",
                        abi: "C",
                        params: &[Param { name: "aReferrerURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString suggestedFileName; */
                    Method {
                        name: "get_suggestedFileName",
                        abi: "C",
                        params: &[Param { name: "aSuggestedFileName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long fileSize; */
                    Method {
                        name: "get_fileSize",
                        abi: "C",
                        params: &[Param { name: "aFileSize", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString sha256Hash; */
                    Method {
                        name: "get_sha256Hash",
                        abi: "C",
                        params: &[Param { name: "aSha256Hash", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIArray signatureInfo; */
                    Method {
                        name: "get_signatureInfo",
                        abi: "C",
                        params: &[Param { name: "aSignatureInfo", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIArray redirects; */
                    Method {
                        name: "get_redirects",
                        abi: "C",
                        params: &[Param { name: "aRedirects", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIApplicationReputationCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onComplete (in bool aShouldBlock, in nsresult aStatus, in unsigned long aVerdict); */
                    Method {
                        name: "onComplete",
                        abi: "C",
                        params: &[Param { name: "aShouldBlock", ty: "bool" }, Param { name: "aStatus", ty: "nsresult" }, Param { name: "aVerdict", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

