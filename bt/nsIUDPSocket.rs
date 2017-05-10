//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUDPSocket.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUDPSocket",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIUDPSocketListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onPacketReceived (in nsIUDPSocket aSocket, in nsIUDPMessage aMessage); */
                    Method {
                        name: "onPacketReceived",
                        abi: "C",
                        params: &[Param { name: "aSocket", ty: "*const nsIUDPSocket" }, Param { name: "aMessage", ty: "*const nsIUDPMessage" }],
                        ret: "nsresult",
                    },

                    /* void onStopListening (in nsIUDPSocket aSocket, in nsresult aStatus); */
                    Method {
                        name: "onStopListening",
                        abi: "C",
                        params: &[Param { name: "aSocket", ty: "*const nsIUDPSocket" }, Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIUDPMessage",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

