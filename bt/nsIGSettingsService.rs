//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGSettingsService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIGSettingsCollection",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setString (in AUTF8String key, in AUTF8String value); */
                    Method {
                        name: "setString",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "value", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void setBoolean (in AUTF8String key, in boolean value); */
                    Method {
                        name: "setBoolean",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "value", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void setInt (in AUTF8String key, in long value); */
                    Method {
                        name: "setInt",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "value", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getString (in AUTF8String key); */
                    Method {
                        name: "getString",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean getBoolean (in AUTF8String key); */
                    Method {
                        name: "getBoolean",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* long getInt (in AUTF8String key); */
                    Method {
                        name: "getInt",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIArray getStringList (in AUTF8String key); */
                    Method {
                        name: "getStringList",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIGSettingsService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIGSettingsCollection getCollectionForSchema (in AUTF8String schema); */
                    Method {
                        name: "getCollectionForSchema",
                        abi: "C",
                        params: &[Param { name: "schema", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIGSettingsCollection" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

