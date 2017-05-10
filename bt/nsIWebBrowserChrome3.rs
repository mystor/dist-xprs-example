//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserChrome3.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebBrowserChrome3",
            base: Some("nsIWebBrowserChrome2"),
            methods: Some(&[
                    /* AString onBeforeLinkTraversal (in AString originalTarget, in nsIURI linkURI, in nsIDOMNode linkNode, in boolean isAppTab); */
                    Method {
                        name: "onBeforeLinkTraversal",
                        abi: "C",
                        params: &[Param { name: "originalTarget", ty: "*const nsAString" }, Param { name: "linkURI", ty: "*const nsIURI" }, Param { name: "linkNode", ty: "*const nsIDOMNode" }, Param { name: "isAppTab", ty: "bool" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* bool shouldLoadURI (in nsIDocShell aDocShell, in nsIURI aURI, in nsIURI aReferrer, in boolean aHasPostData, in nsIPrincipal aTriggeringPrincipal); */
                    Method {
                        name: "shouldLoadURI",
                        abi: "C",
                        params: &[Param { name: "aDocShell", ty: "*const nsIDocShell" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aReferrer", ty: "*const nsIURI" }, Param { name: "aHasPostData", ty: "bool" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* bool shouldLoadURIInThisProcess (in nsIURI aURI); */
                    Method {
                        name: "shouldLoadURIInThisProcess",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* bool reloadInFreshProcess (in nsIDocShell aDocShell, in nsIURI aURI, in nsIURI aReferrer, in nsIPrincipal aTriggeringPrincipal, in uint32_t aLoadFlags); */
                    Method {
                        name: "reloadInFreshProcess",
                        abi: "C",
                        params: &[Param { name: "aDocShell", ty: "*const nsIDocShell" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aReferrer", ty: "*const nsIURI" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aLoadFlags", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void startPrerenderingDocument (in nsIURI aHref, in nsIURI aReferrer); */
                    Method {
                        name: "startPrerenderingDocument",
                        abi: "C",
                        params: &[Param { name: "aHref", ty: "*const nsIURI" }, Param { name: "aReferrer", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* bool shouldSwitchToPrerenderedDocument (in nsIURI aHref, in nsIURI aReferrer, in nsIRunnable aSuccess, in nsIRunnable aFailure); */
                    Method {
                        name: "shouldSwitchToPrerenderedDocument",
                        abi: "C",
                        params: &[Param { name: "aHref", ty: "*const nsIURI" }, Param { name: "aReferrer", ty: "*const nsIURI" }, Param { name: "aSuccess", ty: "*const nsIRunnable" }, Param { name: "aFailure", ty: "*const nsIRunnable" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

