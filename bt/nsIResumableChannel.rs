//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIResumableChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIResumableChannel",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void resumeAt (in unsigned long long startPos, in ACString entityID); */
                    Method {
                        name: "resumeAt",
                        abi: "C",
                        params: &[Param { name: "startPos", ty: "libc::uint64_t" }, Param { name: "entityID", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString entityID; */
                    Method {
                        name: "get_entityID",
                        abi: "C",
                        params: &[Param { name: "aEntityID", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

