//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScriptChannel",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute unsigned long executionPolicy; */
                    Method {
                        name: "get_executionPolicy",
                        abi: "C",
                        params: &[Param { name: "aExecutionPolicy", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_executionPolicy",
                        abi: "C",
                        params: &[Param { name: "aExecutionPolicy", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean executeAsync; */
                    Method {
                        name: "get_executeAsync",
                        abi: "C",
                        params: &[Param { name: "aExecuteAsync", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_executeAsync",
                        abi: "C",
                        params: &[Param { name: "aExecuteAsync", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

