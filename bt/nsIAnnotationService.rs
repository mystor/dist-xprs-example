//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAnnotationService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAnnotationObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onPageAnnotationSet (in nsIURI aPage, in AUTF8String aName); */
                    Method {
                        name: "onPageAnnotationSet",
                        abi: "C",
                        params: &[Param { name: "aPage", ty: "*const nsIURI" }, Param { name: "aName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void onItemAnnotationSet (in long long aItemId, in AUTF8String aName, in unsigned short aSource); */
                    Method {
                        name: "onItemAnnotationSet",
                        abi: "C",
                        params: &[Param { name: "aItemId", ty: "libc::int64_t" }, Param { name: "aName", ty: "*const nsACString" }, Param { name: "aSource", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* void onPageAnnotationRemoved (in nsIURI aURI, in AUTF8String aName); */
                    Method {
                        name: "onPageAnnotationRemoved",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void onItemAnnotationRemoved (in long long aItemId, in AUTF8String aName, in unsigned short aSource); */
                    Method {
                        name: "onItemAnnotationRemoved",
                        abi: "C",
                        params: &[Param { name: "aItemId", ty: "libc::int64_t" }, Param { name: "aName", ty: "*const nsACString" }, Param { name: "aSource", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIAnnotationService",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "mozIAnnotatedResult",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AUTF8String guid; */
                    Method {
                        name: "get_guid",
                        abi: "C",
                        params: &[Param { name: "aGuid", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI uri; */
                    Method {
                        name: "get_uri",
                        abi: "C",
                        params: &[Param { name: "aUri", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long itemId; */
                    Method {
                        name: "get_itemId",
                        abi: "C",
                        params: &[Param { name: "aItemId", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String annotationName; */
                    Method {
                        name: "get_annotationName",
                        abi: "C",
                        params: &[Param { name: "aAnnotationName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIVariant annotationValue; */
                    Method {
                        name: "get_annotationValue",
                        abi: "C",
                        params: &[Param { name: "aAnnotationValue", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

