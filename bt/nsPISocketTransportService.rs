//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsPISocketTransportService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsPISocketTransportService",
            base: Some("nsIRoutedSocketTransportService"),
            methods: Some(&[
                    /* void init (); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void shutdown (in bool aXpcomShutdown); */
                    Method {
                        name: "shutdown",
                        abi: "C",
                        params: &[Param { name: "aXpcomShutdown", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long sendBufferSize; */
                    Method {
                        name: "get_sendBufferSize",
                        abi: "C",
                        params: &[Param { name: "aSendBufferSize", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean offline; */
                    Method {
                        name: "get_offline",
                        abi: "C",
                        params: &[Param { name: "aOffline", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_offline",
                        abi: "C",
                        params: &[Param { name: "aOffline", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long keepaliveIdleTime; */
                    Method {
                        name: "get_keepaliveIdleTime",
                        abi: "C",
                        params: &[Param { name: "aKeepaliveIdleTime", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long keepaliveRetryInterval; */
                    Method {
                        name: "get_keepaliveRetryInterval",
                        abi: "C",
                        params: &[Param { name: "aKeepaliveRetryInterval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long keepaliveProbeCount; */
                    Method {
                        name: "get_keepaliveProbeCount",
                        abi: "C",
                        params: &[Param { name: "aKeepaliveProbeCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

