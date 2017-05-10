//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMacUtils.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMacUtils",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean isUniversalBinary; */
                    Method {
                        name: "get_isUniversalBinary",
                        abi: "C",
                        params: &[Param { name: "aIsUniversalBinary", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString architecturesInBinary; */
                    Method {
                        name: "get_architecturesInBinary",
                        abi: "C",
                        params: &[Param { name: "aArchitecturesInBinary", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isTranslated; */
                    Method {
                        name: "get_isTranslated",
                        abi: "C",
                        params: &[Param { name: "aIsTranslated", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

