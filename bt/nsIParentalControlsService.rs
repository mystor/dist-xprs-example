//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIParentalControlsService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIParentalControlsService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean parentalControlsEnabled; */
                    Method {
                        name: "get_parentalControlsEnabled",
                        abi: "C",
                        params: &[Param { name: "aParentalControlsEnabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean blockFileDownloadsEnabled; */
                    Method {
                        name: "get_blockFileDownloadsEnabled",
                        abi: "C",
                        params: &[Param { name: "aBlockFileDownloadsEnabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isAllowed (in short aAction, [optional] in nsIURI aUri); */
                    Method {
                        name: "isAllowed",
                        abi: "C",
                        params: &[Param { name: "aAction", ty: "libc::int16_t" }, Param { name: "aUri", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean requestURIOverride (in nsIURI aTarget, [optional] in nsIInterfaceRequestor aWindowContext); */
                    Method {
                        name: "requestURIOverride",
                        abi: "C",
                        params: &[Param { name: "aTarget", ty: "*const nsIURI" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean requestURIOverrides (in nsIArray aTargets, [optional] in nsIInterfaceRequestor aWindowContext); */
                    Method {
                        name: "requestURIOverrides",
                        abi: "C",
                        params: &[Param { name: "aTargets", ty: "*const nsIArray" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean loggingEnabled; */
                    Method {
                        name: "get_loggingEnabled",
                        abi: "C",
                        params: &[Param { name: "aLoggingEnabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void log (in short aEntryType, in boolean aFlag, in nsIURI aSource, [optional] in nsIFile aTarget); */
                    Method {
                        name: "log",
                        abi: "C",
                        params: &[Param { name: "aEntryType", ty: "libc::int16_t" }, Param { name: "aFlag", ty: "bool" }, Param { name: "aSource", ty: "*const nsIURI" }, Param { name: "aTarget", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

