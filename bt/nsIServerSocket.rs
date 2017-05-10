//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIServerSocket.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIServerSocket",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIServerSocketListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onSocketAccepted (in nsIServerSocket aServ, in nsISocketTransport aTransport); */
                    Method {
                        name: "onSocketAccepted",
                        abi: "C",
                        params: &[Param { name: "aServ", ty: "*const nsIServerSocket" }, Param { name: "aTransport", ty: "*const nsISocketTransport" }],
                        ret: "nsresult",
                    },

                    /* void onStopListening (in nsIServerSocket aServ, in nsresult aStatus); */
                    Method {
                        name: "onStopListening",
                        abi: "C",
                        params: &[Param { name: "aServ", ty: "*const nsIServerSocket" }, Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

