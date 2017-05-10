//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDashboard.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "NetDashboardCallback",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIDashboard",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void requestSockets (in NetDashboardCallback cb); */
                    Method {
                        name: "requestSockets",
                        abi: "C",
                        params: &[Param { name: "cb", ty: "*const NetDashboardCallback" }],
                        ret: "nsresult",
                    },

                    /* void requestHttpConnections (in NetDashboardCallback cb); */
                    Method {
                        name: "requestHttpConnections",
                        abi: "C",
                        params: &[Param { name: "cb", ty: "*const NetDashboardCallback" }],
                        ret: "nsresult",
                    },

                    /* void requestWebsocketConnections (in NetDashboardCallback cb); */
                    Method {
                        name: "requestWebsocketConnections",
                        abi: "C",
                        params: &[Param { name: "cb", ty: "*const NetDashboardCallback" }],
                        ret: "nsresult",
                    },

                    /* void requestDNSInfo (in NetDashboardCallback cb); */
                    Method {
                        name: "requestDNSInfo",
                        abi: "C",
                        params: &[Param { name: "cb", ty: "*const NetDashboardCallback" }],
                        ret: "nsresult",
                    },

                    /* void requestConnection (in ACString aHost, in unsigned long aPort, in string aProtocol, in unsigned long aTimeout, in NetDashboardCallback cb); */
                    Method {
                        name: "requestConnection",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*const nsACString" }, Param { name: "aPort", ty: "libc::uint32_t" }, Param { name: "aProtocol", ty: "*const libc::c_char" }, Param { name: "aTimeout", ty: "libc::uint32_t" }, Param { name: "cb", ty: "*const NetDashboardCallback" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean enableLogging; */
                    Method {
                        name: "get_enableLogging",
                        abi: "C",
                        params: &[Param { name: "aEnableLogging", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_enableLogging",
                        abi: "C",
                        params: &[Param { name: "aEnableLogging", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void requestDNSLookup (in ACString aHost, in NetDashboardCallback cb); */
                    Method {
                        name: "requestDNSLookup",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*const nsACString" }, Param { name: "cb", ty: "*const NetDashboardCallback" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getLogPath (); */
                    Method {
                        name: "getLogPath",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

