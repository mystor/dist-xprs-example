//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIChannelEventSink.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIChannelEventSink",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void asyncOnChannelRedirect (in nsIChannel oldChannel, in nsIChannel newChannel, in unsigned long flags, in nsIAsyncVerifyRedirectCallback callback); */
                    Method {
                        name: "asyncOnChannelRedirect",
                        abi: "C",
                        params: &[Param { name: "oldChannel", ty: "*const nsIChannel" }, Param { name: "newChannel", ty: "*const nsIChannel" }, Param { name: "flags", ty: "libc::uint32_t" }, Param { name: "callback", ty: "*const nsIAsyncVerifyRedirectCallback" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

