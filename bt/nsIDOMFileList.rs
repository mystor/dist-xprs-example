//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMFileList.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMFileList",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsISupports item (in unsigned long index); */
                    Method {
                        name: "item",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

