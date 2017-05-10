//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIDownloadPlatform.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIDownloadPlatform",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void downloadDone (in nsIURI aSource, in nsIURI aReferrer, in nsIFile aTarget, in ACString aContentType, in boolean aIsPrivate); */
                    Method {
                        name: "downloadDone",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*const nsIURI" }, Param { name: "aReferrer", ty: "*const nsIURI" }, Param { name: "aTarget", ty: "*const nsIFile" }, Param { name: "aContentType", ty: "*const nsACString" }, Param { name: "aIsPrivate", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* unsigned long mapUrlToZone (in AString aURL); */
                    Method {
                        name: "mapUrlToZone",
                        abi: "C",
                        params: &[Param { name: "aURL", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

