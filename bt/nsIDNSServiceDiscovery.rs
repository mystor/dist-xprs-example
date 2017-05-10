//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDNSServiceDiscovery.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDNSServiceInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute AUTF8String host; */
                    Method {
                        name: "get_host",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_host",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String address; */
                    Method {
                        name: "get_address",
                        abi: "C",
                        params: &[Param { name: "aAddress", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_address",
                        abi: "C",
                        params: &[Param { name: "aAddress", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned short port; */
                    Method {
                        name: "get_port",
                        abi: "C",
                        params: &[Param { name: "aPort", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_port",
                        abi: "C",
                        params: &[Param { name: "aPort", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String serviceName; */
                    Method {
                        name: "get_serviceName",
                        abi: "C",
                        params: &[Param { name: "aServiceName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_serviceName",
                        abi: "C",
                        params: &[Param { name: "aServiceName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String serviceType; */
                    Method {
                        name: "get_serviceType",
                        abi: "C",
                        params: &[Param { name: "aServiceType", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_serviceType",
                        abi: "C",
                        params: &[Param { name: "aServiceType", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String domainName; */
                    Method {
                        name: "get_domainName",
                        abi: "C",
                        params: &[Param { name: "aDomainName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_domainName",
                        abi: "C",
                        params: &[Param { name: "aDomainName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIPropertyBag2 attributes; */
                    Method {
                        name: "get_attributes",
                        abi: "C",
                        params: &[Param { name: "aAttributes", ty: "*mut *const nsIPropertyBag2" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_attributes",
                        abi: "C",
                        params: &[Param { name: "aAttributes", ty: "*const nsIPropertyBag2" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDNSServiceDiscoveryListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onDiscoveryStarted (in AUTF8String aServiceType); */
                    Method {
                        name: "onDiscoveryStarted",
                        abi: "C",
                        params: &[Param { name: "aServiceType", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void onDiscoveryStopped (in AUTF8String aServiceType); */
                    Method {
                        name: "onDiscoveryStopped",
                        abi: "C",
                        params: &[Param { name: "aServiceType", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void onServiceFound (in nsIDNSServiceInfo aServiceInfo); */
                    Method {
                        name: "onServiceFound",
                        abi: "C",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }],
                        ret: "nsresult",
                    },

                    /* void onServiceLost (in nsIDNSServiceInfo aServiceInfo); */
                    Method {
                        name: "onServiceLost",
                        abi: "C",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }],
                        ret: "nsresult",
                    },

                    /* void onStartDiscoveryFailed (in AUTF8String aServiceType, in long aErrorCode); */
                    Method {
                        name: "onStartDiscoveryFailed",
                        abi: "C",
                        params: &[Param { name: "aServiceType", ty: "*const nsACString" }, Param { name: "aErrorCode", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void onStopDiscoveryFailed (in AUTF8String aServiceType, in long aErrorCode); */
                    Method {
                        name: "onStopDiscoveryFailed",
                        abi: "C",
                        params: &[Param { name: "aServiceType", ty: "*const nsACString" }, Param { name: "aErrorCode", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDNSRegistrationListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onServiceRegistered (in nsIDNSServiceInfo aServiceInfo); */
                    Method {
                        name: "onServiceRegistered",
                        abi: "C",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }],
                        ret: "nsresult",
                    },

                    /* void onServiceUnregistered (in nsIDNSServiceInfo aServiceInfo); */
                    Method {
                        name: "onServiceUnregistered",
                        abi: "C",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }],
                        ret: "nsresult",
                    },

                    /* void onRegistrationFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
                    Method {
                        name: "onRegistrationFailed",
                        abi: "C",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }, Param { name: "aErrorCode", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void onUnregistrationFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
                    Method {
                        name: "onUnregistrationFailed",
                        abi: "C",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }, Param { name: "aErrorCode", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDNSServiceResolveListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onServiceResolved (in nsIDNSServiceInfo aServiceInfo); */
                    Method {
                        name: "onServiceResolved",
                        abi: "C",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }],
                        ret: "nsresult",
                    },

                    /* void onResolveFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
                    Method {
                        name: "onResolveFailed",
                        abi: "C",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }, Param { name: "aErrorCode", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDNSServiceDiscovery",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsICancelable startDiscovery (in AUTF8String aServiceType, in nsIDNSServiceDiscoveryListener aListener); */
                    Method {
                        name: "startDiscovery",
                        abi: "C",
                        params: &[Param { name: "aServiceType", ty: "*const nsACString" }, Param { name: "aListener", ty: "*const nsIDNSServiceDiscoveryListener" }, Param { name: "_retval", ty: "*mut *const nsICancelable" }],
                        ret: "nsresult",
                    },

                    /* nsICancelable registerService (in nsIDNSServiceInfo aServiceInfo, in nsIDNSRegistrationListener aListener); */
                    Method {
                        name: "registerService",
                        abi: "C",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }, Param { name: "aListener", ty: "*const nsIDNSRegistrationListener" }, Param { name: "_retval", ty: "*mut *const nsICancelable" }],
                        ret: "nsresult",
                    },

                    /* void resolveService (in nsIDNSServiceInfo aServiceInfo, in nsIDNSServiceResolveListener aListener); */
                    Method {
                        name: "resolveService",
                        abi: "C",
                        params: &[Param { name: "aServiceInfo", ty: "*const nsIDNSServiceInfo" }, Param { name: "aListener", ty: "*const nsIDNSServiceResolveListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

