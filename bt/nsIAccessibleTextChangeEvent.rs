//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleTextChangeEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleTextChangeEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Some(&[
                    /* readonly attribute long start; */
                    Method {
                        name: "get_start",
                        abi: "C",
                        params: &[Param { name: "aStart", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isInserted; */
                    Method {
                        name: "get_isInserted",
                        abi: "C",
                        params: &[Param { name: "aIsInserted", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString modifiedText; */
                    Method {
                        name: "get_modifiedText",
                        abi: "C",
                        params: &[Param { name: "aModifiedText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

