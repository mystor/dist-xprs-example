//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrefLocalizedString.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrefLocalizedString",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute wstring data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* wstring toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void setDataWithLength (in unsigned long length, [size_is (length)] in wstring data); */
                    Method {
                        name: "setDataWithLength",
                        abi: "C",
                        params: &[Param { name: "length", ty: "libc::uint32_t" }, Param { name: "data", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

