//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpAuthenticator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpAuthenticator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [must_use] void challengeReceived (in nsIHttpAuthenticableChannel aChannel, in string aChallenge, in boolean aProxyAuth, inout nsISupports aSessionState, inout nsISupports aContinuationState, out boolean aInvalidatesIdentity); */
                    Method {
                        name: "challengeReceived",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsIHttpAuthenticableChannel" }, Param { name: "aChallenge", ty: "*const libc::c_char" }, Param { name: "aProxyAuth", ty: "bool" }, Param { name: "aSessionState", ty: "*mut *const nsISupports" }, Param { name: "aContinuationState", ty: "*mut *const nsISupports" }, Param { name: "aInvalidatesIdentity", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void generateCredentialsAsync (in nsIHttpAuthenticableChannel aChannel, in nsIHttpAuthenticatorCallback aCallback, in string aChallenge, in boolean aProxyAuth, in wstring aDomain, in wstring aUser, in wstring aPassword, in nsISupports aSessionState, in nsISupports aContinuationState, out nsICancelable aCancel); */
                    Method {
                        name: "generateCredentialsAsync",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsIHttpAuthenticableChannel" }, Param { name: "aCallback", ty: "*const nsIHttpAuthenticatorCallback" }, Param { name: "aChallenge", ty: "*const libc::c_char" }, Param { name: "aProxyAuth", ty: "bool" }, Param { name: "aDomain", ty: "*const libc::int16_t" }, Param { name: "aUser", ty: "*const libc::int16_t" }, Param { name: "aPassword", ty: "*const libc::int16_t" }, Param { name: "aSessionState", ty: "*const nsISupports" }, Param { name: "aContinuationState", ty: "*const nsISupports" }, Param { name: "aCancel", ty: "*mut *const nsICancelable" }],
                        ret: "nsresult",
                    },

                    /* [must_use] string generateCredentials (in nsIHttpAuthenticableChannel aChannel, in string aChallenge, in boolean aProxyAuth, in wstring aDomain, in wstring aUser, in wstring aPassword, inout nsISupports aSessionState, inout nsISupports aContinuationState, out unsigned long aFlags); */
                    Method {
                        name: "generateCredentials",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsIHttpAuthenticableChannel" }, Param { name: "aChallenge", ty: "*const libc::c_char" }, Param { name: "aProxyAuth", ty: "bool" }, Param { name: "aDomain", ty: "*const libc::int16_t" }, Param { name: "aUser", ty: "*const libc::int16_t" }, Param { name: "aPassword", ty: "*const libc::int16_t" }, Param { name: "aSessionState", ty: "*mut *const nsISupports" }, Param { name: "aContinuationState", ty: "*mut *const nsISupports" }, Param { name: "aFlags", ty: "*mut libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute unsigned long authFlags; */
                    Method {
                        name: "get_authFlags",
                        abi: "C",
                        params: &[Param { name: "aAuthFlags", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

