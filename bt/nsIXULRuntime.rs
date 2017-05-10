//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULRuntime.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXULRuntime",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean inSafeMode; */
                    Method {
                        name: "get_inSafeMode",
                        abi: "C",
                        params: &[Param { name: "aInSafeMode", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean logConsoleErrors; */
                    Method {
                        name: "get_logConsoleErrors",
                        abi: "C",
                        params: &[Param { name: "aLogConsoleErrors", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_logConsoleErrors",
                        abi: "C",
                        params: &[Param { name: "aLogConsoleErrors", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String OS; */
                    Method {
                        name: "get_OS",
                        abi: "C",
                        params: &[Param { name: "aOS", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String XPCOMABI; */
                    Method {
                        name: "get_XPCOMABI",
                        abi: "C",
                        params: &[Param { name: "aXPCOMABI", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String widgetToolkit; */
                    Method {
                        name: "get_widgetToolkit",
                        abi: "C",
                        params: &[Param { name: "aWidgetToolkit", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long processType; */
                    Method {
                        name: "get_processType",
                        abi: "C",
                        params: &[Param { name: "aProcessType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long processID; */
                    Method {
                        name: "get_processID",
                        abi: "C",
                        params: &[Param { name: "aProcessID", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint64_t uniqueProcessID; */
                    Method {
                        name: "get_uniqueProcessID",
                        abi: "C",
                        params: &[Param { name: "aUniqueProcessID", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString remoteType; */
                    Method {
                        name: "get_remoteType",
                        abi: "C",
                        params: &[Param { name: "aRemoteType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean browserTabsRemoteAutostart; */
                    Method {
                        name: "get_browserTabsRemoteAutostart",
                        abi: "C",
                        params: &[Param { name: "aBrowserTabsRemoteAutostart", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t maxWebProcessCount; */
                    Method {
                        name: "get_maxWebProcessCount",
                        abi: "C",
                        params: &[Param { name: "aMaxWebProcessCount", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long multiprocessBlockPolicy; */
                    Method {
                        name: "get_multiprocessBlockPolicy",
                        abi: "C",
                        params: &[Param { name: "aMultiprocessBlockPolicy", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean accessibilityEnabled; */
                    Method {
                        name: "get_accessibilityEnabled",
                        abi: "C",
                        params: &[Param { name: "aAccessibilityEnabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean is64Bit; */
                    Method {
                        name: "get_is64Bit",
                        abi: "C",
                        params: &[Param { name: "aIs64Bit", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void invalidateCachesOnRestart (); */
                    Method {
                        name: "invalidateCachesOnRestart",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void ensureContentProcess (); */
                    Method {
                        name: "ensureContentProcess",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute PRTime replacedLockTime; */
                    Method {
                        name: "get_replacedLockTime",
                        abi: "C",
                        params: &[Param { name: "aReplacedLockTime", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString lastRunCrashID; */
                    Method {
                        name: "get_lastRunCrashID",
                        abi: "C",
                        params: &[Param { name: "aLastRunCrashID", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isReleaseOrBeta; */
                    Method {
                        name: "get_isReleaseOrBeta",
                        abi: "C",
                        params: &[Param { name: "aIsReleaseOrBeta", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isOfficialBranding; */
                    Method {
                        name: "get_isOfficialBranding",
                        abi: "C",
                        params: &[Param { name: "aIsOfficialBranding", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String defaultUpdateChannel; */
                    Method {
                        name: "get_defaultUpdateChannel",
                        abi: "C",
                        params: &[Param { name: "aDefaultUpdateChannel", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String distributionID; */
                    Method {
                        name: "get_distributionID",
                        abi: "C",
                        params: &[Param { name: "aDistributionID", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isOfficial; */
                    Method {
                        name: "get_isOfficial",
                        abi: "C",
                        params: &[Param { name: "aIsOfficial", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean windowsDLLBlocklistStatus; */
                    Method {
                        name: "get_windowsDLLBlocklistStatus",
                        abi: "C",
                        params: &[Param { name: "aWindowsDLLBlocklistStatus", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

