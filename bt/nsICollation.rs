//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICollation.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICollationFactory",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsICollation CreateCollation (); */
                    Method {
                        name: "CreateCollation",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsICollation" }],
                        ret: "nsresult",
                    },

                    /* nsICollation CreateCollationForLocale (in ACString locale); */
                    Method {
                        name: "CreateCollationForLocale",
                        abi: "C",
                        params: &[Param { name: "locale", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsICollation" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsICollation",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

