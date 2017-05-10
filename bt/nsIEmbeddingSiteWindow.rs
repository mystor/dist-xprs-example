//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEmbeddingSiteWindow.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEmbeddingSiteWindow",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setDimensions (in unsigned long flags, in long x, in long y, in long cx, in long cy); */
                    Method {
                        name: "setDimensions",
                        abi: "C",
                        params: &[Param { name: "flags", ty: "libc::uint32_t" }, Param { name: "x", ty: "libc::int32_t" }, Param { name: "y", ty: "libc::int32_t" }, Param { name: "cx", ty: "libc::int32_t" }, Param { name: "cy", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void getDimensions (in unsigned long flags, out long x, out long y, out long cx, out long cy); */
                    Method {
                        name: "getDimensions",
                        abi: "C",
                        params: &[Param { name: "flags", ty: "libc::uint32_t" }, Param { name: "x", ty: "*mut libc::int32_t" }, Param { name: "y", ty: "*mut libc::int32_t" }, Param { name: "cx", ty: "*mut libc::int32_t" }, Param { name: "cy", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setFocus (); */
                    Method {
                        name: "setFocus",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* attribute boolean visibility; */
                    Method {
                        name: "get_visibility",
                        abi: "C",
                        params: &[Param { name: "aVisibility", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_visibility",
                        abi: "C",
                        params: &[Param { name: "aVisibility", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute wstring title; */
                    Method {
                        name: "get_title",
                        abi: "C",
                        params: &[Param { name: "aTitle", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_title",
                        abi: "C",
                        params: &[Param { name: "aTitle", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript] readonly attribute voidPtr siteWindow; */
                    Method {
                        name: "get_siteWindow",
                        abi: "C",
                        params: &[Param { name: "aSiteWindow", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* void blur (); */
                    Method {
                        name: "blur",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

