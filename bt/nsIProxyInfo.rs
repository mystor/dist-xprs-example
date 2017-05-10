//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProxyInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProxyInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AUTF8String host; */
                    Method {
                        name: "get_host",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long port; */
                    Method {
                        name: "get_port",
                        abi: "C",
                        params: &[Param { name: "aPort", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long flags; */
                    Method {
                        name: "get_flags",
                        abi: "C",
                        params: &[Param { name: "aFlags", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long resolveFlags; */
                    Method {
                        name: "get_resolveFlags",
                        abi: "C",
                        params: &[Param { name: "aResolveFlags", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString username; */
                    Method {
                        name: "get_username",
                        abi: "C",
                        params: &[Param { name: "aUsername", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString password; */
                    Method {
                        name: "get_password",
                        abi: "C",
                        params: &[Param { name: "aPassword", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long failoverTimeout; */
                    Method {
                        name: "get_failoverTimeout",
                        abi: "C",
                        params: &[Param { name: "aFailoverTimeout", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIProxyInfo failoverProxy; */
                    Method {
                        name: "get_failoverProxy",
                        abi: "C",
                        params: &[Param { name: "aFailoverProxy", ty: "*mut *const nsIProxyInfo" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_failoverProxy",
                        abi: "C",
                        params: &[Param { name: "aFailoverProxy", ty: "*const nsIProxyInfo" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

