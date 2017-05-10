//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISHTransaction.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISHTransaction",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsISHEntry sHEntry; */
                    Method {
                        name: "get_sHEntry",
                        abi: "C",
                        params: &[Param { name: "aSHEntry", ty: "*mut *const nsISHEntry" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_sHEntry",
                        abi: "C",
                        params: &[Param { name: "aSHEntry", ty: "*const nsISHEntry" }],
                        ret: "nsresult",
                    },

                    /* attribute nsISHTransaction prev; */
                    Method {
                        name: "get_prev",
                        abi: "C",
                        params: &[Param { name: "aPrev", ty: "*mut *const nsISHTransaction" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_prev",
                        abi: "C",
                        params: &[Param { name: "aPrev", ty: "*const nsISHTransaction" }],
                        ret: "nsresult",
                    },

                    /* attribute nsISHTransaction next; */
                    Method {
                        name: "get_next",
                        abi: "C",
                        params: &[Param { name: "aNext", ty: "*mut *const nsISHTransaction" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_next",
                        abi: "C",
                        params: &[Param { name: "aNext", ty: "*const nsISHTransaction" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean persist; */
                    Method {
                        name: "get_persist",
                        abi: "C",
                        params: &[Param { name: "aPersist", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_persist",
                        abi: "C",
                        params: &[Param { name: "aPersist", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void create (in nsISHEntry aSHEntry, in nsISHTransaction aPrev); */
                    Method {
                        name: "create",
                        abi: "C",
                        params: &[Param { name: "aSHEntry", ty: "*const nsISHEntry" }, Param { name: "aPrev", ty: "*const nsISHTransaction" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

