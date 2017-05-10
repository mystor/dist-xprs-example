//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINetworkPredictorVerifier.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINetworkPredictorVerifier",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onPredictPrefetch (in nsIURI uri, in uint32_t status); */
                    Method {
                        name: "onPredictPrefetch",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "status", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void onPredictPreconnect (in nsIURI uri); */
                    Method {
                        name: "onPredictPreconnect",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* void onPredictDNS (in nsIURI uri); */
                    Method {
                        name: "onPredictDNS",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

