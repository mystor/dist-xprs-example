//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleCaretMoveEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleCaretMoveEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Some(&[
                    /* readonly attribute long caretOffset; */
                    Method {
                        name: "get_caretOffset",
                        abi: "C",
                        params: &[Param { name: "aCaretOffset", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

