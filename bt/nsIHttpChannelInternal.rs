//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpChannelInternal.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpUpgradeListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [must_use] void onTransportAvailable (in nsISocketTransport aTransport, in nsIAsyncInputStream aSocketIn, in nsIAsyncOutputStream aSocketOut); */
                    Method {
                        name: "onTransportAvailable",
                        abi: "C",
                        params: &[Param { name: "aTransport", ty: "*const nsISocketTransport" }, Param { name: "aSocketIn", ty: "*const nsIAsyncInputStream" }, Param { name: "aSocketOut", ty: "*const nsIAsyncOutputStream" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIHttpChannelInternal",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

