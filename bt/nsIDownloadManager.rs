//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDownloadManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDownloadManagerResult",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handleResult (in nsresult aStatus, in nsIDownload aDownload); */
                    Method {
                        name: "handleResult",
                        abi: "C",
                        params: &[Param { name: "aStatus", ty: "nsresult" }, Param { name: "aDownload", ty: "*const nsIDownload" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDownloadManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIDownload addDownload (in short aDownloadType, in nsIURI aSource, in nsIURI aTarget, in AString aDisplayName, in nsIMIMEInfo aMIMEInfo, in PRTime aStartTime, in nsIFile aTempFile, in nsICancelable aCancelable, in boolean aIsPrivate); */
                    Method {
                        name: "addDownload",
                        abi: "C",
                        params: &[Param { name: "aDownloadType", ty: "libc::int16_t" }, Param { name: "aSource", ty: "*const nsIURI" }, Param { name: "aTarget", ty: "*const nsIURI" }, Param { name: "aDisplayName", ty: "*const nsAString" }, Param { name: "aMIMEInfo", ty: "*const nsIMIMEInfo" }, Param { name: "aStartTime", ty: "PRTime" }, Param { name: "aTempFile", ty: "*const nsIFile" }, Param { name: "aCancelable", ty: "*const nsICancelable" }, Param { name: "aIsPrivate", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIDownload" }],
                        ret: "nsresult",
                    },

                    /* nsIDownload getDownload (in unsigned long aID); */
                    Method {
                        name: "getDownload",
                        abi: "C",
                        params: &[Param { name: "aID", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIDownload" }],
                        ret: "nsresult",
                    },

                    /* void getDownloadByGUID (in ACString aGUID, in nsIDownloadManagerResult aCallback); */
                    Method {
                        name: "getDownloadByGUID",
                        abi: "C",
                        params: &[Param { name: "aGUID", ty: "*const nsACString" }, Param { name: "aCallback", ty: "*const nsIDownloadManagerResult" }],
                        ret: "nsresult",
                    },

                    /* void cancelDownload (in unsigned long aID); */
                    Method {
                        name: "cancelDownload",
                        abi: "C",
                        params: &[Param { name: "aID", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void removeDownload (in unsigned long aID); */
                    Method {
                        name: "removeDownload",
                        abi: "C",
                        params: &[Param { name: "aID", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void removeDownloadsByTimeframe (in long long aBeginTime, in long long aEndTime); */
                    Method {
                        name: "removeDownloadsByTimeframe",
                        abi: "C",
                        params: &[Param { name: "aBeginTime", ty: "libc::int64_t" }, Param { name: "aEndTime", ty: "libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* void pauseDownload (in unsigned long aID); */
                    Method {
                        name: "pauseDownload",
                        abi: "C",
                        params: &[Param { name: "aID", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void resumeDownload (in unsigned long aID); */
                    Method {
                        name: "resumeDownload",
                        abi: "C",
                        params: &[Param { name: "aID", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void retryDownload (in unsigned long aID); */
                    Method {
                        name: "retryDownload",
                        abi: "C",
                        params: &[Param { name: "aID", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute mozIStorageConnection DBConnection; */
                    Method {
                        name: "get_DBConnection",
                        abi: "C",
                        params: &[Param { name: "aDBConnection", ty: "*mut *const mozIStorageConnection" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute mozIStorageConnection privateDBConnection; */
                    Method {
                        name: "get_privateDBConnection",
                        abi: "C",
                        params: &[Param { name: "aPrivateDBConnection", ty: "*mut *const mozIStorageConnection" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean canCleanUp; */
                    Method {
                        name: "get_canCleanUp",
                        abi: "C",
                        params: &[Param { name: "aCanCleanUp", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean canCleanUpPrivate; */
                    Method {
                        name: "get_canCleanUpPrivate",
                        abi: "C",
                        params: &[Param { name: "aCanCleanUpPrivate", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void cleanUp (); */
                    Method {
                        name: "cleanUp",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void cleanUpPrivate (); */
                    Method {
                        name: "cleanUpPrivate",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute long activeDownloadCount; */
                    Method {
                        name: "get_activeDownloadCount",
                        abi: "C",
                        params: &[Param { name: "aActiveDownloadCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long activePrivateDownloadCount; */
                    Method {
                        name: "get_activePrivateDownloadCount",
                        abi: "C",
                        params: &[Param { name: "aActivePrivateDownloadCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsISimpleEnumerator activeDownloads; */
                    Method {
                        name: "get_activeDownloads",
                        abi: "C",
                        params: &[Param { name: "aActiveDownloads", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsISimpleEnumerator activePrivateDownloads; */
                    Method {
                        name: "get_activePrivateDownloads",
                        abi: "C",
                        params: &[Param { name: "aActivePrivateDownloads", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* void addListener (in nsIDownloadProgressListener aListener); */
                    Method {
                        name: "addListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIDownloadProgressListener" }],
                        ret: "nsresult",
                    },

                    /* void addPrivacyAwareListener (in nsIDownloadProgressListener aListener); */
                    Method {
                        name: "addPrivacyAwareListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIDownloadProgressListener" }],
                        ret: "nsresult",
                    },

                    /* void removeListener (in nsIDownloadProgressListener aListener); */
                    Method {
                        name: "removeListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIDownloadProgressListener" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIFile defaultDownloadsDirectory; */
                    Method {
                        name: "get_defaultDownloadsDirectory",
                        abi: "C",
                        params: &[Param { name: "aDefaultDownloadsDirectory", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIFile userDownloadsDirectory; */
                    Method {
                        name: "get_userDownloadsDirectory",
                        abi: "C",
                        params: &[Param { name: "aUserDownloadsDirectory", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

