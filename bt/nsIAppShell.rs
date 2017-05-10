//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAppShell.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAppShell",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void run (); */
                    Method {
                        name: "run",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void exit (); */
                    Method {
                        name: "exit",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void favorPerformanceHint (in boolean favorPerfOverStarvation, in unsigned long starvationDelay); */
                    Method {
                        name: "favorPerformanceHint",
                        abi: "C",
                        params: &[Param { name: "favorPerfOverStarvation", ty: "bool" }, Param { name: "starvationDelay", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void suspendNative (); */
                    Method {
                        name: "suspendNative",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void resumeNative (); */
                    Method {
                        name: "resumeNative",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long eventloopNestingLevel; */
                    Method {
                        name: "get_eventloopNestingLevel",
                        abi: "C",
                        params: &[Param { name: "aEventloopNestingLevel", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

