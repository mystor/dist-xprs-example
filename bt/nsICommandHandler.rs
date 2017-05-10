//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICommandHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICommandHandlerInit",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute mozIDOMWindowProxy window; */
                    Method {
                        name: "get_window",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_window",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsICommandHandler",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* string exec (in string aCommand, in string aParameters); */
                    Method {
                        name: "exec",
                        abi: "C",
                        params: &[Param { name: "aCommand", ty: "*const libc::c_char" }, Param { name: "aParameters", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* string query (in string aCommand, in string aParameters); */
                    Method {
                        name: "query",
                        abi: "C",
                        params: &[Param { name: "aCommand", ty: "*const libc::c_char" }, Param { name: "aParameters", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

