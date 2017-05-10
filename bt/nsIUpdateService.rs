//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUpdateService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUpdatePatch",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute AString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString URL; */
                    Method {
                        name: "get_URL",
                        abi: "C",
                        params: &[Param { name: "aURL", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_URL",
                        abi: "C",
                        params: &[Param { name: "aURL", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString finalURL; */
                    Method {
                        name: "get_finalURL",
                        abi: "C",
                        params: &[Param { name: "aFinalURL", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_finalURL",
                        abi: "C",
                        params: &[Param { name: "aFinalURL", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString hashFunction; */
                    Method {
                        name: "get_hashFunction",
                        abi: "C",
                        params: &[Param { name: "aHashFunction", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_hashFunction",
                        abi: "C",
                        params: &[Param { name: "aHashFunction", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString hashValue; */
                    Method {
                        name: "get_hashValue",
                        abi: "C",
                        params: &[Param { name: "aHashValue", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_hashValue",
                        abi: "C",
                        params: &[Param { name: "aHashValue", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long size; */
                    Method {
                        name: "get_size",
                        abi: "C",
                        params: &[Param { name: "aSize", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_size",
                        abi: "C",
                        params: &[Param { name: "aSize", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute AString state; */
                    Method {
                        name: "get_state",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_state",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean selected; */
                    Method {
                        name: "get_selected",
                        abi: "C",
                        params: &[Param { name: "aSelected", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_selected",
                        abi: "C",
                        params: &[Param { name: "aSelected", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMElement serialize (in nsIDOMDocument updates); */
                    Method {
                        name: "serialize",
                        abi: "C",
                        params: &[Param { name: "updates", ty: "*const nsIDOMDocument" }, Param { name: "_retval", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIUpdate",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute AString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString displayVersion; */
                    Method {
                        name: "get_displayVersion",
                        abi: "C",
                        params: &[Param { name: "aDisplayVersion", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_displayVersion",
                        abi: "C",
                        params: &[Param { name: "aDisplayVersion", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString appVersion; */
                    Method {
                        name: "get_appVersion",
                        abi: "C",
                        params: &[Param { name: "aAppVersion", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_appVersion",
                        abi: "C",
                        params: &[Param { name: "aAppVersion", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString previousAppVersion; */
                    Method {
                        name: "get_previousAppVersion",
                        abi: "C",
                        params: &[Param { name: "aPreviousAppVersion", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_previousAppVersion",
                        abi: "C",
                        params: &[Param { name: "aPreviousAppVersion", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString buildID; */
                    Method {
                        name: "get_buildID",
                        abi: "C",
                        params: &[Param { name: "aBuildID", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_buildID",
                        abi: "C",
                        params: &[Param { name: "aBuildID", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString detailsURL; */
                    Method {
                        name: "get_detailsURL",
                        abi: "C",
                        params: &[Param { name: "aDetailsURL", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_detailsURL",
                        abi: "C",
                        params: &[Param { name: "aDetailsURL", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString serviceURL; */
                    Method {
                        name: "get_serviceURL",
                        abi: "C",
                        params: &[Param { name: "aServiceURL", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_serviceURL",
                        abi: "C",
                        params: &[Param { name: "aServiceURL", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString channel; */
                    Method {
                        name: "get_channel",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_channel",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean showPrompt; */
                    Method {
                        name: "get_showPrompt",
                        abi: "C",
                        params: &[Param { name: "aShowPrompt", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_showPrompt",
                        abi: "C",
                        params: &[Param { name: "aShowPrompt", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean showNeverForVersion; */
                    Method {
                        name: "get_showNeverForVersion",
                        abi: "C",
                        params: &[Param { name: "aShowNeverForVersion", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_showNeverForVersion",
                        abi: "C",
                        params: &[Param { name: "aShowNeverForVersion", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean unsupported; */
                    Method {
                        name: "get_unsupported",
                        abi: "C",
                        params: &[Param { name: "aUnsupported", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_unsupported",
                        abi: "C",
                        params: &[Param { name: "aUnsupported", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute long long promptWaitTime; */
                    Method {
                        name: "get_promptWaitTime",
                        abi: "C",
                        params: &[Param { name: "aPromptWaitTime", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_promptWaitTime",
                        abi: "C",
                        params: &[Param { name: "aPromptWaitTime", ty: "libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean isCompleteUpdate; */
                    Method {
                        name: "get_isCompleteUpdate",
                        abi: "C",
                        params: &[Param { name: "aIsCompleteUpdate", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_isCompleteUpdate",
                        abi: "C",
                        params: &[Param { name: "aIsCompleteUpdate", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean isSecurityUpdate; */
                    Method {
                        name: "get_isSecurityUpdate",
                        abi: "C",
                        params: &[Param { name: "aIsSecurityUpdate", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_isSecurityUpdate",
                        abi: "C",
                        params: &[Param { name: "aIsSecurityUpdate", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute long long installDate; */
                    Method {
                        name: "get_installDate",
                        abi: "C",
                        params: &[Param { name: "aInstallDate", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_installDate",
                        abi: "C",
                        params: &[Param { name: "aInstallDate", ty: "libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* attribute AString statusText; */
                    Method {
                        name: "get_statusText",
                        abi: "C",
                        params: &[Param { name: "aStatusText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_statusText",
                        abi: "C",
                        params: &[Param { name: "aStatusText", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIUpdatePatch selectedPatch; */
                    Method {
                        name: "get_selectedPatch",
                        abi: "C",
                        params: &[Param { name: "aSelectedPatch", ty: "*mut *const nsIUpdatePatch" }],
                        ret: "nsresult",
                    },

                    /* attribute AString state; */
                    Method {
                        name: "get_state",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_state",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute long errorCode; */
                    Method {
                        name: "get_errorCode",
                        abi: "C",
                        params: &[Param { name: "aErrorCode", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_errorCode",
                        abi: "C",
                        params: &[Param { name: "aErrorCode", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean elevationFailure; */
                    Method {
                        name: "get_elevationFailure",
                        abi: "C",
                        params: &[Param { name: "aElevationFailure", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_elevationFailure",
                        abi: "C",
                        params: &[Param { name: "aElevationFailure", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long patchCount; */
                    Method {
                        name: "get_patchCount",
                        abi: "C",
                        params: &[Param { name: "aPatchCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIUpdatePatch getPatchAt (in unsigned long index); */
                    Method {
                        name: "getPatchAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIUpdatePatch" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMElement serialize (in nsIDOMDocument updates); */
                    Method {
                        name: "serialize",
                        abi: "C",
                        params: &[Param { name: "updates", ty: "*const nsIDOMDocument" }, Param { name: "_retval", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIUpdateCheckListener",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIUpdateChecker",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void checkForUpdates (in nsIUpdateCheckListener listener, in boolean force); */
                    Method {
                        name: "checkForUpdates",
                        abi: "C",
                        params: &[Param { name: "listener", ty: "*const nsIUpdateCheckListener" }, Param { name: "force", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void stopChecking (in unsigned short duration); */
                    Method {
                        name: "stopChecking",
                        abi: "C",
                        params: &[Param { name: "duration", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIApplicationUpdateService",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIUpdateProcessor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void processUpdate (in nsIUpdate update); */
                    Method {
                        name: "processUpdate",
                        abi: "C",
                        params: &[Param { name: "update", ty: "*const nsIUpdate" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIUpdateManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIUpdate getUpdateAt (in long index); */
                    Method {
                        name: "getUpdateAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIUpdate" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long updateCount; */
                    Method {
                        name: "get_updateCount",
                        abi: "C",
                        params: &[Param { name: "aUpdateCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIUpdate activeUpdate; */
                    Method {
                        name: "get_activeUpdate",
                        abi: "C",
                        params: &[Param { name: "aActiveUpdate", ty: "*mut *const nsIUpdate" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_activeUpdate",
                        abi: "C",
                        params: &[Param { name: "aActiveUpdate", ty: "*const nsIUpdate" }],
                        ret: "nsresult",
                    },

                    /* void saveUpdates (); */
                    Method {
                        name: "saveUpdates",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void refreshUpdateStatus (); */
                    Method {
                        name: "refreshUpdateStatus",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void elevationOptedIn (); */
                    Method {
                        name: "elevationOptedIn",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void cleanupActiveUpdate (); */
                    Method {
                        name: "cleanupActiveUpdate",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIUpdatePrompt",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void checkForUpdates (); */
                    Method {
                        name: "checkForUpdates",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void showUpdateAvailable (in nsIUpdate update); */
                    Method {
                        name: "showUpdateAvailable",
                        abi: "C",
                        params: &[Param { name: "update", ty: "*const nsIUpdate" }],
                        ret: "nsresult",
                    },

                    /* void showUpdateDownloaded (in nsIUpdate update, [optional] in boolean background); */
                    Method {
                        name: "showUpdateDownloaded",
                        abi: "C",
                        params: &[Param { name: "update", ty: "*const nsIUpdate" }, Param { name: "background", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void showUpdateError (in nsIUpdate update); */
                    Method {
                        name: "showUpdateError",
                        abi: "C",
                        params: &[Param { name: "update", ty: "*const nsIUpdate" }],
                        ret: "nsresult",
                    },

                    /* void showUpdateHistory (in nsIDOMWindow parent); */
                    Method {
                        name: "showUpdateHistory",
                        abi: "C",
                        params: &[Param { name: "parent", ty: "*const nsIDOMWindow" }],
                        ret: "nsresult",
                    },

                    /* void showUpdateElevationRequired (); */
                    Method {
                        name: "showUpdateElevationRequired",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

