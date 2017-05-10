//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISecurityReporter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISecurityReporter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void reportTLSError (in nsITransportSecurityInfo aSecurityInfo, in AUTF8String aHostname, in long aPort); */
                    Method {
                        name: "reportTLSError",
                        abi: "C",
                        params: &[Param { name: "aSecurityInfo", ty: "*const nsITransportSecurityInfo" }, Param { name: "aHostname", ty: "*const nsACString" }, Param { name: "aPort", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

