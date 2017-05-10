//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMWindowUtils.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMWindowUtils",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsITranslationNodeList",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode item (in unsigned long index); */
                    Method {
                        name: "item",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* boolean isTranslationRootAtIndex (in unsigned long index); */
                    Method {
                        name: "isTranslationRootAtIndex",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIJSRAIIHelper",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void destruct (); */
                    Method {
                        name: "destruct",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

