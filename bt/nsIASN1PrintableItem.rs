//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIASN1PrintableItem.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIASN1PrintableItem",
            base: Some("nsIASN1Object"),
            methods: Some(&[
                    /* [noscript] void setData (in charPtr data, in unsigned long len); */
                    Method {
                        name: "setData",
                        abi: "C",
                        params: &[Param { name: "data", ty: "*const u8" }, Param { name: "len", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void getData (out charPtr data, out unsigned long len); */
                    Method {
                        name: "getData",
                        abi: "C",
                        params: &[Param { name: "data", ty: "*mut *const u8" }, Param { name: "len", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

