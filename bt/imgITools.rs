//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgITools.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "imgITools",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* imgIContainer decodeImage (in nsIInputStream aStream, in ACString aMimeType); */
                    Method {
                        name: "decodeImage",
                        abi: "C",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aMimeType", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const imgIContainer" }],
                        ret: "nsresult",
                    },

                    /* [deprecated] void decodeImageData (in nsIInputStream aStream, in ACString aMimeType, inout imgIContainer aContainer); */
                    Method {
                        name: "decodeImageData",
                        abi: "C",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aMimeType", ty: "*const nsACString" }, Param { name: "aContainer", ty: "*mut *const imgIContainer" }],
                        ret: "nsresult",
                    },

                    /* nsIInputStream encodeImage (in imgIContainer aContainer, in ACString aMimeType, [optional] in AString outputOptions); */
                    Method {
                        name: "encodeImage",
                        abi: "C",
                        params: &[Param { name: "aContainer", ty: "*const imgIContainer" }, Param { name: "aMimeType", ty: "*const nsACString" }, Param { name: "outputOptions", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* nsIInputStream encodeScaledImage (in imgIContainer aContainer, in ACString aMimeType, in long aWidth, in long aHeight, [optional] in AString outputOptions); */
                    Method {
                        name: "encodeScaledImage",
                        abi: "C",
                        params: &[Param { name: "aContainer", ty: "*const imgIContainer" }, Param { name: "aMimeType", ty: "*const nsACString" }, Param { name: "aWidth", ty: "libc::int32_t" }, Param { name: "aHeight", ty: "libc::int32_t" }, Param { name: "outputOptions", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* imgILoader getImgLoaderForDocument (in nsIDOMDocument doc); */
                    Method {
                        name: "getImgLoaderForDocument",
                        abi: "C",
                        params: &[Param { name: "doc", ty: "*const nsIDOMDocument" }, Param { name: "_retval", ty: "*mut *const imgILoader" }],
                        ret: "nsresult",
                    },

                    /* imgICache getImgCacheForDocument (in nsIDOMDocument doc); */
                    Method {
                        name: "getImgCacheForDocument",
                        abi: "C",
                        params: &[Param { name: "doc", ty: "*const nsIDOMDocument" }, Param { name: "_retval", ty: "*mut *const imgICache" }],
                        ret: "nsresult",
                    },

                    /* nsIInputStream encodeCroppedImage (in imgIContainer aContainer, in ACString aMimeType, in long aOffsetX, in long aOffsetY, in long aWidth, in long aHeight, [optional] in AString outputOptions); */
                    Method {
                        name: "encodeCroppedImage",
                        abi: "C",
                        params: &[Param { name: "aContainer", ty: "*const imgIContainer" }, Param { name: "aMimeType", ty: "*const nsACString" }, Param { name: "aOffsetX", ty: "libc::int32_t" }, Param { name: "aOffsetY", ty: "libc::int32_t" }, Param { name: "aWidth", ty: "libc::int32_t" }, Param { name: "aHeight", ty: "libc::int32_t" }, Param { name: "outputOptions", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* imgINotificationObserver createScriptedObserver (in imgIScriptedNotificationObserver aObserver); */
                    Method {
                        name: "createScriptedObserver",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const imgIScriptedNotificationObserver" }, Param { name: "_retval", ty: "*mut *const imgINotificationObserver" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

