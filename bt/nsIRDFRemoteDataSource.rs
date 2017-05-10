//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFRemoteDataSource.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRDFRemoteDataSource",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean loaded; */
                    Method {
                        name: "get_loaded",
                        abi: "C",
                        params: &[Param { name: "aLoaded", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void Init (in string aURI); */
                    Method {
                        name: "Init",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void Refresh (in boolean aBlocking); */
                    Method {
                        name: "Refresh",
                        abi: "C",
                        params: &[Param { name: "aBlocking", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void Flush (); */
                    Method {
                        name: "Flush",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void FlushTo (in string aURI); */
                    Method {
                        name: "FlushTo",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

