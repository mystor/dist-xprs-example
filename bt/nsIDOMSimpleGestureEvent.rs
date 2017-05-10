//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMSimpleGestureEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMSimpleGestureEvent",
            base: Some("nsIDOMMouseEvent"),
            methods: Some(&[
                    /* attribute unsigned long allowedDirections; */
                    Method {
                        name: "get_allowedDirections",
                        abi: "C",
                        params: &[Param { name: "aAllowedDirections", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_allowedDirections",
                        abi: "C",
                        params: &[Param { name: "aAllowedDirections", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long direction; */
                    Method {
                        name: "get_direction",
                        abi: "C",
                        params: &[Param { name: "aDirection", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute double delta; */
                    Method {
                        name: "get_delta",
                        abi: "C",
                        params: &[Param { name: "aDelta", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long clickCount; */
                    Method {
                        name: "get_clickCount",
                        abi: "C",
                        params: &[Param { name: "aClickCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

