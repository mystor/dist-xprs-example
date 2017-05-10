//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrivateBrowsingChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrivateBrowsingChannel",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setPrivate (in boolean aPrivate); */
                    Method {
                        name: "setPrivate",
                        abi: "C",
                        params: &[Param { name: "aPrivate", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isChannelPrivate; */
                    Method {
                        name: "get_isChannelPrivate",
                        abi: "C",
                        params: &[Param { name: "aIsChannelPrivate", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [noscript] boolean isPrivateModeOverriden (out boolean aValue); */
                    Method {
                        name: "isPrivateModeOverriden",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

