//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScreenManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScreenManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIScreen screenForRect (in long left, in long top, in long width, in long height); */
                    Method {
                        name: "screenForRect",
                        abi: "C",
                        params: &[Param { name: "left", ty: "libc::int32_t" }, Param { name: "top", ty: "libc::int32_t" }, Param { name: "width", ty: "libc::int32_t" }, Param { name: "height", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIScreen" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIScreen primaryScreen; */
                    Method {
                        name: "get_primaryScreen",
                        abi: "C",
                        params: &[Param { name: "aPrimaryScreen", ty: "*mut *const nsIScreen" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

