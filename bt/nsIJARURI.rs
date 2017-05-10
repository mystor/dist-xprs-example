//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIJARURI.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIJARURI",
            base: Some("nsIURL"),
            methods: Some(&[
                    /* readonly attribute nsIURI JARFile; */
                    Method {
                        name: "get_JARFile",
                        abi: "C",
                        params: &[Param { name: "aJARFile", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String JAREntry; */
                    Method {
                        name: "get_JAREntry",
                        abi: "C",
                        params: &[Param { name: "aJAREntry", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_JAREntry",
                        abi: "C",
                        params: &[Param { name: "aJAREntry", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* nsIJARURI cloneWithJARFile (in nsIURI jarFile); */
                    Method {
                        name: "cloneWithJARFile",
                        abi: "C",
                        params: &[Param { name: "jarFile", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsIJARURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

