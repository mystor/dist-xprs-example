//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void openAsyncDatabase (in nsIVariant aDatabaseStore, [optional] in nsIPropertyBag2 aOptions, in mozIStorageCompletionCallback aCallback); */
                    Method {
                        name: "openAsyncDatabase",
                        abi: "C",
                        params: &[Param { name: "aDatabaseStore", ty: "*const nsIVariant" }, Param { name: "aOptions", ty: "*const nsIPropertyBag2" }, Param { name: "aCallback", ty: "*const mozIStorageCompletionCallback" }],
                        ret: "nsresult",
                    },

                    /* mozIStorageConnection openSpecialDatabase (in string aStorageKey); */
                    Method {
                        name: "openSpecialDatabase",
                        abi: "C",
                        params: &[Param { name: "aStorageKey", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const mozIStorageConnection" }],
                        ret: "nsresult",
                    },

                    /* mozIStorageConnection openDatabase (in nsIFile aDatabaseFile); */
                    Method {
                        name: "openDatabase",
                        abi: "C",
                        params: &[Param { name: "aDatabaseFile", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut *const mozIStorageConnection" }],
                        ret: "nsresult",
                    },

                    /* mozIStorageConnection openUnsharedDatabase (in nsIFile aDatabaseFile); */
                    Method {
                        name: "openUnsharedDatabase",
                        abi: "C",
                        params: &[Param { name: "aDatabaseFile", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut *const mozIStorageConnection" }],
                        ret: "nsresult",
                    },

                    /* mozIStorageConnection openDatabaseWithFileURL (in nsIFileURL aFileURL); */
                    Method {
                        name: "openDatabaseWithFileURL",
                        abi: "C",
                        params: &[Param { name: "aFileURL", ty: "*const nsIFileURL" }, Param { name: "_retval", ty: "*mut *const mozIStorageConnection" }],
                        ret: "nsresult",
                    },

                    /* nsIFile backupDatabaseFile (in nsIFile aDBFile, in AString aBackupFileName, [optional] in nsIFile aBackupParentDirectory); */
                    Method {
                        name: "backupDatabaseFile",
                        abi: "C",
                        params: &[Param { name: "aDBFile", ty: "*const nsIFile" }, Param { name: "aBackupFileName", ty: "*const nsAString" }, Param { name: "aBackupParentDirectory", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

