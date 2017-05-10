//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIErrorService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIErrorService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void registerErrorStringBundle (in short errorModule, in string stringBundleURL); */
                    Method {
                        name: "registerErrorStringBundle",
                        abi: "C",
                        params: &[Param { name: "errorModule", ty: "libc::int16_t" }, Param { name: "stringBundleURL", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void unregisterErrorStringBundle (in short errorModule); */
                    Method {
                        name: "unregisterErrorStringBundle",
                        abi: "C",
                        params: &[Param { name: "errorModule", ty: "libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* string getErrorStringBundle (in short errorModule); */
                    Method {
                        name: "getErrorStringBundle",
                        abi: "C",
                        params: &[Param { name: "errorModule", ty: "libc::int16_t" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

