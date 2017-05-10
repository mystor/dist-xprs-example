//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIQuotaCallbacks.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIQuotaUsageCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onUsageResult (in nsIQuotaUsageRequest aRequest); */
                    Method {
                        name: "onUsageResult",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const nsIQuotaUsageRequest" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIQuotaCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onComplete (in nsIQuotaRequest aRequest); */
                    Method {
                        name: "onComplete",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const nsIQuotaRequest" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

