//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWifiMonitor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWifiMonitor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void startWatching (in nsIWifiListener aListener); */
                    Method {
                        name: "startWatching",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIWifiListener" }],
                        ret: "nsresult",
                    },

                    /* void stopWatching (in nsIWifiListener aListener); */
                    Method {
                        name: "stopWatching",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIWifiListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

