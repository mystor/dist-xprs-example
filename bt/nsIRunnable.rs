//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRunnable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRunnable",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void run (); */
                    Method {
                        name: "run",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIRunnablePriority",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long priority; */
                    Method {
                        name: "get_priority",
                        abi: "C",
                        params: &[Param { name: "aPriority", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

