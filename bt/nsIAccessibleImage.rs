//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleImage.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleImage",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void getImagePosition (in unsigned long coordType, out long x, out long y); */
                    Method {
                        name: "getImagePosition",
                        abi: "C",
                        params: &[Param { name: "coordType", ty: "libc::uint32_t" }, Param { name: "x", ty: "*mut libc::int32_t" }, Param { name: "y", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void getImageSize (out long width, out long height); */
                    Method {
                        name: "getImageSize",
                        abi: "C",
                        params: &[Param { name: "width", ty: "*mut libc::int32_t" }, Param { name: "height", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

