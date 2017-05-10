//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINetworkProperties.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINetworkProperties",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean isWifi; */
                    Method {
                        name: "get_isWifi",
                        abi: "C",
                        params: &[Param { name: "aIsWifi", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long dhcpGateway; */
                    Method {
                        name: "get_dhcpGateway",
                        abi: "C",
                        params: &[Param { name: "aDhcpGateway", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

