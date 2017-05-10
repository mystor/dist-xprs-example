//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISemanticUnitScanner.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISemanticUnitScanner",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void start (in string characterSet); */
                    Method {
                        name: "start",
                        abi: "C",
                        params: &[Param { name: "characterSet", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* boolean next (in wstring text, in long length, in long pos, in boolean isLastBuffer, out long begin, out long end); */
                    Method {
                        name: "next",
                        abi: "C",
                        params: &[Param { name: "text", ty: "*const libc::int16_t" }, Param { name: "length", ty: "libc::int32_t" }, Param { name: "pos", ty: "libc::int32_t" }, Param { name: "isLastBuffer", ty: "bool" }, Param { name: "begin", ty: "*mut libc::int32_t" }, Param { name: "end", ty: "*mut libc::int32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

