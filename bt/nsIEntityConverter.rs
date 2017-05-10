//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEntityConverter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEntityConverter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* string ConvertUTF32ToEntity (in unsigned long character, in unsigned long entityVersion); */
                    Method {
                        name: "ConvertUTF32ToEntity",
                        abi: "C",
                        params: &[Param { name: "character", ty: "libc::uint32_t" }, Param { name: "entityVersion", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* string ConvertToEntity (in wchar character, in unsigned long entityVersion); */
                    Method {
                        name: "ConvertToEntity",
                        abi: "C",
                        params: &[Param { name: "character", ty: "libc::int16_t" }, Param { name: "entityVersion", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* wstring ConvertToEntities (in wstring inString, in unsigned long entityVersion); */
                    Method {
                        name: "ConvertToEntities",
                        abi: "C",
                        params: &[Param { name: "inString", ty: "*const libc::int16_t" }, Param { name: "entityVersion", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

