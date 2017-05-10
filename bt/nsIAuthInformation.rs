//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAuthInformation.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAuthInformation",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long flags; */
                    Method {
                        name: "get_flags",
                        abi: "C",
                        params: &[Param { name: "aFlags", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString realm; */
                    Method {
                        name: "get_realm",
                        abi: "C",
                        params: &[Param { name: "aRealm", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String authenticationScheme; */
                    Method {
                        name: "get_authenticationScheme",
                        abi: "C",
                        params: &[Param { name: "aAuthenticationScheme", ty: "*mut nsACString" }],
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

                    /* attribute AString domain; */
                    Method {
                        name: "get_domain",
                        abi: "C",
                        params: &[Param { name: "aDomain", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_domain",
                        abi: "C",
                        params: &[Param { name: "aDomain", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

