//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpccomponents.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXPCComponents_InterfacesByID",
            base: Some("nsISupports"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "nsIXPCComponents_Interfaces",
            base: Some("nsISupports"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "nsIXPCComponents_Classes",
            base: Some("nsISupports"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "nsIXPCComponents_ClassesByID",
            base: Some("nsISupports"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "nsIXPCComponents_Results",
            base: Some("nsISupports"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "nsIXPCComponents_ID",
            base: Some("nsISupports"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "nsIXPCComponents_Exception",
            base: Some("nsISupports"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "nsIXPCComponents_Constructor",
            base: Some("nsISupports"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "nsIXPCConstructor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIJSCID classID; */
                    Method {
                        name: "get_classID",
                        abi: "C",
                        params: &[Param { name: "aClassID", ty: "*mut *const nsIJSCID" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIJSIID interfaceID; */
                    Method {
                        name: "get_interfaceID",
                        abi: "C",
                        params: &[Param { name: "aInterfaceID", ty: "*mut *const nsIJSIID" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute string initializer; */
                    Method {
                        name: "get_initializer",
                        abi: "C",
                        params: &[Param { name: "aInitializer", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIXPCComponents_utils_Sandbox",
            base: Some("nsISupports"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "ScheduledGCCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void callback (); */
                    Method {
                        name: "callback",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIXPCComponents_Utils",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIXPCComponentsBase",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIXPCComponents_Interfaces interfaces; */
                    Method {
                        name: "get_interfaces",
                        abi: "C",
                        params: &[Param { name: "aInterfaces", ty: "*mut *const nsIXPCComponents_Interfaces" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIXPCComponents_InterfacesByID interfacesByID; */
                    Method {
                        name: "get_interfacesByID",
                        abi: "C",
                        params: &[Param { name: "aInterfacesByID", ty: "*mut *const nsIXPCComponents_InterfacesByID" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIXPCComponents_Results results; */
                    Method {
                        name: "get_results",
                        abi: "C",
                        params: &[Param { name: "aResults", ty: "*mut *const nsIXPCComponents_Results" }],
                        ret: "nsresult",
                    },

                    /* boolean isSuccessCode (in nsresult result); */
                    Method {
                        name: "isSuccessCode",
                        abi: "C",
                        params: &[Param { name: "result", ty: "nsresult" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIXPCComponents",
            base: Some("nsIXPCComponentsBase"),
            methods: None,
        },


        ]; D}

