//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMClientRect.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMClientRect",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute float left; */
                    Method {
                        name: "get_left",
                        abi: "C",
                        params: &[Param { name: "aLeft", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute float top; */
                    Method {
                        name: "get_top",
                        abi: "C",
                        params: &[Param { name: "aTop", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute float right; */
                    Method {
                        name: "get_right",
                        abi: "C",
                        params: &[Param { name: "aRight", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute float bottom; */
                    Method {
                        name: "get_bottom",
                        abi: "C",
                        params: &[Param { name: "aBottom", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute float width; */
                    Method {
                        name: "get_width",
                        abi: "C",
                        params: &[Param { name: "aWidth", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute float height; */
                    Method {
                        name: "get_height",
                        abi: "C",
                        params: &[Param { name: "aHeight", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

