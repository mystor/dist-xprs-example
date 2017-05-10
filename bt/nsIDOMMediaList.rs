//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMMediaList.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMMediaList",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString mediaText; */
                    Method {
                        name: "get_mediaText",
                        abi: "C",
                        params: &[Param { name: "aMediaText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_mediaText",
                        abi: "C",
                        params: &[Param { name: "aMediaText", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* DOMString item (in unsigned long index); */
                    Method {
                        name: "item",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void deleteMedium (in DOMString oldMedium) raises (DOMException); */
                    Method {
                        name: "deleteMedium",
                        abi: "C",
                        params: &[Param { name: "oldMedium", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void appendMedium (in DOMString newMedium) raises (DOMException); */
                    Method {
                        name: "appendMedium",
                        abi: "C",
                        params: &[Param { name: "newMedium", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

