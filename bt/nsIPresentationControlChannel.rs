//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationControlChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationChannelDescription",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute uint8_t type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut uint8_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIArray tcpAddress; */
                    Method {
                        name: "get_tcpAddress",
                        abi: "C",
                        params: &[Param { name: "aTcpAddress", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint16_t tcpPort; */
                    Method {
                        name: "get_tcpPort",
                        abi: "C",
                        params: &[Param { name: "aTcpPort", ty: "*mut uint16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString dataChannelSDP; */
                    Method {
                        name: "get_dataChannelSDP",
                        abi: "C",
                        params: &[Param { name: "aDataChannelSDP", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPresentationControlChannelListener",
            base: Some("nsISupports"),
            methods: Some(&[
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

                    /* void notifyConnected (); */
                    Method {
                        name: "notifyConnected",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void notifyDisconnected (in nsresult reason); */
                    Method {
                        name: "notifyDisconnected",
                        abi: "C",
                        params: &[Param { name: "reason", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void notifyReconnected (); */
                    Method {
                        name: "notifyReconnected",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPresentationControlChannel",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsIPresentationControlChannelListener listener; */
                    Method {
                        name: "get_listener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*mut *const nsIPresentationControlChannelListener" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_listener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIPresentationControlChannelListener" }],
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

                    /* void launch (in DOMString presentationId, in DOMString url); */
                    Method {
                        name: "launch",
                        abi: "C",
                        params: &[Param { name: "presentationId", ty: "*const nsAString" }, Param { name: "url", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void terminate (in DOMString presentationId); */
                    Method {
                        name: "terminate",
                        abi: "C",
                        params: &[Param { name: "presentationId", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void disconnect (in nsresult reason); */
                    Method {
                        name: "disconnect",
                        abi: "C",
                        params: &[Param { name: "reason", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void reconnect (in DOMString presentationId, in DOMString url); */
                    Method {
                        name: "reconnect",
                        abi: "C",
                        params: &[Param { name: "presentationId", ty: "*const nsAString" }, Param { name: "url", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

