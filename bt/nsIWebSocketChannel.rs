//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebSocketChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebSocketChannel",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [must_use] readonly attribute nsIURI originalURI; */
                    Method {
                        name: "get_originalURI",
                        abi: "C",
                        params: &[Param { name: "aOriginalURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute nsIURI URI; */
                    Method {
                        name: "get_URI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute nsIInterfaceRequestor notificationCallbacks; */
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

                    /* [must_use] readonly attribute nsISupports securityInfo; */
                    Method {
                        name: "get_securityInfo",
                        abi: "C",
                        params: &[Param { name: "aSecurityInfo", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute nsILoadGroup loadGroup; */
                    Method {
                        name: "get_loadGroup",
                        abi: "C",
                        params: &[Param { name: "aLoadGroup", ty: "*mut *const nsILoadGroup" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_loadGroup",
                        abi: "C",
                        params: &[Param { name: "aLoadGroup", ty: "*const nsILoadGroup" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute nsILoadInfo loadInfo; */
                    Method {
                        name: "get_loadInfo",
                        abi: "C",
                        params: &[Param { name: "aLoadInfo", ty: "*mut *const nsILoadInfo" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_loadInfo",
                        abi: "C",
                        params: &[Param { name: "aLoadInfo", ty: "*const nsILoadInfo" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute ACString protocol; */
                    Method {
                        name: "get_protocol",
                        abi: "C",
                        params: &[Param { name: "aProtocol", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_protocol",
                        abi: "C",
                        params: &[Param { name: "aProtocol", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute ACString extensions; */
                    Method {
                        name: "get_extensions",
                        abi: "C",
                        params: &[Param { name: "aExtensions", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void initLoadInfo (in nsIDOMNode aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in unsigned long aContentPolicyType); */
                    Method {
                        name: "initLoadInfo",
                        abi: "C",
                        params: &[Param { name: "aLoadingNode", ty: "*const nsIDOMNode" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aSecurityFlags", ty: "libc::uint32_t" }, Param { name: "aContentPolicyType", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void asyncOpen (in nsIURI aURI, in ACString aOrigin, in unsigned long long aInnerWindowID, in nsIWebSocketListener aListener, in nsISupports aContext); */
                    Method {
                        name: "asyncOpen",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aOrigin", ty: "*const nsACString" }, Param { name: "aInnerWindowID", ty: "libc::uint64_t" }, Param { name: "aListener", ty: "*const nsIWebSocketListener" }, Param { name: "aContext", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void close (in unsigned short aCode, in AUTF8String aReason); */
                    Method {
                        name: "close",
                        abi: "C",
                        params: &[Param { name: "aCode", ty: "libc::uint16_t" }, Param { name: "aReason", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void sendMsg (in AUTF8String aMsg); */
                    Method {
                        name: "sendMsg",
                        abi: "C",
                        params: &[Param { name: "aMsg", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void sendBinaryMsg (in ACString aMsg); */
                    Method {
                        name: "sendBinaryMsg",
                        abi: "C",
                        params: &[Param { name: "aMsg", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void sendBinaryStream (in nsIInputStream aStream, in unsigned long length); */
                    Method {
                        name: "sendBinaryStream",
                        abi: "C",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "length", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute unsigned long pingInterval; */
                    Method {
                        name: "get_pingInterval",
                        abi: "C",
                        params: &[Param { name: "aPingInterval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_pingInterval",
                        abi: "C",
                        params: &[Param { name: "aPingInterval", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute unsigned long pingTimeout; */
                    Method {
                        name: "get_pingTimeout",
                        abi: "C",
                        params: &[Param { name: "aPingTimeout", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_pingTimeout",
                        abi: "C",
                        params: &[Param { name: "aPingTimeout", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute unsigned long serial; */
                    Method {
                        name: "get_serial",
                        abi: "C",
                        params: &[Param { name: "aSerial", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_serial",
                        abi: "C",
                        params: &[Param { name: "aSerial", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void setServerParameters (in nsITransportProvider aProvider, in ACString aNegotiatedExtensions); */
                    Method {
                        name: "setServerParameters",
                        abi: "C",
                        params: &[Param { name: "aProvider", ty: "*const nsITransportProvider" }, Param { name: "aNegotiatedExtensions", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

