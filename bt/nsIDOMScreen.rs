//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMScreen.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMScreen",
            base: Some("nsIDOMEventTarget"),
            methods: Some(&[
                    /* readonly attribute long top; */
                    Method {
                        name: "get_top",
                        abi: "C",
                        params: &[Param { name: "aTop", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long availWidth; */
                    Method {
                        name: "get_availWidth",
                        abi: "C",
                        params: &[Param { name: "aAvailWidth", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long availHeight; */
                    Method {
                        name: "get_availHeight",
                        abi: "C",
                        params: &[Param { name: "aAvailHeight", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long availLeft; */
                    Method {
                        name: "get_availLeft",
                        abi: "C",
                        params: &[Param { name: "aAvailLeft", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long availTop; */
                    Method {
                        name: "get_availTop",
                        abi: "C",
                        params: &[Param { name: "aAvailTop", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

