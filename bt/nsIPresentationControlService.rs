//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationControlService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITCPDeviceInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AUTF8String id; */
                    Method {
                        name: "get_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String address; */
                    Method {
                        name: "get_address",
                        abi: "C",
                        params: &[Param { name: "aAddress", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint16_t port; */
                    Method {
                        name: "get_port",
                        abi: "C",
                        params: &[Param { name: "aPort", ty: "*mut uint16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String certFingerprint; */
                    Method {
                        name: "get_certFingerprint",
                        abi: "C",
                        params: &[Param { name: "aCertFingerprint", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPresentationControlServerListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onServerReady (in uint16_t aPort, in AUTF8String aCertFingerprint); */
                    Method {
                        name: "onServerReady",
                        abi: "C",
                        params: &[Param { name: "aPort", ty: "uint16_t" }, Param { name: "aCertFingerprint", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void onServerStopped (in nsresult aResult); */
                    Method {
                        name: "onServerStopped",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void onSessionRequest (in nsITCPDeviceInfo aDeviceInfo, in DOMString aUrl, in DOMString aPresentationId, in nsIPresentationControlChannel aControlChannel); */
                    Method {
                        name: "onSessionRequest",
                        abi: "C",
                        params: &[Param { name: "aDeviceInfo", ty: "*const nsITCPDeviceInfo" }, Param { name: "aUrl", ty: "*const nsAString" }, Param { name: "aPresentationId", ty: "*const nsAString" }, Param { name: "aControlChannel", ty: "*const nsIPresentationControlChannel" }],
                        ret: "nsresult",
                    },

                    /* void onTerminateRequest (in nsITCPDeviceInfo aDeviceInfo, in DOMString aPresentationId, in nsIPresentationControlChannel aControlChannel, in boolean aIsFromReceiver); */
                    Method {
                        name: "onTerminateRequest",
                        abi: "C",
                        params: &[Param { name: "aDeviceInfo", ty: "*const nsITCPDeviceInfo" }, Param { name: "aPresentationId", ty: "*const nsAString" }, Param { name: "aControlChannel", ty: "*const nsIPresentationControlChannel" }, Param { name: "aIsFromReceiver", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void onReconnectRequest (in nsITCPDeviceInfo aDeviceInfo, in DOMString url, in DOMString aPresentationId, in nsIPresentationControlChannel aControlChannel); */
                    Method {
                        name: "onReconnectRequest",
                        abi: "C",
                        params: &[Param { name: "aDeviceInfo", ty: "*const nsITCPDeviceInfo" }, Param { name: "url", ty: "*const nsAString" }, Param { name: "aPresentationId", ty: "*const nsAString" }, Param { name: "aControlChannel", ty: "*const nsIPresentationControlChannel" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPresentationControlService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void startServer (in boolean aEncrypted, [optional] in uint16_t aPort); */
                    Method {
                        name: "startServer",
                        abi: "C",
                        params: &[Param { name: "aEncrypted", ty: "bool" }, Param { name: "aPort", ty: "uint16_t" }],
                        ret: "nsresult",
                    },

                    /* nsIPresentationControlChannel connect (in nsITCPDeviceInfo aDeviceInfo); */
                    Method {
                        name: "connect",
                        abi: "C",
                        params: &[Param { name: "aDeviceInfo", ty: "*const nsITCPDeviceInfo" }, Param { name: "_retval", ty: "*mut *const nsIPresentationControlChannel" }],
                        ret: "nsresult",
                    },

                    /* boolean isCompatibleServer (in uint32_t aVersion); */
                    Method {
                        name: "isCompatibleServer",
                        abi: "C",
                        params: &[Param { name: "aVersion", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void close (); */
                    Method {
                        name: "close",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint16_t port; */
                    Method {
                        name: "get_port",
                        abi: "C",
                        params: &[Param { name: "aPort", ty: "*mut uint16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t version; */
                    Method {
                        name: "get_version",
                        abi: "C",
                        params: &[Param { name: "aVersion", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String id; */
                    Method {
                        name: "get_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String certFingerprint; */
                    Method {
                        name: "get_certFingerprint",
                        abi: "C",
                        params: &[Param { name: "aCertFingerprint", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_certFingerprint",
                        abi: "C",
                        params: &[Param { name: "aCertFingerprint", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIPresentationControlServerListener listener; */
                    Method {
                        name: "get_listener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*mut *const nsIPresentationControlServerListener" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_listener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIPresentationControlServerListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

