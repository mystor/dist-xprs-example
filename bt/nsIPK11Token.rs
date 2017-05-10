//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPK11Token.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPK11Token",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AUTF8String tokenName; */
                    Method {
                        name: "get_tokenName",
                        abi: "C",
                        params: &[Param { name: "aTokenName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String tokenLabel; */
                    Method {
                        name: "get_tokenLabel",
                        abi: "C",
                        params: &[Param { name: "aTokenLabel", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String tokenManID; */
                    Method {
                        name: "get_tokenManID",
                        abi: "C",
                        params: &[Param { name: "aTokenManID", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String tokenHWVersion; */
                    Method {
                        name: "get_tokenHWVersion",
                        abi: "C",
                        params: &[Param { name: "aTokenHWVersion", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String tokenFWVersion; */
                    Method {
                        name: "get_tokenFWVersion",
                        abi: "C",
                        params: &[Param { name: "aTokenFWVersion", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String tokenSerialNumber; */
                    Method {
                        name: "get_tokenSerialNumber",
                        abi: "C",
                        params: &[Param { name: "aTokenSerialNumber", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean isLoggedIn (); */
                    Method {
                        name: "isLoggedIn",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void login (in boolean force); */
                    Method {
                        name: "login",
                        abi: "C",
                        params: &[Param { name: "force", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void logoutSimple (); */
                    Method {
                        name: "logoutSimple",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void logoutAndDropAuthenticatedResources (); */
                    Method {
                        name: "logoutAndDropAuthenticatedResources",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void reset (); */
                    Method {
                        name: "reset",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute long minimumPasswordLength; */
                    Method {
                        name: "get_minimumPasswordLength",
                        abi: "C",
                        params: &[Param { name: "aMinimumPasswordLength", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean needsUserInit; */
                    Method {
                        name: "get_needsUserInit",
                        abi: "C",
                        params: &[Param { name: "aNeedsUserInit", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean checkPassword (in AUTF8String password); */
                    Method {
                        name: "checkPassword",
                        abi: "C",
                        params: &[Param { name: "password", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void initPassword (in AUTF8String initialPassword); */
                    Method {
                        name: "initPassword",
                        abi: "C",
                        params: &[Param { name: "initialPassword", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void changePassword (in AUTF8String oldPassword, in AUTF8String newPassword); */
                    Method {
                        name: "changePassword",
                        abi: "C",
                        params: &[Param { name: "oldPassword", ty: "*const nsACString" }, Param { name: "newPassword", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* long getAskPasswordTimes (); */
                    Method {
                        name: "getAskPasswordTimes",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* long getAskPasswordTimeout (); */
                    Method {
                        name: "getAskPasswordTimeout",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setAskPasswordDefaults ([const] in long askTimes, [const] in long timeout); */
                    Method {
                        name: "setAskPasswordDefaults",
                        abi: "C",
                        params: &[Param { name: "askTimes", ty: "libc::int32_t" }, Param { name: "timeout", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean hasPassword; */
                    Method {
                        name: "get_hasPassword",
                        abi: "C",
                        params: &[Param { name: "aHasPassword", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isHardwareToken (); */
                    Method {
                        name: "isHardwareToken",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean needsLogin (); */
                    Method {
                        name: "needsLogin",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

