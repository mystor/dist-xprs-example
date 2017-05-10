//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMStorageManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMStorageManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIDOMStorage precacheStorage (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "precacheStorage",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut *const nsIDOMStorage" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMStorage createStorage (in mozIDOMWindow aWindow, in nsIPrincipal aPrincipal, in DOMString aDocumentURI, [optional] in bool aPrivate); */
                    Method {
                        name: "createStorage",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindow" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aDocumentURI", ty: "*const nsAString" }, Param { name: "aPrivate", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIDOMStorage" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMStorage getStorage (in mozIDOMWindow aWindow, in nsIPrincipal aPrincipal, [optional] in bool aPrivate); */
                    Method {
                        name: "getStorage",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindow" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aPrivate", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIDOMStorage" }],
                        ret: "nsresult",
                    },

                    /* void cloneStorage (in nsIDOMStorage aStorageToCloneFrom); */
                    Method {
                        name: "cloneStorage",
                        abi: "C",
                        params: &[Param { name: "aStorageToCloneFrom", ty: "*const nsIDOMStorage" }],
                        ret: "nsresult",
                    },

                    /* bool checkStorage (in nsIPrincipal aPrincipal, in nsIDOMStorage aStorage); */
                    Method {
                        name: "checkStorage",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aStorage", ty: "*const nsIDOMStorage" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMStorage getLocalStorageForPrincipal (in nsIPrincipal aPrincipal, in DOMString aDocumentURI, [optional] in bool aPrivate); */
                    Method {
                        name: "getLocalStorageForPrincipal",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aDocumentURI", ty: "*const nsAString" }, Param { name: "aPrivate", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIDOMStorage" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

