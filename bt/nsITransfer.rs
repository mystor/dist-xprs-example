//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransfer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITransfer",
            base: Some("nsIWebProgressListener2"),
            methods: Some(&[
                    /* void init (in nsIURI aSource, in nsIURI aTarget, in AString aDisplayName, in nsIMIMEInfo aMIMEInfo, in PRTime startTime, in nsIFile aTempFile, in nsICancelable aCancelable, in boolean aIsPrivate); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*const nsIURI" }, Param { name: "aTarget", ty: "*const nsIURI" }, Param { name: "aDisplayName", ty: "*const nsAString" }, Param { name: "aMIMEInfo", ty: "*const nsIMIMEInfo" }, Param { name: "startTime", ty: "PRTime" }, Param { name: "aTempFile", ty: "*const nsIFile" }, Param { name: "aCancelable", ty: "*const nsICancelable" }, Param { name: "aIsPrivate", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void setSha256Hash (in ACString aHash); */
                    Method {
                        name: "setSha256Hash",
                        abi: "C",
                        params: &[Param { name: "aHash", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void setSignatureInfo (in nsIArray aSignatureInfo); */
                    Method {
                        name: "setSignatureInfo",
                        abi: "C",
                        params: &[Param { name: "aSignatureInfo", ty: "*const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* void setRedirects (in nsIArray aRedirects); */
                    Method {
                        name: "setRedirects",
                        abi: "C",
                        params: &[Param { name: "aRedirects", ty: "*const nsIArray" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

