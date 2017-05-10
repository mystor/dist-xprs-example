//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrefService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrefService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void readUserPrefs (in nsIFile aFile); */
                    Method {
                        name: "readUserPrefs",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void resetPrefs (); */
                    Method {
                        name: "resetPrefs",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void resetUserPrefs (); */
                    Method {
                        name: "resetUserPrefs",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void savePrefFile (in nsIFile aFile); */
                    Method {
                        name: "savePrefFile",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* nsIPrefBranch getBranch (in string aPrefRoot); */
                    Method {
                        name: "getBranch",
                        abi: "C",
                        params: &[Param { name: "aPrefRoot", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsIPrefBranch" }],
                        ret: "nsresult",
                    },

                    /* nsIPrefBranch getDefaultBranch (in string aPrefRoot); */
                    Method {
                        name: "getDefaultBranch",
                        abi: "C",
                        params: &[Param { name: "aPrefRoot", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsIPrefBranch" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean dirty; */
                    Method {
                        name: "get_dirty",
                        abi: "C",
                        params: &[Param { name: "aDirty", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

