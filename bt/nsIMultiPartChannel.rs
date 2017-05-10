//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMultiPartChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMultiPartChannel",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIChannel baseChannel; */
                    Method {
                        name: "get_baseChannel",
                        abi: "C",
                        params: &[Param { name: "aBaseChannel", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t partID; */
                    Method {
                        name: "get_partID",
                        abi: "C",
                        params: &[Param { name: "aPartID", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isLastPart; */
                    Method {
                        name: "get_isLastPart",
                        abi: "C",
                        params: &[Param { name: "aIsLastPart", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

