//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMessageLoop.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMessageLoop",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void postIdleTask (in nsIRunnable task, in uint32_t ensureRunsAfterMS); */
                    Method {
                        name: "postIdleTask",
                        abi: "C",
                        params: &[Param { name: "task", ty: "*const nsIRunnable" }, Param { name: "ensureRunsAfterMS", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

