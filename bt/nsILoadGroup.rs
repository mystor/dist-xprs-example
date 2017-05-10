//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoadGroup.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILoadGroup",
            base: Some("nsIRequest"),
            methods: Some(&[
                    /* attribute nsIRequestObserver groupObserver; */
                    Method {
                        name: "get_groupObserver",
                        abi: "C",
                        params: &[Param { name: "aGroupObserver", ty: "*mut *const nsIRequestObserver" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_groupObserver",
                        abi: "C",
                        params: &[Param { name: "aGroupObserver", ty: "*const nsIRequestObserver" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIRequest defaultLoadRequest; */
                    Method {
                        name: "get_defaultLoadRequest",
                        abi: "C",
                        params: &[Param { name: "aDefaultLoadRequest", ty: "*mut *const nsIRequest" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_defaultLoadRequest",
                        abi: "C",
                        params: &[Param { name: "aDefaultLoadRequest", ty: "*const nsIRequest" }],
                        ret: "nsresult",
                    },

                    /* void addRequest (in nsIRequest aRequest, in nsISupports aContext); */
                    Method {
                        name: "addRequest",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aContext", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void removeRequest (in nsIRequest aRequest, in nsISupports aContext, in nsresult aStatus); */
                    Method {
                        name: "removeRequest",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsISimpleEnumerator requests; */
                    Method {
                        name: "get_requests",
                        abi: "C",
                        params: &[Param { name: "aRequests", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long activeCount; */
                    Method {
                        name: "get_activeCount",
                        abi: "C",
                        params: &[Param { name: "aActiveCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIInterfaceRequestor notificationCallbacks; */
                    Method {
                        name: "get_notificationCallbacks",
                        abi: "C",
                        params: &[Param { name: "aNotificationCallbacks", ty: "*mut *const nsIInterfaceRequestor" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_notificationCallbacks",
                        abi: "C",
                        params: &[Param { name: "aNotificationCallbacks", ty: "*const nsIInterfaceRequestor" }],
                        ret: "nsresult",
                    },

                    /* [noscript] readonly attribute unsigned long long requestContextID; */
                    Method {
                        name: "get_requestContextID",
                        abi: "C",
                        params: &[Param { name: "aRequestContextID", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    /* attribute nsLoadFlags defaultLoadFlags; */
                    Method {
                        name: "get_defaultLoadFlags",
                        abi: "C",
                        params: &[Param { name: "aDefaultLoadFlags", ty: "*mut nsLoadFlags" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_defaultLoadFlags",
                        abi: "C",
                        params: &[Param { name: "aDefaultLoadFlags", ty: "nsLoadFlags" }],
                        ret: "nsresult",
                    },

                    /* attribute ACString userAgentOverrideCache; */
                    Method {
                        name: "get_userAgentOverrideCache",
                        abi: "C",
                        params: &[Param { name: "aUserAgentOverrideCache", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_userAgentOverrideCache",
                        abi: "C",
                        params: &[Param { name: "aUserAgentOverrideCache", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

