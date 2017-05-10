//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCharacterData.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMCharacterData",
            base: Some("nsIDOMNode"),
            methods: Some(&[
                    /* attribute DOMString data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* DOMString substringData (in unsigned long offset, in unsigned long count) raises (DOMException); */
                    Method {
                        name: "substringData",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::uint32_t" }, Param { name: "count", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void appendData (in DOMString arg) raises (DOMException); */
                    Method {
                        name: "appendData",
                        abi: "C",
                        params: &[Param { name: "arg", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void insertData (in unsigned long offset, in DOMString arg) raises (DOMException); */
                    Method {
                        name: "insertData",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::uint32_t" }, Param { name: "arg", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void deleteData (in unsigned long offset, in unsigned long count) raises (DOMException); */
                    Method {
                        name: "deleteData",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::uint32_t" }, Param { name: "count", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void replaceData (in unsigned long offset, in unsigned long count, in DOMString arg) raises (DOMException); */
                    Method {
                        name: "replaceData",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::uint32_t" }, Param { name: "count", ty: "libc::uint32_t" }, Param { name: "arg", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

