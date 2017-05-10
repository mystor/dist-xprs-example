//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICrashReporter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICrashReporter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean enabled; */
                    Method {
                        name: "get_enabled",
                        abi: "C",
                        params: &[Param { name: "aEnabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void setEnabled (in bool enabled); */
                    Method {
                        name: "setEnabled",
                        abi: "C",
                        params: &[Param { name: "enabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIURL serverURL; */
                    Method {
                        name: "get_serverURL",
                        abi: "C",
                        params: &[Param { name: "aServerURL", ty: "*mut *const nsIURL" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_serverURL",
                        abi: "C",
                        params: &[Param { name: "aServerURL", ty: "*const nsIURL" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIFile minidumpPath; */
                    Method {
                        name: "get_minidumpPath",
                        abi: "C",
                        params: &[Param { name: "aMinidumpPath", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_minidumpPath",
                        abi: "C",
                        params: &[Param { name: "aMinidumpPath", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* nsIFile getMinidumpForID (in AString id); */
                    Method {
                        name: "getMinidumpForID",
                        abi: "C",
                        params: &[Param { name: "id", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* nsIFile getExtraFileForID (in AString id); */
                    Method {
                        name: "getExtraFileForID",
                        abi: "C",
                        params: &[Param { name: "id", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void annotateCrashReport (in AUTF8String key, in AUTF8String data); */
                    Method {
                        name: "annotateCrashReport",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "data", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void appendAppNotesToCrashReport (in ACString data); */
                    Method {
                        name: "appendAppNotesToCrashReport",
                        abi: "C",
                        params: &[Param { name: "data", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void registerAppMemory (in unsigned long long ptr, in unsigned long long size); */
                    Method {
                        name: "registerAppMemory",
                        abi: "C",
                        params: &[Param { name: "ptr", ty: "libc::uint64_t" }, Param { name: "size", ty: "libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void writeMinidumpForException (in voidPtr aExceptionInfo); */
                    Method {
                        name: "writeMinidumpForException",
                        abi: "C",
                        params: &[Param { name: "aExceptionInfo", ty: "*const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void appendObjCExceptionInfoToAppNotes (in voidPtr aException); */
                    Method {
                        name: "appendObjCExceptionInfoToAppNotes",
                        abi: "C",
                        params: &[Param { name: "aException", ty: "*const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean submitReports; */
                    Method {
                        name: "get_submitReports",
                        abi: "C",
                        params: &[Param { name: "aSubmitReports", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_submitReports",
                        abi: "C",
                        params: &[Param { name: "aSubmitReports", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void UpdateCrashEventsDir (); */
                    Method {
                        name: "UpdateCrashEventsDir",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void saveMemoryReport (); */
                    Method {
                        name: "saveMemoryReport",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void setTelemetrySessionId (in AUTF8String id); */
                    Method {
                        name: "setTelemetrySessionId",
                        abi: "C",
                        params: &[Param { name: "id", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

