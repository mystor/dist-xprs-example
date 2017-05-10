//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIKeygenThread.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIKeygenThread",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void startKeyGeneration (in nsIObserver observer); */
                    Method {
                        name: "startKeyGeneration",
                        abi: "C",
                        params: &[Param { name: "observer", ty: "*const nsIObserver" }],
                        ret: "nsresult",
                    },

                    /* void userCanceled (out boolean threadAlreadyClosedDialog); */
                    Method {
                        name: "userCanceled",
                        abi: "C",
                        params: &[Param { name: "threadAlreadyClosedDialog", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

