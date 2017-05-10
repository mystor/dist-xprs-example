//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpctest_returncode.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXPCTestReturnCodeParent",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsresult callChild (in long childBehavior); */
                    Method {
                        name: "callChild",
                        abi: "C",
                        params: &[Param { name: "childBehavior", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIXPCTestReturnCodeChild",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void doIt (in long behavior); */
                    Method {
                        name: "doIt",
                        abi: "C",
                        params: &[Param { name: "behavior", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

