//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozITXTToHTMLConv.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozITXTToHTMLConv",
            base: Some("nsIStreamConverter"),
            methods: Some(&[
                    /* wstring scanTXT (in wstring text, in unsigned long whattodo); */
                    Method {
                        name: "scanTXT",
                        abi: "C",
                        params: &[Param { name: "text", ty: "*const libc::int16_t" }, Param { name: "whattodo", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* wstring scanHTML (in wstring text, in unsigned long whattodo); */
                    Method {
                        name: "scanHTML",
                        abi: "C",
                        params: &[Param { name: "text", ty: "*const libc::int16_t" }, Param { name: "whattodo", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* unsigned long citeLevelTXT (in wstring line, out unsigned long logLineStart); */
                    Method {
                        name: "citeLevelTXT",
                        abi: "C",
                        params: &[Param { name: "line", ty: "*const libc::int16_t" }, Param { name: "logLineStart", ty: "*mut libc::uint32_t" }, Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void findURLInPlaintext (in wstring text, in long aLength, in long aPos, out long aStartPos, out long aEndPos); */
                    Method {
                        name: "findURLInPlaintext",
                        abi: "C",
                        params: &[Param { name: "text", ty: "*const libc::int16_t" }, Param { name: "aLength", ty: "libc::int32_t" }, Param { name: "aPos", ty: "libc::int32_t" }, Param { name: "aStartPos", ty: "*mut libc::int32_t" }, Param { name: "aEndPos", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

