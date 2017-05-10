//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMClientRectList.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMClientRectList",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMClientRect item (in unsigned long index); */
                    Method {
                        name: "item",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMClientRect" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

