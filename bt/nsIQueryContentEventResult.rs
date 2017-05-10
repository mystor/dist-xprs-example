//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIQueryContentEventResult.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIQueryContentEventResult",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long offset; */
                    Method {
                        name: "get_offset",
                        abi: "C",
                        params: &[Param { name: "aOffset", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long tentativeCaretOffset; */
                    Method {
                        name: "get_tentativeCaretOffset",
                        abi: "C",
                        params: &[Param { name: "aTentativeCaretOffset", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean reversed; */
                    Method {
                        name: "get_reversed",
                        abi: "C",
                        params: &[Param { name: "aReversed", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long left; */
                    Method {
                        name: "get_left",
                        abi: "C",
                        params: &[Param { name: "aLeft", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long top; */
                    Method {
                        name: "get_top",
                        abi: "C",
                        params: &[Param { name: "aTop", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long width; */
                    Method {
                        name: "get_width",
                        abi: "C",
                        params: &[Param { name: "aWidth", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long height; */
                    Method {
                        name: "get_height",
                        abi: "C",
                        params: &[Param { name: "aHeight", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString text; */
                    Method {
                        name: "get_text",
                        abi: "C",
                        params: &[Param { name: "aText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void getCharacterRect (in long offset, out long left, out long top, out long width, out long height); */
                    Method {
                        name: "getCharacterRect",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::int32_t" }, Param { name: "left", ty: "*mut libc::int32_t" }, Param { name: "top", ty: "*mut libc::int32_t" }, Param { name: "width", ty: "*mut libc::int32_t" }, Param { name: "height", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean succeeded; */
                    Method {
                        name: "get_succeeded",
                        abi: "C",
                        params: &[Param { name: "aSucceeded", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean notFound; */
                    Method {
                        name: "get_notFound",
                        abi: "C",
                        params: &[Param { name: "aNotFound", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean tentativeCaretOffsetNotFound; */
                    Method {
                        name: "get_tentativeCaretOffsetNotFound",
                        abi: "C",
                        params: &[Param { name: "aTentativeCaretOffsetNotFound", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

