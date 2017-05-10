//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIExternalHelperAppService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIExternalHelperAppService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIStreamListener doContent (in ACString aMimeContentType, in nsIRequest aRequest, in nsIInterfaceRequestor aContentContext, in boolean aForceSave, [optional] in nsIInterfaceRequestor aWindowContext); */
                    Method {
                        name: "doContent",
                        abi: "C",
                        params: &[Param { name: "aMimeContentType", ty: "*const nsACString" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aContentContext", ty: "*const nsIInterfaceRequestor" }, Param { name: "aForceSave", ty: "bool" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }, Param { name: "_retval", ty: "*mut *const nsIStreamListener" }],
                        ret: "nsresult",
                    },

                    /* boolean applyDecodingForExtension (in AUTF8String aExtension, in ACString aEncodingType); */
                    Method {
                        name: "applyDecodingForExtension",
                        abi: "C",
                        params: &[Param { name: "aExtension", ty: "*const nsACString" }, Param { name: "aEncodingType", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsPIExternalAppLauncher",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void deleteTemporaryFileOnExit (in nsIFile aTemporaryFile); */
                    Method {
                        name: "deleteTemporaryFileOnExit",
                        abi: "C",
                        params: &[Param { name: "aTemporaryFile", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void deleteTemporaryPrivateFileWhenPossible (in nsIFile aTemporaryFile); */
                    Method {
                        name: "deleteTemporaryPrivateFileWhenPossible",
                        abi: "C",
                        params: &[Param { name: "aTemporaryFile", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIHelperAppLauncher",
            base: Some("nsICancelable"),
            methods: Some(&[
                    /* readonly attribute nsIMIMEInfo MIMEInfo; */
                    Method {
                        name: "get_MIMEInfo",
                        abi: "C",
                        params: &[Param { name: "aMIMEInfo", ty: "*mut *const nsIMIMEInfo" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI source; */
                    Method {
                        name: "get_source",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString suggestedFileName; */
                    Method {
                        name: "get_suggestedFileName",
                        abi: "C",
                        params: &[Param { name: "aSuggestedFileName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void saveToDisk (in nsIFile aNewFileLocation, in boolean aRememberThisPreference); */
                    Method {
                        name: "saveToDisk",
                        abi: "C",
                        params: &[Param { name: "aNewFileLocation", ty: "*const nsIFile" }, Param { name: "aRememberThisPreference", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void launchWithApplication (in nsIFile aApplication, in boolean aRememberThisPreference); */
                    Method {
                        name: "launchWithApplication",
                        abi: "C",
                        params: &[Param { name: "aApplication", ty: "*const nsIFile" }, Param { name: "aRememberThisPreference", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void saveDestinationAvailable (in nsIFile aFile); */
                    Method {
                        name: "saveDestinationAvailable",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void setWebProgressListener (in nsIWebProgressListener2 aWebProgressListener); */
                    Method {
                        name: "setWebProgressListener",
                        abi: "C",
                        params: &[Param { name: "aWebProgressListener", ty: "*const nsIWebProgressListener2" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIFile targetFile; */
                    Method {
                        name: "get_targetFile",
                        abi: "C",
                        params: &[Param { name: "aTargetFile", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean targetFileIsExecutable; */
                    Method {
                        name: "get_targetFileIsExecutable",
                        abi: "C",
                        params: &[Param { name: "aTargetFileIsExecutable", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute PRTime timeDownloadStarted; */
                    Method {
                        name: "get_timeDownloadStarted",
                        abi: "C",
                        params: &[Param { name: "aTimeDownloadStarted", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute int64_t contentLength; */
                    Method {
                        name: "get_contentLength",
                        abi: "C",
                        params: &[Param { name: "aContentLength", ty: "*mut int64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

