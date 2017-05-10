//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIServiceManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIServiceManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void getService (in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
                    Method {
                        name: "getService",
                        abi: "C",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* void getServiceByContractID (in string aContractID, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
                    Method {
                        name: "getServiceByContractID",
                        abi: "C",
                        params: &[Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* boolean isServiceInstantiated (in nsCIDRef aClass, in nsIIDRef aIID); */
                    Method {
                        name: "isServiceInstantiated",
                        abi: "C",
                        params: &[Param { name: "aClass", ty: "*const nsCID" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isServiceInstantiatedByContractID (in string aContractID, in nsIIDRef aIID); */
                    Method {
                        name: "isServiceInstantiatedByContractID",
                        abi: "C",
                        params: &[Param { name: "aContractID", ty: "*const libc::c_char" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

