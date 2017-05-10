//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINetworkInfoService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIListNetworkAddressesListener",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIGetHostnameListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onGotHostname (in AUTF8String aHostname); */
                    Method {
                        name: "onGotHostname",
                        abi: "C",
                        params: &[Param { name: "aHostname", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void onGetHostnameFailed (); */
                    Method {
                        name: "onGetHostnameFailed",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINetworkInfoService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void listNetworkAddresses (in nsIListNetworkAddressesListener aListener); */
                    Method {
                        name: "listNetworkAddresses",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIListNetworkAddressesListener" }],
                        ret: "nsresult",
                    },

                    /* void getHostname (in nsIGetHostnameListener aListener); */
                    Method {
                        name: "getHostname",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIGetHostnameListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

