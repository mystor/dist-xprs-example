//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICertificateDialogs.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICertificateDialogs",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean confirmDownloadCACert (in nsIInterfaceRequestor ctx, in nsIX509Cert cert, out unsigned long trust); */
                    Method {
                        name: "confirmDownloadCACert",
                        abi: "C",
                        params: &[Param { name: "ctx", ty: "*const nsIInterfaceRequestor" }, Param { name: "cert", ty: "*const nsIX509Cert" }, Param { name: "trust", ty: "*mut libc::uint32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean setPKCS12FilePassword (in nsIInterfaceRequestor ctx, out AString password); */
                    Method {
                        name: "setPKCS12FilePassword",
                        abi: "C",
                        params: &[Param { name: "ctx", ty: "*const nsIInterfaceRequestor" }, Param { name: "password", ty: "*mut nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean getPKCS12FilePassword (in nsIInterfaceRequestor ctx, out AString password); */
                    Method {
                        name: "getPKCS12FilePassword",
                        abi: "C",
                        params: &[Param { name: "ctx", ty: "*const nsIInterfaceRequestor" }, Param { name: "password", ty: "*mut nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void viewCert (in nsIInterfaceRequestor ctx, in nsIX509Cert cert); */
                    Method {
                        name: "viewCert",
                        abi: "C",
                        params: &[Param { name: "ctx", ty: "*const nsIInterfaceRequestor" }, Param { name: "cert", ty: "*const nsIX509Cert" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

