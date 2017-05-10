//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationSessionTransport.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationSessionTransportCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void notifyTransportReady (); */
                    Method {
                        name: "notifyTransportReady",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void notifyTransportClosed (in nsresult reason); */
                    Method {
                        name: "notifyTransportClosed",
                        abi: "C",
                        params: &[Param { name: "reason", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void notifyData (in ACString data, in boolean isBinary); */
                    Method {
                        name: "notifyData",
                        abi: "C",
                        params: &[Param { name: "data", ty: "*const nsACString" }, Param { name: "isBinary", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPresentationSessionTransport",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsIPresentationSessionTransportCallback callback; */
                    Method {
                        name: "get_callback",
                        abi: "C",
                        params: &[Param { name: "aCallback", ty: "*mut *const nsIPresentationSessionTransportCallback" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_callback",
                        abi: "C",
                        params: &[Param { name: "aCallback", ty: "*const nsIPresentationSessionTransportCallback" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsINetAddr selfAddress; */
                    Method {
                        name: "get_selfAddress",
                        abi: "C",
                        params: &[Param { name: "aSelfAddress", ty: "*mut *const nsINetAddr" }],
                        ret: "nsresult",
                    },

                    /* void enableDataNotification (); */
                    Method {
                        name: "enableDataNotification",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void send (in DOMString data); */
                    Method {
                        name: "send",
                        abi: "C",
                        params: &[Param { name: "data", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void sendBinaryMsg (in ACString data); */
                    Method {
                        name: "sendBinaryMsg",
                        abi: "C",
                        params: &[Param { name: "data", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void sendBlob (in nsIDOMBlob blob); */
                    Method {
                        name: "sendBlob",
                        abi: "C",
                        params: &[Param { name: "blob", ty: "*const nsIDOMBlob" }],
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


        ]; D}

