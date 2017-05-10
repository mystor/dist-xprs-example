//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDisplayInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDisplayInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute long id; */
                    Method {
                        name: "get_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean connected; */
                    Method {
                        name: "get_connected",
                        abi: "C",
                        params: &[Param { name: "aConnected", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

