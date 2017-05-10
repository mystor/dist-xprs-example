//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPersistentProperties2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPropertyElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute AUTF8String key; */
                    Method {
                        name: "get_key",
                        abi: "C",
                        params: &[Param { name: "aKey", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_key",
                        abi: "C",
                        params: &[Param { name: "aKey", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString value; */
                    Method {
                        name: "get_value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPersistentProperties",
            base: Some("nsIProperties"),
            methods: Some(&[
                    /* void load (in nsIInputStream input); */
                    Method {
                        name: "load",
                        abi: "C",
                        params: &[Param { name: "input", ty: "*const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* void save (in nsIOutputStream output, in AUTF8String header); */
                    Method {
                        name: "save",
                        abi: "C",
                        params: &[Param { name: "output", ty: "*const nsIOutputStream" }, Param { name: "header", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator enumerate (); */
                    Method {
                        name: "enumerate",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* AString getStringProperty (in AUTF8String key); */
                    Method {
                        name: "getStringProperty",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString setStringProperty (in AUTF8String key, in AString value); */
                    Method {
                        name: "setStringProperty",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "value", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

