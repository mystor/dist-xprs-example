//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULBrowserWindow.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXULBrowserWindow",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setJSStatus (in AString status); */
                    Method {
                        name: "setJSStatus",
                        abi: "C",
                        params: &[Param { name: "status", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setOverLink (in AString link, in nsIDOMElement element); */
                    Method {
                        name: "setOverLink",
                        abi: "C",
                        params: &[Param { name: "link", ty: "*const nsAString" }, Param { name: "element", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* AString onBeforeLinkTraversal (in AString originalTarget, in nsIURI linkURI, in nsIDOMNode linkNode, in boolean isAppTab); */
                    Method {
                        name: "onBeforeLinkTraversal",
                        abi: "C",
                        params: &[Param { name: "originalTarget", ty: "*const nsAString" }, Param { name: "linkURI", ty: "*const nsIURI" }, Param { name: "linkNode", ty: "*const nsIDOMNode" }, Param { name: "isAppTab", ty: "bool" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void forceInitialBrowserRemote (in AString aRemoteType); */
                    Method {
                        name: "forceInitialBrowserRemote",
                        abi: "C",
                        params: &[Param { name: "aRemoteType", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void forceInitialBrowserNonRemote (in mozIDOMWindowProxy openerWindow); */
                    Method {
                        name: "forceInitialBrowserNonRemote",
                        abi: "C",
                        params: &[Param { name: "openerWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* bool shouldLoadURI (in nsIDocShell aDocShell, in nsIURI aURI, in nsIURI aReferrer, in boolean aHasPostData, in nsIPrincipal aTriggeringPrincipal); */
                    Method {
                        name: "shouldLoadURI",
                        abi: "C",
                        params: &[Param { name: "aDocShell", ty: "*const nsIDocShell" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aReferrer", ty: "*const nsIURI" }, Param { name: "aHasPostData", ty: "bool" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void showTooltip (in long x, in long y, in AString tooltip, in AString direction); */
                    Method {
                        name: "showTooltip",
                        abi: "C",
                        params: &[Param { name: "x", ty: "libc::int32_t" }, Param { name: "y", ty: "libc::int32_t" }, Param { name: "tooltip", ty: "*const nsAString" }, Param { name: "direction", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void hideTooltip (); */
                    Method {
                        name: "hideTooltip",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* uint32_t getTabCount (); */
                    Method {
                        name: "getTabCount",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void navigateAndRestoreByIndex (in nsIBrowser aBrowser, in long aIndex); */
                    Method {
                        name: "navigateAndRestoreByIndex",
                        abi: "C",
                        params: &[Param { name: "aBrowser", ty: "*const nsIBrowser" }, Param { name: "aIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

