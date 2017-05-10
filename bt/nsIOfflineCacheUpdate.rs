//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIOfflineCacheUpdate.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIOfflineCacheUpdateObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void updateStateChanged (in nsIOfflineCacheUpdate aUpdate, in uint32_t state); */
                    Method {
                        name: "updateStateChanged",
                        abi: "C",
                        params: &[Param { name: "aUpdate", ty: "*const nsIOfflineCacheUpdate" }, Param { name: "state", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void applicationCacheAvailable (in nsIApplicationCache applicationCache); */
                    Method {
                        name: "applicationCacheAvailable",
                        abi: "C",
                        params: &[Param { name: "applicationCache", ty: "*const nsIApplicationCache" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIOfflineCacheUpdate",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned short status; */
                    Method {
                        name: "get_status",
                        abi: "C",
                        params: &[Param { name: "aStatus", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean partial; */
                    Method {
                        name: "get_partial",
                        abi: "C",
                        params: &[Param { name: "aPartial", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isUpgrade; */
                    Method {
                        name: "get_isUpgrade",
                        abi: "C",
                        params: &[Param { name: "aIsUpgrade", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString updateDomain; */
                    Method {
                        name: "get_updateDomain",
                        abi: "C",
                        params: &[Param { name: "aUpdateDomain", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI manifestURI; */
                    Method {
                        name: "get_manifestURI",
                        abi: "C",
                        params: &[Param { name: "aManifestURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean succeeded; */
                    Method {
                        name: "get_succeeded",
                        abi: "C",
                        params: &[Param { name: "aSucceeded", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void init (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in nsIDOMDocument aDocument, [optional] in nsIFile aCustomProfileDir); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aManifestURI", ty: "*const nsIURI" }, Param { name: "aDocumentURI", ty: "*const nsIURI" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aDocument", ty: "*const nsIDOMDocument" }, Param { name: "aCustomProfileDir", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void initPartial (in nsIURI aManifestURI, in ACString aClientID, in nsIURI aDocumentURI, in nsIPrincipal aPrincipal); */
                    Method {
                        name: "initPartial",
                        abi: "C",
                        params: &[Param { name: "aManifestURI", ty: "*const nsIURI" }, Param { name: "aClientID", ty: "*const nsACString" }, Param { name: "aDocumentURI", ty: "*const nsIURI" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    /* void initForUpdateCheck (in nsIURI aManifestURI, in nsIPrincipal aLoadingPrincipal, in nsIObserver aObserver); */
                    Method {
                        name: "initForUpdateCheck",
                        abi: "C",
                        params: &[Param { name: "aManifestURI", ty: "*const nsIURI" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aObserver", ty: "*const nsIObserver" }],
                        ret: "nsresult",
                    },

                    /* void addDynamicURI (in nsIURI aURI); */
                    Method {
                        name: "addDynamicURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* void schedule (); */
                    Method {
                        name: "schedule",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void addObserver (in nsIOfflineCacheUpdateObserver aObserver, [optional] in boolean aHoldWeak); */
                    Method {
                        name: "addObserver",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIOfflineCacheUpdateObserver" }, Param { name: "aHoldWeak", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void removeObserver (in nsIOfflineCacheUpdateObserver aObserver); */
                    Method {
                        name: "removeObserver",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIOfflineCacheUpdateObserver" }],
                        ret: "nsresult",
                    },

                    /* void cancel (); */
                    Method {
                        name: "cancel",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint64_t byteProgress; */
                    Method {
                        name: "get_byteProgress",
                        abi: "C",
                        params: &[Param { name: "aByteProgress", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIOfflineCacheUpdateService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long numUpdates; */
                    Method {
                        name: "get_numUpdates",
                        abi: "C",
                        params: &[Param { name: "aNumUpdates", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIOfflineCacheUpdate getUpdate (in unsigned long index); */
                    Method {
                        name: "getUpdate",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIOfflineCacheUpdate" }],
                        ret: "nsresult",
                    },

                    /* nsIOfflineCacheUpdate scheduleUpdate (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in mozIDOMWindow aWindow); */
                    Method {
                        name: "scheduleUpdate",
                        abi: "C",
                        params: &[Param { name: "aManifestURI", ty: "*const nsIURI" }, Param { name: "aDocumentURI", ty: "*const nsIURI" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aWindow", ty: "*const mozIDOMWindow" }, Param { name: "_retval", ty: "*mut *const nsIOfflineCacheUpdate" }],
                        ret: "nsresult",
                    },

                    /* nsIOfflineCacheUpdate scheduleAppUpdate (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in nsIFile aProfileDir); */
                    Method {
                        name: "scheduleAppUpdate",
                        abi: "C",
                        params: &[Param { name: "aManifestURI", ty: "*const nsIURI" }, Param { name: "aDocumentURI", ty: "*const nsIURI" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aProfileDir", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut *const nsIOfflineCacheUpdate" }],
                        ret: "nsresult",
                    },

                    /* void scheduleOnDocumentStop (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in nsIDOMDocument aDocument); */
                    Method {
                        name: "scheduleOnDocumentStop",
                        abi: "C",
                        params: &[Param { name: "aManifestURI", ty: "*const nsIURI" }, Param { name: "aDocumentURI", ty: "*const nsIURI" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aDocument", ty: "*const nsIDOMDocument" }],
                        ret: "nsresult",
                    },

                    /* void checkForUpdate (in nsIURI aManifestURI, in nsIPrincipal aLoadingPrincipal, in nsIObserver aObserver); */
                    Method {
                        name: "checkForUpdate",
                        abi: "C",
                        params: &[Param { name: "aManifestURI", ty: "*const nsIURI" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aObserver", ty: "*const nsIObserver" }],
                        ret: "nsresult",
                    },

                    /* boolean offlineAppAllowed (in nsIPrincipal aPrincipal, in nsIPrefBranch aPrefBranch); */
                    Method {
                        name: "offlineAppAllowed",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aPrefBranch", ty: "*const nsIPrefBranch" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean offlineAppAllowedForURI (in nsIURI aURI, in nsIPrefBranch aPrefBranch); */
                    Method {
                        name: "offlineAppAllowedForURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aPrefBranch", ty: "*const nsIPrefBranch" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void allowOfflineApp (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "allowOfflineApp",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

