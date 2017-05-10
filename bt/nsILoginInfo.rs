//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoginInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILoginInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute AString hostname; */
                    Method {
                        name: "get_hostname",
                        abi: "C",
                        params: &[Param { name: "aHostname", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_hostname",
                        abi: "C",
                        params: &[Param { name: "aHostname", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString formSubmitURL; */
                    Method {
                        name: "get_formSubmitURL",
                        abi: "C",
                        params: &[Param { name: "aFormSubmitURL", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_formSubmitURL",
                        abi: "C",
                        params: &[Param { name: "aFormSubmitURL", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString httpRealm; */
                    Method {
                        name: "get_httpRealm",
                        abi: "C",
                        params: &[Param { name: "aHttpRealm", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_httpRealm",
                        abi: "C",
                        params: &[Param { name: "aHttpRealm", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString username; */
                    Method {
                        name: "get_username",
                        abi: "C",
                        params: &[Param { name: "aUsername", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_username",
                        abi: "C",
                        params: &[Param { name: "aUsername", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString usernameField; */
                    Method {
                        name: "get_usernameField",
                        abi: "C",
                        params: &[Param { name: "aUsernameField", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_usernameField",
                        abi: "C",
                        params: &[Param { name: "aUsernameField", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString password; */
                    Method {
                        name: "get_password",
                        abi: "C",
                        params: &[Param { name: "aPassword", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_password",
                        abi: "C",
                        params: &[Param { name: "aPassword", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString passwordField; */
                    Method {
                        name: "get_passwordField",
                        abi: "C",
                        params: &[Param { name: "aPasswordField", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_passwordField",
                        abi: "C",
                        params: &[Param { name: "aPasswordField", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void init (in AString aHostname, in AString aFormSubmitURL, in AString aHttpRealm, in AString aUsername, in AString aPassword, in AString aUsernameField, in AString aPasswordField); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aHostname", ty: "*const nsAString" }, Param { name: "aFormSubmitURL", ty: "*const nsAString" }, Param { name: "aHttpRealm", ty: "*const nsAString" }, Param { name: "aUsername", ty: "*const nsAString" }, Param { name: "aPassword", ty: "*const nsAString" }, Param { name: "aUsernameField", ty: "*const nsAString" }, Param { name: "aPasswordField", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* boolean equals (in nsILoginInfo aLoginInfo); */
                    Method {
                        name: "equals",
                        abi: "C",
                        params: &[Param { name: "aLoginInfo", ty: "*const nsILoginInfo" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean matches (in nsILoginInfo aLoginInfo, in boolean ignorePassword); */
                    Method {
                        name: "matches",
                        abi: "C",
                        params: &[Param { name: "aLoginInfo", ty: "*const nsILoginInfo" }, Param { name: "ignorePassword", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsILoginInfo clone (); */
                    Method {
                        name: "clone",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsILoginInfo" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

