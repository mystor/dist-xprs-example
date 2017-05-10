//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWifiAccessPoint.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWifiAccessPoint",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute ACString mac; */
                    Method {
                        name: "get_mac",
                        abi: "C",
                        params: &[Param { name: "aMac", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString ssid; */
                    Method {
                        name: "get_ssid",
                        abi: "C",
                        params: &[Param { name: "aSsid", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString rawSSID; */
                    Method {
                        name: "get_rawSSID",
                        abi: "C",
                        params: &[Param { name: "aRawSSID", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long signal; */
                    Method {
                        name: "get_signal",
                        abi: "C",
                        params: &[Param { name: "aSignal", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

