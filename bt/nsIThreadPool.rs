//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIThreadPool.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIThreadPoolListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onThreadCreated (); */
                    Method {
                        name: "onThreadCreated",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void onThreadShuttingDown (); */
                    Method {
                        name: "onThreadShuttingDown",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIThreadPool",
            base: Some("nsIEventTarget"),
            methods: Some(&[
                    /* void shutdown (); */
                    Method {
                        name: "shutdown",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long threadLimit; */
                    Method {
                        name: "get_threadLimit",
                        abi: "C",
                        params: &[Param { name: "aThreadLimit", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_threadLimit",
                        abi: "C",
                        params: &[Param { name: "aThreadLimit", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long idleThreadLimit; */
                    Method {
                        name: "get_idleThreadLimit",
                        abi: "C",
                        params: &[Param { name: "aIdleThreadLimit", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_idleThreadLimit",
                        abi: "C",
                        params: &[Param { name: "aIdleThreadLimit", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long idleThreadTimeout; */
                    Method {
                        name: "get_idleThreadTimeout",
                        abi: "C",
                        params: &[Param { name: "aIdleThreadTimeout", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_idleThreadTimeout",
                        abi: "C",
                        params: &[Param { name: "aIdleThreadTimeout", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long threadStackSize; */
                    Method {
                        name: "get_threadStackSize",
                        abi: "C",
                        params: &[Param { name: "aThreadStackSize", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_threadStackSize",
                        abi: "C",
                        params: &[Param { name: "aThreadStackSize", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIThreadPoolListener listener; */
                    Method {
                        name: "get_listener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*mut *const nsIThreadPoolListener" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_listener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIThreadPoolListener" }],
                        ret: "nsresult",
                    },

                    /* void setName (in ACString aName); */
                    Method {
                        name: "setName",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

