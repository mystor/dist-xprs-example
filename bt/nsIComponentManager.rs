//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIComponentManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIComponentManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void getClassObject (in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
                    Method {
                        name: "getClassObject",
                        abi: "C",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* void getClassObjectByContractID (in string aContractID, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
                    Method {
                        name: "getClassObjectByContractID",
                        abi: "C",
                        params: &[Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* void createInstance (in nsCIDRef aClass, in nsISupports aDelegate, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
                    Method {
                        name: "createInstance",
                        abi: "C",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aDelegate", ty: "*const nsISupports" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* void createInstanceByContractID (in string aContractID, in nsISupports aDelegate, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
                    Method {
                        name: "createInstanceByContractID",
                        abi: "C",
                        params: &[Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "aDelegate", ty: "*const nsISupports" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* void addBootstrappedManifestLocation (in nsIFile aLocation); */
                    Method {
                        name: "addBootstrappedManifestLocation",
                        abi: "C",
                        params: &[Param { name: "aLocation", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void removeBootstrappedManifestLocation (in nsIFile aLocation); */
                    Method {
                        name: "removeBootstrappedManifestLocation",
                        abi: "C",
                        params: &[Param { name: "aLocation", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* nsIArray getManifestLocations (); */
                    Method {
                        name: "getManifestLocations",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

