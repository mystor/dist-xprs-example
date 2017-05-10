//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpAuthenticableChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpAuthenticableChannel",
            base: Some("nsIProxiedChannel"),
            methods: Some(&[
                    /* [must_use] readonly attribute boolean isSSL; */
                    Method {
                        name: "get_isSSL",
                        abi: "C",
                        params: &[Param { name: "aIsSSL", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute boolean proxyMethodIsConnect; */
                    Method {
                        name: "get_proxyMethodIsConnect",
                        abi: "C",
                        params: &[Param { name: "aProxyMethodIsConnect", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void cancel (in nsresult aStatus); */
                    Method {
                        name: "cancel",
                        abi: "C",
                        params: &[Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute nsLoadFlags loadFlags; */
                    Method {
                        name: "get_loadFlags",
                        abi: "C",
                        params: &[Param { name: "aLoadFlags", ty: "*mut nsLoadFlags" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute nsIURI URI; */
                    Method {
                        name: "get_URI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute nsILoadGroup loadGroup; */
                    Method {
                        name: "get_loadGroup",
                        abi: "C",
                        params: &[Param { name: "aLoadGroup", ty: "*mut *const nsILoadGroup" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute nsIInterfaceRequestor notificationCallbacks; */
                    Method {
                        name: "get_notificationCallbacks",
                        abi: "C",
                        params: &[Param { name: "aNotificationCallbacks", ty: "*mut *const nsIInterfaceRequestor" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute ACString requestMethod; */
                    Method {
                        name: "get_requestMethod",
                        abi: "C",
                        params: &[Param { name: "aRequestMethod", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute ACString serverResponseHeader; */
                    Method {
                        name: "get_serverResponseHeader",
                        abi: "C",
                        params: &[Param { name: "aServerResponseHeader", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute ACString proxyChallenges; */
                    Method {
                        name: "get_proxyChallenges",
                        abi: "C",
                        params: &[Param { name: "aProxyChallenges", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute ACString WWWChallenges; */
                    Method {
                        name: "get_WWWChallenges",
                        abi: "C",
                        params: &[Param { name: "aWWWChallenges", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void setProxyCredentials (in ACString credentials); */
                    Method {
                        name: "setProxyCredentials",
                        abi: "C",
                        params: &[Param { name: "credentials", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void setWWWCredentials (in ACString credentials); */
                    Method {
                        name: "setWWWCredentials",
                        abi: "C",
                        params: &[Param { name: "credentials", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void onAuthAvailable (); */
                    Method {
                        name: "onAuthAvailable",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* [must_use] void onAuthCancelled (in boolean userCancel); */
                    Method {
                        name: "onAuthCancelled",
                        abi: "C",
                        params: &[Param { name: "userCancel", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void closeStickyConnection (); */
                    Method {
                        name: "closeStickyConnection",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* [must_use] void forceNoSpdy (); */
                    Method {
                        name: "forceNoSpdy",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void connectionRestartable (in boolean restartable); */
                    Method {
                        name: "connectionRestartable",
                        abi: "C",
                        params: &[Param { name: "restartable", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

