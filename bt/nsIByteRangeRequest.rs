//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIByteRangeRequest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIByteRangeRequest",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean isByteRangeRequest; */
                    Method {
                        name: "get_isByteRangeRequest",
                        abi: "C",
                        params: &[Param { name: "aIsByteRangeRequest", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long startRange; */
                    Method {
                        name: "get_startRange",
                        abi: "C",
                        params: &[Param { name: "aStartRange", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long endRange; */
                    Method {
                        name: "get_endRange",
                        abi: "C",
                        params: &[Param { name: "aEndRange", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

