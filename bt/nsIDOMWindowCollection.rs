//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMWindowCollection.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMWindowCollection",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* mozIDOMWindowProxy item (in unsigned long index); */
                    Method {
                        name: "item",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* mozIDOMWindowProxy namedItem (in DOMString name); */
                    Method {
                        name: "namedItem",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

