//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebProgressListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebProgressListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onStateChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aStateFlags, in nsresult aStatus); */
                    Method {
                        name: "onStateChange",
                        abi: "C",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aStateFlags", ty: "libc::uint32_t" }, Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void onProgressChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in long aCurSelfProgress, in long aMaxSelfProgress, in long aCurTotalProgress, in long aMaxTotalProgress); */
                    Method {
                        name: "onProgressChange",
                        abi: "C",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aCurSelfProgress", ty: "libc::int32_t" }, Param { name: "aMaxSelfProgress", ty: "libc::int32_t" }, Param { name: "aCurTotalProgress", ty: "libc::int32_t" }, Param { name: "aMaxTotalProgress", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void onLocationChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in nsIURI aLocation, [optional] in unsigned long aFlags); */
                    Method {
                        name: "onLocationChange",
                        abi: "C",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aLocation", ty: "*const nsIURI" }, Param { name: "aFlags", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void onStatusChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in nsresult aStatus, in wstring aMessage); */
                    Method {
                        name: "onStatusChange",
                        abi: "C",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aStatus", ty: "nsresult" }, Param { name: "aMessage", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void onSecurityChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aState); */
                    Method {
                        name: "onSecurityChange",
                        abi: "C",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aState", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

