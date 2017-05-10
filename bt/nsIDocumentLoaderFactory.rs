//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocumentLoaderFactory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDocumentLoaderFactory",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIContentViewer createInstance (in string aCommand, in nsIChannel aChannel, in nsILoadGroup aLoadGroup, in ACString aContentType, in nsIDocShell aContainer, in nsISupports aExtraInfo, out nsIStreamListener aDocListenerResult); */
                    Method {
                        name: "createInstance",
                        abi: "C",
                        params: &[Param { name: "aCommand", ty: "*const libc::c_char" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aLoadGroup", ty: "*const nsILoadGroup" }, Param { name: "aContentType", ty: "*const nsACString" }, Param { name: "aContainer", ty: "*const nsIDocShell" }, Param { name: "aExtraInfo", ty: "*const nsISupports" }, Param { name: "aDocListenerResult", ty: "*mut *const nsIStreamListener" }, Param { name: "_retval", ty: "*mut *const nsIContentViewer" }],
                        ret: "nsresult",
                    },

                    /* nsIContentViewer createInstanceForDocument (in nsISupports aContainer, in nsIDocument aDocument, in string aCommand); */
                    Method {
                        name: "createInstanceForDocument",
                        abi: "C",
                        params: &[Param { name: "aContainer", ty: "*const nsISupports" }, Param { name: "aDocument", ty: "*const nsIDocument" }, Param { name: "aCommand", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsIContentViewer" }],
                        ret: "nsresult",
                    },

                    /* nsIDocument createBlankDocument (in nsILoadGroup aLoadGroup, in nsIPrincipal aPrincipal); */
                    Method {
                        name: "createBlankDocument",
                        abi: "C",
                        params: &[Param { name: "aLoadGroup", ty: "*const nsILoadGroup" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut *const nsIDocument" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

