//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgIRequest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "imgIRequest",
            base: Some("nsIRequest"),
            methods: Some(&[
                    /* readonly attribute imgIContainer image; */
                    Method {
                        name: "get_image",
                        abi: "C",
                        params: &[Param { name: "aImage", ty: "*mut *const imgIContainer" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long imageStatus; */
                    Method {
                        name: "get_imageStatus",
                        abi: "C",
                        params: &[Param { name: "aImageStatus", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript] readonly attribute nsresult imageErrorCode; */
                    Method {
                        name: "get_imageErrorCode",
                        abi: "C",
                        params: &[Param { name: "aImageErrorCode", ty: "*mut nsresult" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI URI; */
                    Method {
                        name: "get_URI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI currentURI; */
                    Method {
                        name: "get_currentURI",
                        abi: "C",
                        params: &[Param { name: "aCurrentURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute imgINotificationObserver notificationObserver; */
                    Method {
                        name: "get_notificationObserver",
                        abi: "C",
                        params: &[Param { name: "aNotificationObserver", ty: "*mut *const imgINotificationObserver" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute string mimeType; */
                    Method {
                        name: "get_mimeType",
                        abi: "C",
                        params: &[Param { name: "aMimeType", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* imgIRequest clone (in imgINotificationObserver aObserver); */
                    Method {
                        name: "clone",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const imgINotificationObserver" }, Param { name: "_retval", ty: "*mut *const imgIRequest" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIPrincipal imagePrincipal; */
                    Method {
                        name: "get_imagePrincipal",
                        abi: "C",
                        params: &[Param { name: "aImagePrincipal", ty: "*mut *const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute bool multipart; */
                    Method {
                        name: "get_multipart",
                        abi: "C",
                        params: &[Param { name: "aMultipart", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long CORSMode; */
                    Method {
                        name: "get_CORSMode",
                        abi: "C",
                        params: &[Param { name: "aCORSMode", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void cancelAndForgetObserver (in nsresult aStatus); */
                    Method {
                        name: "cancelAndForgetObserver",
                        abi: "C",
                        params: &[Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void startDecoding (in uint32_t aFlags); */
                    Method {
                        name: "startDecoding",
                        abi: "C",
                        params: &[Param { name: "aFlags", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript,notxpcom] boolean startDecodingWithResult (in uint32_t aFlags); */
                    Method {
                        name: "startDecodingWithResult",
                        abi: "C",
                        params: &[Param { name: "aFlags", ty: "uint32_t" }],
                        ret: "bool",
                    },

                    /* void lockImage (); */
                    Method {
                        name: "lockImage",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void unlockImage (); */
                    Method {
                        name: "unlockImage",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void requestDiscard (); */
                    Method {
                        name: "requestDiscard",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* imgIRequest getStaticRequest (); */
                    Method {
                        name: "getStaticRequest",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const imgIRequest" }],
                        ret: "nsresult",
                    },

                    /* void incrementAnimationConsumers (); */
                    Method {
                        name: "incrementAnimationConsumers",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void decrementAnimationConsumers (); */
                    Method {
                        name: "decrementAnimationConsumers",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void boostPriority (in uint32_t aCategory); */
                    Method {
                        name: "boostPriority",
                        abi: "C",
                        params: &[Param { name: "aCategory", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

