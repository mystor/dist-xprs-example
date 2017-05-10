//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDownloadProgressListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDownloadProgressListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsIDOMDocument document; */
                    Method {
                        name: "get_document",
                        abi: "C",
                        params: &[Param { name: "aDocument", ty: "*mut *const nsIDOMDocument" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_document",
                        abi: "C",
                        params: &[Param { name: "aDocument", ty: "*const nsIDOMDocument" }],
                        ret: "nsresult",
                    },

                    /* void onDownloadStateChange (in short aState, in nsIDownload aDownload); */
                    Method {
                        name: "onDownloadStateChange",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "libc::int16_t" }, Param { name: "aDownload", ty: "*const nsIDownload" }],
                        ret: "nsresult",
                    },

                    /* void onStateChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aStateFlags, in nsresult aStatus, in nsIDownload aDownload); */
                    Method {
                        name: "onStateChange",
                        abi: "C",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aStateFlags", ty: "libc::uint32_t" }, Param { name: "aStatus", ty: "nsresult" }, Param { name: "aDownload", ty: "*const nsIDownload" }],
                        ret: "nsresult",
                    },

                    /* void onProgressChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in long long aCurSelfProgress, in long long aMaxSelfProgress, in long long aCurTotalProgress, in long long aMaxTotalProgress, in nsIDownload aDownload); */
                    Method {
                        name: "onProgressChange",
                        abi: "C",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aCurSelfProgress", ty: "libc::int64_t" }, Param { name: "aMaxSelfProgress", ty: "libc::int64_t" }, Param { name: "aCurTotalProgress", ty: "libc::int64_t" }, Param { name: "aMaxTotalProgress", ty: "libc::int64_t" }, Param { name: "aDownload", ty: "*const nsIDownload" }],
                        ret: "nsresult",
                    },

                    /* void onSecurityChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aState, in nsIDownload aDownload); */
                    Method {
                        name: "onSecurityChange",
                        abi: "C",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aState", ty: "libc::uint32_t" }, Param { name: "aDownload", ty: "*const nsIDownload" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

