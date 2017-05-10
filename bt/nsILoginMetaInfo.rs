//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoginMetaInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILoginMetaInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute AString guid; */
                    Method {
                        name: "get_guid",
                        abi: "C",
                        params: &[Param { name: "aGuid", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_guid",
                        abi: "C",
                        params: &[Param { name: "aGuid", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long long timeCreated; */
                    Method {
                        name: "get_timeCreated",
                        abi: "C",
                        params: &[Param { name: "aTimeCreated", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_timeCreated",
                        abi: "C",
                        params: &[Param { name: "aTimeCreated", ty: "libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long long timeLastUsed; */
                    Method {
                        name: "get_timeLastUsed",
                        abi: "C",
                        params: &[Param { name: "aTimeLastUsed", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_timeLastUsed",
                        abi: "C",
                        params: &[Param { name: "aTimeLastUsed", ty: "libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long long timePasswordChanged; */
                    Method {
                        name: "get_timePasswordChanged",
                        abi: "C",
                        params: &[Param { name: "aTimePasswordChanged", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_timePasswordChanged",
                        abi: "C",
                        params: &[Param { name: "aTimePasswordChanged", ty: "libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long timesUsed; */
                    Method {
                        name: "get_timesUsed",
                        abi: "C",
                        params: &[Param { name: "aTimesUsed", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_timesUsed",
                        abi: "C",
                        params: &[Param { name: "aTimesUsed", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

