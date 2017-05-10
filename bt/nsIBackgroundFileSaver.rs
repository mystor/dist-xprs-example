//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBackgroundFileSaver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIBackgroundFileSaver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsIBackgroundFileSaverObserver observer; */
                    Method {
                        name: "get_observer",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*mut *const nsIBackgroundFileSaverObserver" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_observer",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIBackgroundFileSaverObserver" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIArray signatureInfo; */
                    Method {
                        name: "get_signatureInfo",
                        abi: "C",
                        params: &[Param { name: "aSignatureInfo", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString sha256Hash; */
                    Method {
                        name: "get_sha256Hash",
                        abi: "C",
                        params: &[Param { name: "aSha256Hash", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* void enableSignatureInfo (); */
                    Method {
                        name: "enableSignatureInfo",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void enableSha256 (); */
                    Method {
                        name: "enableSha256",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void enableAppend (); */
                    Method {
                        name: "enableAppend",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void setTarget (in nsIFile aTarget, in bool aKeepPartial); */
                    Method {
                        name: "setTarget",
                        abi: "C",
                        params: &[Param { name: "aTarget", ty: "*const nsIFile" }, Param { name: "aKeepPartial", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void finish (in nsresult aStatus); */
                    Method {
                        name: "finish",
                        abi: "C",
                        params: &[Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIBackgroundFileSaverObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onTargetChange (in nsIBackgroundFileSaver aSaver, in nsIFile aTarget); */
                    Method {
                        name: "onTargetChange",
                        abi: "C",
                        params: &[Param { name: "aSaver", ty: "*const nsIBackgroundFileSaver" }, Param { name: "aTarget", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void onSaveComplete (in nsIBackgroundFileSaver aSaver, in nsresult aStatus); */
                    Method {
                        name: "onSaveComplete",
                        abi: "C",
                        params: &[Param { name: "aSaver", ty: "*const nsIBackgroundFileSaver" }, Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

