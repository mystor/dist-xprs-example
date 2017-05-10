//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIInterfaceInfoManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInterfaceInfoManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIInterfaceInfo getInfoForIID (in nsIIDPtr iid); */
                    Method {
                        name: "getInfoForIID",
                        abi: "C",
                        params: &[Param { name: "iid", ty: "*const nsIID" }, Param { name: "_retval", ty: "*mut *const nsIInterfaceInfo" }],
                        ret: "nsresult",
                    },

                    /* nsIInterfaceInfo getInfoForName (in string name); */
                    Method {
                        name: "getInfoForName",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsIInterfaceInfo" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

