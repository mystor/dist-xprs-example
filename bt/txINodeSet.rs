//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/txINodeSet.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "txINodeSet",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIDOMNode item (in unsigned long index); */
                    Method {
                        name: "item",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* double itemAsNumber (in unsigned long index); */
                    Method {
                        name: "itemAsNumber",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* DOMString itemAsString (in unsigned long index); */
                    Method {
                        name: "itemAsString",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void add (in nsIDOMNode node); */
                    Method {
                        name: "add",
                        abi: "C",
                        params: &[Param { name: "node", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

