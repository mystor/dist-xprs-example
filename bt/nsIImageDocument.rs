//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIImageDocument.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIImageDocument",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean imageIsOverflowing; */
                    Method {
                        name: "get_imageIsOverflowing",
                        abi: "C",
                        params: &[Param { name: "aImageIsOverflowing", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean imageIsResized; */
                    Method {
                        name: "get_imageIsResized",
                        abi: "C",
                        params: &[Param { name: "aImageIsResized", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute imgIRequest imageRequest; */
                    Method {
                        name: "get_imageRequest",
                        abi: "C",
                        params: &[Param { name: "aImageRequest", ty: "*mut *const imgIRequest" }],
                        ret: "nsresult",
                    },

                    /* [binaryname(DOMShrinkToFit)] void shrinkToFit (); */
                    Method {
                        name: "DOMShrinkToFit",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* [binaryname(DOMRestoreImage)] void restoreImage (); */
                    Method {
                        name: "DOMRestoreImage",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* [binaryname(DOMRestoreImageTo)] void restoreImageTo (in long x, in long y); */
                    Method {
                        name: "DOMRestoreImageTo",
                        abi: "C",
                        params: &[Param { name: "x", ty: "libc::int32_t" }, Param { name: "y", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* [binaryname(DOMToggleImageSize)] void toggleImageSize (); */
                    Method {
                        name: "DOMToggleImageSize",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

