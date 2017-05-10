//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClientAuthDialogs.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClientAuthDialogs",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean chooseCertificate (in nsIInterfaceRequestor ctx, in AUTF8String hostname, in long port, in AUTF8String organization, in AUTF8String issuerOrg, in nsIArray certList, out unsigned long selectedIndex); */
                    Method {
                        name: "chooseCertificate",
                        abi: "C",
                        params: &[Param { name: "ctx", ty: "*const nsIInterfaceRequestor" }, Param { name: "hostname", ty: "*const nsACString" }, Param { name: "port", ty: "libc::int32_t" }, Param { name: "organization", ty: "*const nsACString" }, Param { name: "issuerOrg", ty: "*const nsACString" }, Param { name: "certList", ty: "*const nsIArray" }, Param { name: "selectedIndex", ty: "*mut libc::uint32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIClientAuthUserDecision",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute boolean rememberClientAuthCertificate; */
                    Method {
                        name: "get_rememberClientAuthCertificate",
                        abi: "C",
                        params: &[Param { name: "aRememberClientAuthCertificate", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_rememberClientAuthCertificate",
                        abi: "C",
                        params: &[Param { name: "aRememberClientAuthCertificate", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

