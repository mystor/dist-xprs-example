//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISubstitutingProtocolHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISubstitutingProtocolHandler",
            base: Some("nsIProtocolHandler"),
            methods: Some(&[
                    /* [must_use] void setSubstitution (in ACString root, in nsIURI baseURI); */
                    Method {
                        name: "setSubstitution",
                        abi: "C",
                        params: &[Param { name: "root", ty: "*const nsACString" }, Param { name: "baseURI", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* [must_use] nsIURI getSubstitution (in ACString root); */
                    Method {
                        name: "getSubstitution",
                        abi: "C",
                        params: &[Param { name: "root", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* [must_use] boolean hasSubstitution (in ACString root); */
                    Method {
                        name: "hasSubstitution",
                        abi: "C",
                        params: &[Param { name: "root", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] AUTF8String resolveURI (in nsIURI resURI); */
                    Method {
                        name: "resolveURI",
                        abi: "C",
                        params: &[Param { name: "resURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

