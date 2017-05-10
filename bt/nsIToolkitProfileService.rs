//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIToolkitProfileService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIToolkitProfileService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute boolean startWithLastProfile; */
                    Method {
                        name: "get_startWithLastProfile",
                        abi: "C",
                        params: &[Param { name: "aStartWithLastProfile", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_startWithLastProfile",
                        abi: "C",
                        params: &[Param { name: "aStartWithLastProfile", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean startOffline; */
                    Method {
                        name: "get_startOffline",
                        abi: "C",
                        params: &[Param { name: "aStartOffline", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_startOffline",
                        abi: "C",
                        params: &[Param { name: "aStartOffline", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsISimpleEnumerator profiles; */
                    Method {
                        name: "get_profiles",
                        abi: "C",
                        params: &[Param { name: "aProfiles", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIToolkitProfile selectedProfile; */
                    Method {
                        name: "get_selectedProfile",
                        abi: "C",
                        params: &[Param { name: "aSelectedProfile", ty: "*mut *const nsIToolkitProfile" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_selectedProfile",
                        abi: "C",
                        params: &[Param { name: "aSelectedProfile", ty: "*const nsIToolkitProfile" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIToolkitProfile defaultProfile; */
                    Method {
                        name: "get_defaultProfile",
                        abi: "C",
                        params: &[Param { name: "aDefaultProfile", ty: "*mut *const nsIToolkitProfile" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_defaultProfile",
                        abi: "C",
                        params: &[Param { name: "aDefaultProfile", ty: "*const nsIToolkitProfile" }],
                        ret: "nsresult",
                    },

                    /* nsIToolkitProfile getProfileByName (in AUTF8String aName); */
                    Method {
                        name: "getProfileByName",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIToolkitProfile" }],
                        ret: "nsresult",
                    },

                    /* nsIProfileLock lockProfilePath (in nsIFile aDirectory, in nsIFile aTempDirectory); */
                    Method {
                        name: "lockProfilePath",
                        abi: "C",
                        params: &[Param { name: "aDirectory", ty: "*const nsIFile" }, Param { name: "aTempDirectory", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut *const nsIProfileLock" }],
                        ret: "nsresult",
                    },

                    /* nsIToolkitProfile createProfile (in nsIFile aRootDir, in AUTF8String aName); */
                    Method {
                        name: "createProfile",
                        abi: "C",
                        params: &[Param { name: "aRootDir", ty: "*const nsIFile" }, Param { name: "aName", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIToolkitProfile" }],
                        ret: "nsresult",
                    },

                    /* nsIToolkitProfile createDefaultProfileForApp (in AUTF8String aProfileName, in AUTF8String aAppName, in AUTF8String aVendorName); */
                    Method {
                        name: "createDefaultProfileForApp",
                        abi: "C",
                        params: &[Param { name: "aProfileName", ty: "*const nsACString" }, Param { name: "aAppName", ty: "*const nsACString" }, Param { name: "aVendorName", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIToolkitProfile" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long profileCount; */
                    Method {
                        name: "get_profileCount",
                        abi: "C",
                        params: &[Param { name: "aProfileCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void flush (); */
                    Method {
                        name: "flush",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

