//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPK11TokenDB.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPK11TokenDB",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIPK11Token getInternalKeyToken (); */
                    Method {
                        name: "getInternalKeyToken",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIPK11Token" }],
                        ret: "nsresult",
                    },

                    /* nsIPK11Token findTokenByName (in AUTF8String tokenName); */
                    Method {
                        name: "findTokenByName",
                        abi: "C",
                        params: &[Param { name: "tokenName", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIPK11Token" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator listTokens (); */
                    Method {
                        name: "listTokens",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

