//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINativeFileWatcher.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINativeFileWatcherErrorCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void complete (in nsresult xpcomError, in long osError); */
                    Method {
                        name: "complete",
                        abi: "C",
                        params: &[Param { name: "xpcomError", ty: "nsresult" }, Param { name: "osError", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINativeFileWatcherCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void changed (in AString resourcePath, in int32_t flags); */
                    Method {
                        name: "changed",
                        abi: "C",
                        params: &[Param { name: "resourcePath", ty: "*const nsAString" }, Param { name: "flags", ty: "int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINativeFileWatcherSuccessCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void complete (in AString resourcePath); */
                    Method {
                        name: "complete",
                        abi: "C",
                        params: &[Param { name: "resourcePath", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINativeFileWatcherService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addPath (in AString pathToWatch, in nsINativeFileWatcherCallback onChange, [optional] in nsINativeFileWatcherErrorCallback onError, [optional] in nsINativeFileWatcherSuccessCallback onSuccess); */
                    Method {
                        name: "addPath",
                        abi: "C",
                        params: &[Param { name: "pathToWatch", ty: "*const nsAString" }, Param { name: "onChange", ty: "*const nsINativeFileWatcherCallback" }, Param { name: "onError", ty: "*const nsINativeFileWatcherErrorCallback" }, Param { name: "onSuccess", ty: "*const nsINativeFileWatcherSuccessCallback" }],
                        ret: "nsresult",
                    },

                    /* void removePath (in AString pathToUnwatch, in nsINativeFileWatcherCallback onChange, [optional] in nsINativeFileWatcherErrorCallback onError, [optional] in nsINativeFileWatcherSuccessCallback onSuccess); */
                    Method {
                        name: "removePath",
                        abi: "C",
                        params: &[Param { name: "pathToUnwatch", ty: "*const nsAString" }, Param { name: "onChange", ty: "*const nsINativeFileWatcherCallback" }, Param { name: "onError", ty: "*const nsINativeFileWatcherErrorCallback" }, Param { name: "onSuccess", ty: "*const nsINativeFileWatcherSuccessCallback" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

