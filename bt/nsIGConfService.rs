//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGConfService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIGConfService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean getBool (in AUTF8String key); */
                    Method {
                        name: "getBool",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getString (in AUTF8String key); */
                    Method {
                        name: "getString",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* long getInt (in AUTF8String key); */
                    Method {
                        name: "getInt",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* float getFloat (in AUTF8String key); */
                    Method {
                        name: "getFloat",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* nsIArray getStringList (in AUTF8String key); */
                    Method {
                        name: "getStringList",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* void setBool (in AUTF8String key, in boolean value); */
                    Method {
                        name: "setBool",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "value", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void setString (in AUTF8String key, in AUTF8String value); */
                    Method {
                        name: "setString",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "value", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void setInt (in AUTF8String key, in long value); */
                    Method {
                        name: "setInt",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "value", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setFloat (in AUTF8String key, in float value); */
                    Method {
                        name: "setFloat",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "value", ty: "libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getAppForProtocol (in AUTF8String scheme, out boolean enabled); */
                    Method {
                        name: "getAppForProtocol",
                        abi: "C",
                        params: &[Param { name: "scheme", ty: "*const nsACString" }, Param { name: "enabled", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean handlerRequiresTerminal (in AUTF8String scheme); */
                    Method {
                        name: "handlerRequiresTerminal",
                        abi: "C",
                        params: &[Param { name: "scheme", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void setAppForProtocol (in AUTF8String scheme, in AUTF8String command); */
                    Method {
                        name: "setAppForProtocol",
                        abi: "C",
                        params: &[Param { name: "scheme", ty: "*const nsACString" }, Param { name: "command", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

