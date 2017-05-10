//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpctest_interfaces.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXPCTestInterfaceA",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute string name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIXPCTestInterfaceB",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute string name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIXPCTestInterfaceC",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute long someInteger; */
                    Method {
                        name: "get_someInteger",
                        abi: "C",
                        params: &[Param { name: "aSomeInteger", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_someInteger",
                        abi: "C",
                        params: &[Param { name: "aSomeInteger", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

