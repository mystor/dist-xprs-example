//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentPrefService2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentPrefService2",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIContentPrefCallback2",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handleResult (in nsIContentPref pref); */
                    Method {
                        name: "handleResult",
                        abi: "C",
                        params: &[Param { name: "pref", ty: "*const nsIContentPref" }],
                        ret: "nsresult",
                    },

                    /* void handleError (in nsresult error); */
                    Method {
                        name: "handleError",
                        abi: "C",
                        params: &[Param { name: "error", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void handleCompletion (in unsigned short reason); */
                    Method {
                        name: "handleCompletion",
                        abi: "C",
                        params: &[Param { name: "reason", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIContentPref",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AString domain; */
                    Method {
                        name: "get_domain",
                        abi: "C",
                        params: &[Param { name: "aDomain", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIVariant value; */
                    Method {
                        name: "get_value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

