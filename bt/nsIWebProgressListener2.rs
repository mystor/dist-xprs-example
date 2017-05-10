//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebProgressListener2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebProgressListener2",
            base: Some("nsIWebProgressListener"),
            methods: Some(&[
                    /* void onProgressChange64 (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in long long aCurSelfProgress, in long long aMaxSelfProgress, in long long aCurTotalProgress, in long long aMaxTotalProgress); */
                    Method {
                        name: "onProgressChange64",
                        abi: "C",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aCurSelfProgress", ty: "libc::int64_t" }, Param { name: "aMaxSelfProgress", ty: "libc::int64_t" }, Param { name: "aCurTotalProgress", ty: "libc::int64_t" }, Param { name: "aMaxTotalProgress", ty: "libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* boolean onRefreshAttempted (in nsIWebProgress aWebProgress, in nsIURI aRefreshURI, in long aMillis, in boolean aSameURI); */
                    Method {
                        name: "onRefreshAttempted",
                        abi: "C",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRefreshURI", ty: "*const nsIURI" }, Param { name: "aMillis", ty: "libc::int32_t" }, Param { name: "aSameURI", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

