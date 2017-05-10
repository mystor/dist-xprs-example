//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMStyleSheetList.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMStyleSheetList",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [binaryname(SlowItem)] nsIDOMStyleSheet item (in unsigned long index); */
                    Method {
                        name: "SlowItem",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMStyleSheet" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

