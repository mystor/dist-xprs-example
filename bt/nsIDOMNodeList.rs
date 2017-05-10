//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMNodeList.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMNodeList",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIDOMNode item (in unsigned long index); */
                    Method {
                        name: "item",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

