//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURLFormatter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURLFormatter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* AString formatURL (in AString aFormat); */
                    Method {
                        name: "formatURL",
                        abi: "C",
                        params: &[Param { name: "aFormat", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString formatURLPref (in AString aPref); */
                    Method {
                        name: "formatURLPref",
                        abi: "C",
                        params: &[Param { name: "aPref", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString trimSensitiveURLs (in AString aMsg); */
                    Method {
                        name: "trimSensitiveURLs",
                        abi: "C",
                        params: &[Param { name: "aMsg", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

