//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHandlerService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHandlerService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsISimpleEnumerator enumerate (); */
                    Method {
                        name: "enumerate",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* void fillHandlerInfo (in nsIHandlerInfo aHandlerInfo, in ACString aOverrideType); */
                    Method {
                        name: "fillHandlerInfo",
                        abi: "C",
                        params: &[Param { name: "aHandlerInfo", ty: "*const nsIHandlerInfo" }, Param { name: "aOverrideType", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void store (in nsIHandlerInfo aHandlerInfo); */
                    Method {
                        name: "store",
                        abi: "C",
                        params: &[Param { name: "aHandlerInfo", ty: "*const nsIHandlerInfo" }],
                        ret: "nsresult",
                    },

                    /* boolean exists (in nsIHandlerInfo aHandlerInfo); */
                    Method {
                        name: "exists",
                        abi: "C",
                        params: &[Param { name: "aHandlerInfo", ty: "*const nsIHandlerInfo" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void remove (in nsIHandlerInfo aHandlerInfo); */
                    Method {
                        name: "remove",
                        abi: "C",
                        params: &[Param { name: "aHandlerInfo", ty: "*const nsIHandlerInfo" }],
                        ret: "nsresult",
                    },

                    /* ACString getTypeFromExtension (in ACString aFileExtension); */
                    Method {
                        name: "getTypeFromExtension",
                        abi: "C",
                        params: &[Param { name: "aFileExtension", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

