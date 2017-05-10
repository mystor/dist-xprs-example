//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIThrottlingService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIThrottlingService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addChannel (in nsIHttpChannel channel); */
                    Method {
                        name: "addChannel",
                        abi: "C",
                        params: &[Param { name: "channel", ty: "*const nsIHttpChannel" }],
                        ret: "nsresult",
                    },

                    /* void removeChannel (in nsIHttpChannel channel); */
                    Method {
                        name: "removeChannel",
                        abi: "C",
                        params: &[Param { name: "channel", ty: "*const nsIHttpChannel" }],
                        ret: "nsresult",
                    },

                    /* void increasePressure (); */
                    Method {
                        name: "increasePressure",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void decreasePressure (); */
                    Method {
                        name: "decreasePressure",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

