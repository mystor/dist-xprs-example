//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationSessionTransportBuilder.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationSessionTransportBuilderListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onSessionTransport (in nsIPresentationSessionTransport transport); */
                    Method {
                        name: "onSessionTransport",
                        abi: "C",
                        params: &[Param { name: "transport", ty: "*const nsIPresentationSessionTransport" }],
                        ret: "nsresult",
                    },

                    /* void onError (in nsresult reason); */
                    Method {
                        name: "onError",
                        abi: "C",
                        params: &[Param { name: "reason", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void sendOffer (in nsIPresentationChannelDescription offer); */
                    Method {
                        name: "sendOffer",
                        abi: "C",
                        params: &[Param { name: "offer", ty: "*const nsIPresentationChannelDescription" }],
                        ret: "nsresult",
                    },

                    /* void sendAnswer (in nsIPresentationChannelDescription answer); */
                    Method {
                        name: "sendAnswer",
                        abi: "C",
                        params: &[Param { name: "answer", ty: "*const nsIPresentationChannelDescription" }],
                        ret: "nsresult",
                    },

                    /* void sendIceCandidate (in DOMString candidate); */
                    Method {
                        name: "sendIceCandidate",
                        abi: "C",
                        params: &[Param { name: "candidate", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void close (in nsresult reason); */
                    Method {
                        name: "close",
                        abi: "C",
                        params: &[Param { name: "reason", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPresentationSessionTransportBuilder",
            base: Some("nsISupports"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "nsIPresentationTransportBuilderConstructor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIPresentationSessionTransportBuilder createTransportBuilder (in uint8_t type); */
                    Method {
                        name: "createTransportBuilder",
                        abi: "C",
                        params: &[Param { name: "type_", ty: "uint8_t" }, Param { name: "_retval", ty: "*mut *const nsIPresentationSessionTransportBuilder" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPresentationTCPSessionTransportBuilder",
            base: Some("nsIPresentationSessionTransportBuilder"),
            methods: Some(&[
                    /* void buildTCPSenderTransport (in nsISocketTransport aTransport, in nsIPresentationSessionTransportBuilderListener aListener); */
                    Method {
                        name: "buildTCPSenderTransport",
                        abi: "C",
                        params: &[Param { name: "aTransport", ty: "*const nsISocketTransport" }, Param { name: "aListener", ty: "*const nsIPresentationSessionTransportBuilderListener" }],
                        ret: "nsresult",
                    },

                    /* void buildTCPReceiverTransport (in nsIPresentationChannelDescription aDescription, in nsIPresentationSessionTransportBuilderListener aListener); */
                    Method {
                        name: "buildTCPReceiverTransport",
                        abi: "C",
                        params: &[Param { name: "aDescription", ty: "*const nsIPresentationChannelDescription" }, Param { name: "aListener", ty: "*const nsIPresentationSessionTransportBuilderListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPresentationDataChannelSessionTransportBuilder",
            base: Some("nsIPresentationSessionTransportBuilder"),
            methods: Some(&[
                    /* void buildDataChannelTransport (in uint8_t aRole, in mozIDOMWindow aWindow, in nsIPresentationSessionTransportBuilderListener aListener); */
                    Method {
                        name: "buildDataChannelTransport",
                        abi: "C",
                        params: &[Param { name: "aRole", ty: "uint8_t" }, Param { name: "aWindow", ty: "*const mozIDOMWindow" }, Param { name: "aListener", ty: "*const nsIPresentationSessionTransportBuilderListener" }],
                        ret: "nsresult",
                    },

                    /* void onOffer (in nsIPresentationChannelDescription offer); */
                    Method {
                        name: "onOffer",
                        abi: "C",
                        params: &[Param { name: "offer", ty: "*const nsIPresentationChannelDescription" }],
                        ret: "nsresult",
                    },

                    /* void onAnswer (in nsIPresentationChannelDescription answer); */
                    Method {
                        name: "onAnswer",
                        abi: "C",
                        params: &[Param { name: "answer", ty: "*const nsIPresentationChannelDescription" }],
                        ret: "nsresult",
                    },

                    /* void onIceCandidate (in DOMString candidate); */
                    Method {
                        name: "onIceCandidate",
                        abi: "C",
                        params: &[Param { name: "candidate", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void notifyDisconnected (in nsresult reason); */
                    Method {
                        name: "notifyDisconnected",
                        abi: "C",
                        params: &[Param { name: "reason", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

