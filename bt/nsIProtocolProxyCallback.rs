//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProtocolProxyCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProtocolProxyCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onProxyAvailable (in nsICancelable aRequest, in nsIChannel aChannel, in nsIProxyInfo aProxyInfo, in nsresult aStatus); */
                    Method {
                        name: "onProxyAvailable",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const nsICancelable" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aProxyInfo", ty: "*const nsIProxyInfo" }, Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

