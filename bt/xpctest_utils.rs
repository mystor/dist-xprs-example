//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpctest_utils.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXPCTestFunctionInterface",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* string echo (in string arg); */
                    Method {
                        name: "echo",
                        abi: "C",
                        params: &[Param { name: "arg", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIXPCTestUtils",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIXPCTestFunctionInterface doubleWrapFunction (in nsIXPCTestFunctionInterface f); */
                    Method {
                        name: "doubleWrapFunction",
                        abi: "C",
                        params: &[Param { name: "f", ty: "*const nsIXPCTestFunctionInterface" }, Param { name: "_retval", ty: "*mut *const nsIXPCTestFunctionInterface" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

