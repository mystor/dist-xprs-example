//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINetworkLinkService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINetworkLinkService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean isLinkUp; */
                    Method {
                        name: "get_isLinkUp",
                        abi: "C",
                        params: &[Param { name: "aIsLinkUp", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean linkStatusKnown; */
                    Method {
                        name: "get_linkStatusKnown",
                        abi: "C",
                        params: &[Param { name: "aLinkStatusKnown", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long linkType; */
                    Method {
                        name: "get_linkType",
                        abi: "C",
                        params: &[Param { name: "aLinkType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

