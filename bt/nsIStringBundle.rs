//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStringBundle.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStringBundle",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIStringBundleService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIStringBundle createBundle (in string aURLSpec); */
                    Method {
                        name: "createBundle",
                        abi: "C",
                        params: &[Param { name: "aURLSpec", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsIStringBundle" }],
                        ret: "nsresult",
                    },

                    /* nsIStringBundle createExtensibleBundle (in string aRegistryKey); */
                    Method {
                        name: "createExtensibleBundle",
                        abi: "C",
                        params: &[Param { name: "aRegistryKey", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsIStringBundle" }],
                        ret: "nsresult",
                    },

                    /* wstring formatStatusMessage (in nsresult aStatus, in wstring aStatusArg); */
                    Method {
                        name: "formatStatusMessage",
                        abi: "C",
                        params: &[Param { name: "aStatus", ty: "nsresult" }, Param { name: "aStatusArg", ty: "*const libc::int16_t" }, Param { name: "_retval", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void flushBundles (); */
                    Method {
                        name: "flushBundles",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

