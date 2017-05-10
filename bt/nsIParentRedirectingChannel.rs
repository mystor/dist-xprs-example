//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIParentRedirectingChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIParentRedirectingChannel",
            base: Some("nsIParentChannel"),
            methods: Some(&[
                    /* void startRedirect (in uint32_t newChannelId, in nsIChannel newChannel, in uint32_t redirectFlags, in nsIAsyncVerifyRedirectCallback callback); */
                    Method {
                        name: "startRedirect",
                        abi: "C",
                        params: &[Param { name: "newChannelId", ty: "uint32_t" }, Param { name: "newChannel", ty: "*const nsIChannel" }, Param { name: "redirectFlags", ty: "uint32_t" }, Param { name: "callback", ty: "*const nsIAsyncVerifyRedirectCallback" }],
                        ret: "nsresult",
                    },

                    /* void completeRedirect (in boolean succeeded); */
                    Method {
                        name: "completeRedirect",
                        abi: "C",
                        params: &[Param { name: "succeeded", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

