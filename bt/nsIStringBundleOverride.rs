//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStringBundleOverride.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStringBundleOverride",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* AString getStringFromName (in AUTF8String url, in ACString key); */
                    Method {
                        name: "getStringFromName",
                        abi: "C",
                        params: &[Param { name: "url", ty: "*const nsACString" }, Param { name: "key", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator enumerateKeysInBundle (in AUTF8String url); */
                    Method {
                        name: "enumerateKeysInBundle",
                        abi: "C",
                        params: &[Param { name: "url", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

