//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDeviceSensors.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDeviceSensorData",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute double x; */
                    Method {
                        name: "get_x",
                        abi: "C",
                        params: &[Param { name: "aX", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute double y; */
                    Method {
                        name: "get_y",
                        abi: "C",
                        params: &[Param { name: "aY", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute double z; */
                    Method {
                        name: "get_z",
                        abi: "C",
                        params: &[Param { name: "aZ", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDeviceSensors",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* bool hasWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
                    Method {
                        name: "hasWindowListener",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "libc::uint32_t" }, Param { name: "aWindow", ty: "*const nsIDOMWindow" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void addWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
                    Method {
                        name: "addWindowListener",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "libc::uint32_t" }, Param { name: "aWindow", ty: "*const nsIDOMWindow" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void removeWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
                    Method {
                        name: "removeWindowListener",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "libc::uint32_t" }, Param { name: "aWindow", ty: "*const nsIDOMWindow" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void removeWindowAsListener (in nsIDOMWindow aWindow); */
                    Method {
                        name: "removeWindowAsListener",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const nsIDOMWindow" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

