//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPartialSHistory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPartialSHistory",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [infallible] readonly attribute unsigned long count; */
                    Method {
                        name: "get_count",
                        abi: "C",
                        params: &[Param { name: "aCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [infallible] readonly attribute long globalIndex; */
                    Method {
                        name: "get_globalIndex",
                        abi: "C",
                        params: &[Param { name: "aGlobalIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* [infallible] readonly attribute unsigned long globalIndexOffset; */
                    Method {
                        name: "get_globalIndexOffset",
                        abi: "C",
                        params: &[Param { name: "aGlobalIndexOffset", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIFrameLoader ownerFrameLoader; */
                    Method {
                        name: "get_ownerFrameLoader",
                        abi: "C",
                        params: &[Param { name: "aOwnerFrameLoader", ty: "*mut *const nsIFrameLoader" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIGroupedSHistory groupedSHistory; */
                    Method {
                        name: "get_groupedSHistory",
                        abi: "C",
                        params: &[Param { name: "aGroupedSHistory", ty: "*mut *const nsIGroupedSHistory" }],
                        ret: "nsresult",
                    },

                    /* [infallible] attribute long activeState; */
                    Method {
                        name: "get_activeState",
                        abi: "C",
                        params: &[Param { name: "aActiveState", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_activeState",
                        abi: "C",
                        params: &[Param { name: "aActiveState", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void onAttachGroupedSHistory (in nsIGroupedSHistory aGroup, in unsigned long aOffset); */
                    Method {
                        name: "onAttachGroupedSHistory",
                        abi: "C",
                        params: &[Param { name: "aGroup", ty: "*const nsIGroupedSHistory" }, Param { name: "aOffset", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void handleSHistoryUpdate (in unsigned long aCount, in unsigned long aLocalIndex, in boolean aTruncate); */
                    Method {
                        name: "handleSHistoryUpdate",
                        abi: "C",
                        params: &[Param { name: "aCount", ty: "libc::uint32_t" }, Param { name: "aLocalIndex", ty: "libc::uint32_t" }, Param { name: "aTruncate", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void onActive (in unsigned long aGlobalLength, in unsigned long aTargetLocalIndex); */
                    Method {
                        name: "onActive",
                        abi: "C",
                        params: &[Param { name: "aGlobalLength", ty: "libc::uint32_t" }, Param { name: "aTargetLocalIndex", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void onDeactive (); */
                    Method {
                        name: "onDeactive",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

