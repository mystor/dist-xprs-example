//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIModule.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIModule",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void getClassObject (in nsIComponentManager aCompMgr, in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult aResult); */
                    Method {
                        name: "getClassObject",
                        abi: "C",
                        params: &[Param { name: "aCompMgr", ty: "*const nsIComponentManager" }, Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "aResult", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* void registerSelf (in nsIComponentManager aCompMgr, in nsIFile aLocation, in string aLoaderStr, in string aType); */
                    Method {
                        name: "registerSelf",
                        abi: "C",
                        params: &[Param { name: "aCompMgr", ty: "*const nsIComponentManager" }, Param { name: "aLocation", ty: "*const nsIFile" }, Param { name: "aLoaderStr", ty: "*const libc::c_char" }, Param { name: "aType", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void unregisterSelf (in nsIComponentManager aCompMgr, in nsIFile aLocation, in string aLoaderStr); */
                    Method {
                        name: "unregisterSelf",
                        abi: "C",
                        params: &[Param { name: "aCompMgr", ty: "*const nsIComponentManager" }, Param { name: "aLocation", ty: "*const nsIFile" }, Param { name: "aLoaderStr", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* boolean canUnload (in nsIComponentManager aCompMgr); */
                    Method {
                        name: "canUnload",
                        abi: "C",
                        params: &[Param { name: "aCompMgr", ty: "*const nsIComponentManager" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

