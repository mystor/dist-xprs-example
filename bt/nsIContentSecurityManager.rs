//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentSecurityManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentSecurityManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIStreamListener performSecurityCheck (in nsIChannel aChannel, in nsIStreamListener aStreamListener); */
                    Method {
                        name: "performSecurityCheck",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aStreamListener", ty: "*const nsIStreamListener" }, Param { name: "_retval", ty: "*mut *const nsIStreamListener" }],
                        ret: "nsresult",
                    },

                    /* boolean isOriginPotentiallyTrustworthy (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "isOriginPotentiallyTrustworthy",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

