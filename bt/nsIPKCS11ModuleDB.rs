//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPKCS11ModuleDB.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPKCS11ModuleDB",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIPKCS11Module getInternal (); */
                    Method {
                        name: "getInternal",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIPKCS11Module" }],
                        ret: "nsresult",
                    },

                    /* nsIPKCS11Module getInternalFIPS (); */
                    Method {
                        name: "getInternalFIPS",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIPKCS11Module" }],
                        ret: "nsresult",
                    },

                    /* nsIPKCS11Module findModuleByName (in AUTF8String name); */
                    Method {
                        name: "findModuleByName",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIPKCS11Module" }],
                        ret: "nsresult",
                    },

                    /* nsIPKCS11Slot findSlotByName (in AUTF8String name); */
                    Method {
                        name: "findSlotByName",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIPKCS11Slot" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator listModules (); */
                    Method {
                        name: "listModules",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean canToggleFIPS; */
                    Method {
                        name: "get_canToggleFIPS",
                        abi: "C",
                        params: &[Param { name: "aCanToggleFIPS", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void toggleFIPSMode (); */
                    Method {
                        name: "toggleFIPSMode",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isFIPSEnabled; */
                    Method {
                        name: "get_isFIPSEnabled",
                        abi: "C",
                        params: &[Param { name: "aIsFIPSEnabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

