//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScreen.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScreen",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void GetRect (out long left, out long top, out long width, out long height); */
                    Method {
                        name: "GetRect",
                        abi: "C",
                        params: &[Param { name: "left", ty: "*mut libc::int32_t" }, Param { name: "top", ty: "*mut libc::int32_t" }, Param { name: "width", ty: "*mut libc::int32_t" }, Param { name: "height", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void GetAvailRect (out long left, out long top, out long width, out long height); */
                    Method {
                        name: "GetAvailRect",
                        abi: "C",
                        params: &[Param { name: "left", ty: "*mut libc::int32_t" }, Param { name: "top", ty: "*mut libc::int32_t" }, Param { name: "width", ty: "*mut libc::int32_t" }, Param { name: "height", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void GetRectDisplayPix (out long left, out long top, out long width, out long height); */
                    Method {
                        name: "GetRectDisplayPix",
                        abi: "C",
                        params: &[Param { name: "left", ty: "*mut libc::int32_t" }, Param { name: "top", ty: "*mut libc::int32_t" }, Param { name: "width", ty: "*mut libc::int32_t" }, Param { name: "height", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void GetAvailRectDisplayPix (out long left, out long top, out long width, out long height); */
                    Method {
                        name: "GetAvailRectDisplayPix",
                        abi: "C",
                        params: &[Param { name: "left", ty: "*mut libc::int32_t" }, Param { name: "top", ty: "*mut libc::int32_t" }, Param { name: "width", ty: "*mut libc::int32_t" }, Param { name: "height", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long pixelDepth; */
                    Method {
                        name: "get_pixelDepth",
                        abi: "C",
                        params: &[Param { name: "aPixelDepth", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long colorDepth; */
                    Method {
                        name: "get_colorDepth",
                        abi: "C",
                        params: &[Param { name: "aColorDepth", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute double contentsScaleFactor; */
                    Method {
                        name: "get_contentsScaleFactor",
                        abi: "C",
                        params: &[Param { name: "aContentsScaleFactor", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute double defaultCSSScaleFactor; */
                    Method {
                        name: "get_defaultCSSScaleFactor",
                        abi: "C",
                        params: &[Param { name: "aDefaultCSSScaleFactor", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

