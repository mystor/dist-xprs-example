//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIViewSourceChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIViewSourceChannel",
            base: Some("nsIChannel"),
            methods: Some(&[
                    /* [must_use] attribute ACString originalContentType; */
                    Method {
                        name: "get_originalContentType",
                        abi: "C",
                        params: &[Param { name: "aOriginalContentType", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_originalContentType",
                        abi: "C",
                        params: &[Param { name: "aOriginalContentType", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute boolean isSrcdocChannel; */
                    Method {
                        name: "get_isSrcdocChannel",
                        abi: "C",
                        params: &[Param { name: "aIsSrcdocChannel", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] attribute nsIURI baseURI; */
                    Method {
                        name: "get_baseURI",
                        abi: "C",
                        params: &[Param { name: "aBaseURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_baseURI",
                        abi: "C",
                        params: &[Param { name: "aBaseURI", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

