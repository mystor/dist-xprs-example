//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISecurityInfoProvider.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISecurityInfoProvider",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsISupports securityInfo; */
                    Method {
                        name: "get_securityInfo",
                        abi: "C",
                        params: &[Param { name: "aSecurityInfo", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean hasTransferredData; */
                    Method {
                        name: "get_hasTransferredData",
                        abi: "C",
                        params: &[Param { name: "aHasTransferredData", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

