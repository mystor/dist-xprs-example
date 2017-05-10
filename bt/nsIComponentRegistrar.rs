//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIComponentRegistrar.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIComponentRegistrar",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void autoRegister (in nsIFile aSpec); */
                    Method {
                        name: "autoRegister",
                        abi: "C",
                        params: &[Param { name: "aSpec", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void autoUnregister (in nsIFile aSpec); */
                    Method {
                        name: "autoUnregister",
                        abi: "C",
                        params: &[Param { name: "aSpec", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void registerFactory (in nsCIDRef aClass, in string aClassName, in string aContractID, in nsIFactory aFactory); */
                    Method {
                        name: "registerFactory",
                        abi: "C",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aClassName", ty: "*const libc::c_char" }, Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "aFactory", ty: "*const nsIFactory" }],
                        ret: "nsresult",
                    },

                    /* void unregisterFactory (in nsCIDRef aClass, in nsIFactory aFactory); */
                    Method {
                        name: "unregisterFactory",
                        abi: "C",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aFactory", ty: "*const nsIFactory" }],
                        ret: "nsresult",
                    },

                    /* void registerFactoryLocation (in nsCIDRef aClass, in string aClassName, in string aContractID, in nsIFile aFile, in string aLoaderStr, in string aType); */
                    Method {
                        name: "registerFactoryLocation",
                        abi: "C",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aClassName", ty: "*const libc::c_char" }, Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "aLoaderStr", ty: "*const libc::c_char" }, Param { name: "aType", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void unregisterFactoryLocation (in nsCIDRef aClass, in nsIFile aFile); */
                    Method {
                        name: "unregisterFactoryLocation",
                        abi: "C",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aFile", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* boolean isCIDRegistered (in nsCIDRef aClass); */
                    Method {
                        name: "isCIDRegistered",
                        abi: "C",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isContractIDRegistered (in string aContractID); */
                    Method {
                        name: "isContractIDRegistered",
                        abi: "C",
                        params: &[Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator enumerateCIDs (); */
                    Method {
                        name: "enumerateCIDs",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator enumerateContractIDs (); */
                    Method {
                        name: "enumerateContractIDs",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* string CIDToContractID (in nsCIDRef aClass); */
                    Method {
                        name: "CIDToContractID",
                        abi: "C",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* nsCIDPtr contractIDToCID (in string aContractID); */
                    Method {
                        name: "contractIDToCID",
                        abi: "C",
                        params: &[Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsCID" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

