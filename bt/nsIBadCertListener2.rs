//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBadCertListener2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIBadCertListener2",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean notifyCertProblem (in nsIInterfaceRequestor socketInfo, in nsISSLStatus status, in AUTF8String targetSite); */
                    Method {
                        name: "notifyCertProblem",
                        abi: "C",
                        params: &[Param { name: "socketInfo", ty: "*const nsIInterfaceRequestor" }, Param { name: "status", ty: "*const nsISSLStatus" }, Param { name: "targetSite", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

