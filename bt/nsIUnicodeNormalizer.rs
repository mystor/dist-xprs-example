//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUnicodeNormalizer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUnicodeNormalizer",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void NormalizeUnicodeNFD (in AString aSrc, out AString aDest); */
                    Method {
                        name: "NormalizeUnicodeNFD",
                        abi: "C",
                        params: &[Param { name: "aSrc", ty: "*const nsAString" }, Param { name: "aDest", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void NormalizeUnicodeNFC (in AString aSrc, out AString aDest); */
                    Method {
                        name: "NormalizeUnicodeNFC",
                        abi: "C",
                        params: &[Param { name: "aSrc", ty: "*const nsAString" }, Param { name: "aDest", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void NormalizeUnicodeNFKD (in AString aSrc, out AString aDest); */
                    Method {
                        name: "NormalizeUnicodeNFKD",
                        abi: "C",
                        params: &[Param { name: "aSrc", ty: "*const nsAString" }, Param { name: "aDest", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void NormalizeUnicodeNFKC (in AString aSrc, out AString aDest); */
                    Method {
                        name: "NormalizeUnicodeNFKC",
                        abi: "C",
                        params: &[Param { name: "aSrc", ty: "*const nsAString" }, Param { name: "aDest", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

