//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDirectoryService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDirectoryServiceProvider",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIFile getFile (in string prop, out boolean persistent); */
                    Method {
                        name: "getFile",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const libc::c_char" }, Param { name: "persistent", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDirectoryServiceProvider2",
            base: Some("nsIDirectoryServiceProvider"),
            methods: Some(&[
                    /* nsISimpleEnumerator getFiles (in string prop); */
                    Method {
                        name: "getFiles",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDirectoryService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void registerProvider (in nsIDirectoryServiceProvider prov); */
                    Method {
                        name: "registerProvider",
                        abi: "C",
                        params: &[Param { name: "prov", ty: "*const nsIDirectoryServiceProvider" }],
                        ret: "nsresult",
                    },

                    /* void unregisterProvider (in nsIDirectoryServiceProvider prov); */
                    Method {
                        name: "unregisterProvider",
                        abi: "C",
                        params: &[Param { name: "prov", ty: "*const nsIDirectoryServiceProvider" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

