//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILocaleService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILocaleService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsILocale newLocale (in AString aLocale); */
                    Method {
                        name: "newLocale",
                        abi: "C",
                        params: &[Param { name: "aLocale", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsILocale" }],
                        ret: "nsresult",
                    },

                    /* nsILocale getSystemLocale (); */
                    Method {
                        name: "getSystemLocale",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsILocale" }],
                        ret: "nsresult",
                    },

                    /* nsILocale getApplicationLocale (); */
                    Method {
                        name: "getApplicationLocale",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsILocale" }],
                        ret: "nsresult",
                    },

                    /* nsILocale getLocaleFromAcceptLanguage (in string acceptLanguage); */
                    Method {
                        name: "getLocaleFromAcceptLanguage",
                        abi: "C",
                        params: &[Param { name: "acceptLanguage", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsILocale" }],
                        ret: "nsresult",
                    },

                    /* AString getLocaleComponentForUserAgent (); */
                    Method {
                        name: "getLocaleComponentForUserAgent",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

