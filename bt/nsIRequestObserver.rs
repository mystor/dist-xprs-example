//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRequestObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRequestObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onStartRequest (in nsIRequest aRequest, in nsISupports aContext); */
                    Method {
                        name: "onStartRequest",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aContext", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void onStopRequest (in nsIRequest aRequest, in nsISupports aContext, in nsresult aStatusCode); */
                    Method {
                        name: "onStopRequest",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aStatusCode", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

