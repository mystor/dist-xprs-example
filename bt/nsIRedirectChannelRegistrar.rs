//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRedirectChannelRegistrar.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRedirectChannelRegistrar",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* uint32_t registerChannel (in nsIChannel channel); */
                    Method {
                        name: "registerChannel",
                        abi: "C",
                        params: &[Param { name: "channel", ty: "*const nsIChannel" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIChannel linkChannels (in uint32_t id, in nsIParentChannel channel); */
                    Method {
                        name: "linkChannels",
                        abi: "C",
                        params: &[Param { name: "id", ty: "uint32_t" }, Param { name: "channel", ty: "*const nsIParentChannel" }, Param { name: "_retval", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* nsIChannel getRegisteredChannel (in uint32_t id); */
                    Method {
                        name: "getRegisteredChannel",
                        abi: "C",
                        params: &[Param { name: "id", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* nsIParentChannel getParentChannel (in uint32_t id); */
                    Method {
                        name: "getParentChannel",
                        abi: "C",
                        params: &[Param { name: "id", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIParentChannel" }],
                        ret: "nsresult",
                    },

                    /* void deregisterChannels (in uint32_t id); */
                    Method {
                        name: "deregisterChannels",
                        abi: "C",
                        params: &[Param { name: "id", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

