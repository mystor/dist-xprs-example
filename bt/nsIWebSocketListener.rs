//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebSocketListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebSocketListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [must_use] void onStart (in nsISupports aContext); */
                    Method {
                        name: "onStart",
                        abi: "C",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void onStop (in nsISupports aContext, in nsresult aStatusCode); */
                    Method {
                        name: "onStop",
                        abi: "C",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aStatusCode", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void onMessageAvailable (in nsISupports aContext, in AUTF8String aMsg); */
                    Method {
                        name: "onMessageAvailable",
                        abi: "C",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aMsg", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void onBinaryMessageAvailable (in nsISupports aContext, in ACString aMsg); */
                    Method {
                        name: "onBinaryMessageAvailable",
                        abi: "C",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aMsg", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void onAcknowledge (in nsISupports aContext, in uint32_t aSize); */
                    Method {
                        name: "onAcknowledge",
                        abi: "C",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aSize", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void onServerClose (in nsISupports aContext, in unsigned short aCode, in AUTF8String aReason); */
                    Method {
                        name: "onServerClose",
                        abi: "C",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aCode", ty: "libc::uint16_t" }, Param { name: "aReason", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

