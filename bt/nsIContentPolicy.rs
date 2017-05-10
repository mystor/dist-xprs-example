//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentPolicy.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentPolicy",
            base: Some("nsIContentPolicyBase"),
            methods: Some(&[
                    /* short shouldLoad (in nsContentPolicyType aContentType, in nsIURI aContentLocation, in nsIURI aRequestOrigin, in nsISupports aContext, in ACString aMimeTypeGuess, in nsISupports aExtra, [optional] in nsIPrincipal aRequestPrincipal); */
                    Method {
                        name: "shouldLoad",
                        abi: "C",
                        params: &[Param { name: "aContentType", ty: "nsContentPolicyType" }, Param { name: "aContentLocation", ty: "*const nsIURI" }, Param { name: "aRequestOrigin", ty: "*const nsIURI" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aMimeTypeGuess", ty: "*const nsACString" }, Param { name: "aExtra", ty: "*const nsISupports" }, Param { name: "aRequestPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* short shouldProcess (in nsContentPolicyType aContentType, in nsIURI aContentLocation, in nsIURI aRequestOrigin, in nsISupports aContext, in ACString aMimeType, in nsISupports aExtra, [optional] in nsIPrincipal aRequestPrincipal); */
                    Method {
                        name: "shouldProcess",
                        abi: "C",
                        params: &[Param { name: "aContentType", ty: "nsContentPolicyType" }, Param { name: "aContentLocation", ty: "*const nsIURI" }, Param { name: "aRequestOrigin", ty: "*const nsIURI" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aMimeType", ty: "*const nsACString" }, Param { name: "aExtra", ty: "*const nsISupports" }, Param { name: "aRequestPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

