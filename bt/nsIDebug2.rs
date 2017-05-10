//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDebug2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDebug2",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean isDebugBuild; */
                    Method {
                        name: "get_isDebugBuild",
                        abi: "C",
                        params: &[Param { name: "aIsDebugBuild", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long assertionCount; */
                    Method {
                        name: "get_assertionCount",
                        abi: "C",
                        params: &[Param { name: "aAssertionCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute bool isDebuggerAttached; */
                    Method {
                        name: "get_isDebuggerAttached",
                        abi: "C",
                        params: &[Param { name: "aIsDebuggerAttached", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void assertion (in string aStr, in string aExpr, in string aFile, in long aLine); */
                    Method {
                        name: "assertion",
                        abi: "C",
                        params: &[Param { name: "aStr", ty: "*const libc::c_char" }, Param { name: "aExpr", ty: "*const libc::c_char" }, Param { name: "aFile", ty: "*const libc::c_char" }, Param { name: "aLine", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void warning (in string aStr, in string aFile, in long aLine); */
                    Method {
                        name: "warning",
                        abi: "C",
                        params: &[Param { name: "aStr", ty: "*const libc::c_char" }, Param { name: "aFile", ty: "*const libc::c_char" }, Param { name: "aLine", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void break (in string aFile, in long aLine); */
                    Method {
                        name: "break_",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*const libc::c_char" }, Param { name: "aLine", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void abort (in string aFile, in long aLine); */
                    Method {
                        name: "abort",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*const libc::c_char" }, Param { name: "aLine", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void rustPanic (in string aMessage); */
                    Method {
                        name: "rustPanic",
                        abi: "C",
                        params: &[Param { name: "aMessage", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

