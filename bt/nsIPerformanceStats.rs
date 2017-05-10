//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPerformanceStats.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPerformanceGroupDetails",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AString groupId; */
                    Method {
                        name: "get_groupId",
                        abi: "C",
                        params: &[Param { name: "aGroupId", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint64_t windowId; */
                    Method {
                        name: "get_windowId",
                        abi: "C",
                        params: &[Param { name: "aWindowId", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute bool isSystem; */
                    Method {
                        name: "get_isSystem",
                        abi: "C",
                        params: &[Param { name: "aIsSystem", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long long processId; */
                    Method {
                        name: "get_processId",
                        abi: "C",
                        params: &[Param { name: "aProcessId", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute bool isContentProcess; */
                    Method {
                        name: "get_isContentProcess",
                        abi: "C",
                        params: &[Param { name: "aIsContentProcess", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPerformanceStats",
            base: Some("nsIPerformanceGroupDetails"),
            methods: None,
        },


        Interface {
            name: "nsIPerformanceSnapshot",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIArray getComponentsData (); */
                    Method {
                        name: "getComponentsData",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* nsIPerformanceStats getProcessData (); */
                    Method {
                        name: "getProcessData",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIPerformanceStats" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPerformanceAlert",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long reason; */
                    Method {
                        name: "get_reason",
                        abi: "C",
                        params: &[Param { name: "aReason", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long long highestJank; */
                    Method {
                        name: "get_highestJank",
                        abi: "C",
                        params: &[Param { name: "aHighestJank", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long long highestCPOW; */
                    Method {
                        name: "get_highestCPOW",
                        abi: "C",
                        params: &[Param { name: "aHighestCPOW", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPerformanceObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void observe (in nsIPerformanceGroupDetails target, in nsIPerformanceAlert alert); */
                    Method {
                        name: "observe",
                        abi: "C",
                        params: &[Param { name: "target", ty: "*const nsIPerformanceGroupDetails" }, Param { name: "alert", ty: "*const nsIPerformanceAlert" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPerformanceObservable",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIPerformanceGroupDetails target; */
                    Method {
                        name: "get_target",
                        abi: "C",
                        params: &[Param { name: "aTarget", ty: "*mut *const nsIPerformanceGroupDetails" }],
                        ret: "nsresult",
                    },

                    /* void addJankObserver (in nsIPerformanceObserver observer); */
                    Method {
                        name: "addJankObserver",
                        abi: "C",
                        params: &[Param { name: "observer", ty: "*const nsIPerformanceObserver" }],
                        ret: "nsresult",
                    },

                    /* void removeJankObserver (in nsIPerformanceObserver observer); */
                    Method {
                        name: "removeJankObserver",
                        abi: "C",
                        params: &[Param { name: "observer", ty: "*const nsIPerformanceObserver" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPerformanceStatsService",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

