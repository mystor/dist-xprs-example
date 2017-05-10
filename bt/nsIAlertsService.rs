//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAlertsService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAlertNotificationImageListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onImageReady (in nsISupports aUserData, in imgIRequest aRequest); */
                    Method {
                        name: "onImageReady",
                        abi: "C",
                        params: &[Param { name: "aUserData", ty: "*const nsISupports" }, Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "nsresult",
                    },

                    /* void onImageMissing (in nsISupports aUserData); */
                    Method {
                        name: "onImageMissing",
                        abi: "C",
                        params: &[Param { name: "aUserData", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIAlertNotification",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init ([optional] in AString aName, [optional] in AString aImageURL, [optional] in AString aTitle, [optional] in AString aText, [optional] in boolean aTextClickable, [optional] in AString aCookie, [optional] in AString aDir, [optional] in AString aLang, [optional] in AString aData, [optional] in nsIPrincipal aPrincipal, [optional] in boolean aInPrivateBrowsing, [optional] in boolean aRequireInteraction); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "aImageURL", ty: "*const nsAString" }, Param { name: "aTitle", ty: "*const nsAString" }, Param { name: "aText", ty: "*const nsAString" }, Param { name: "aTextClickable", ty: "bool" }, Param { name: "aCookie", ty: "*const nsAString" }, Param { name: "aDir", ty: "*const nsAString" }, Param { name: "aLang", ty: "*const nsAString" }, Param { name: "aData", ty: "*const nsAString" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aInPrivateBrowsing", ty: "bool" }, Param { name: "aRequireInteraction", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString imageURL; */
                    Method {
                        name: "get_imageURL",
                        abi: "C",
                        params: &[Param { name: "aImageURL", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString title; */
                    Method {
                        name: "get_title",
                        abi: "C",
                        params: &[Param { name: "aTitle", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString text; */
                    Method {
                        name: "get_text",
                        abi: "C",
                        params: &[Param { name: "aText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean textClickable; */
                    Method {
                        name: "get_textClickable",
                        abi: "C",
                        params: &[Param { name: "aTextClickable", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString cookie; */
                    Method {
                        name: "get_cookie",
                        abi: "C",
                        params: &[Param { name: "aCookie", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString dir; */
                    Method {
                        name: "get_dir",
                        abi: "C",
                        params: &[Param { name: "aDir", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString lang; */
                    Method {
                        name: "get_lang",
                        abi: "C",
                        params: &[Param { name: "aLang", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "get_principal",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*mut *const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI URI; */
                    Method {
                        name: "get_URI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean inPrivateBrowsing; */
                    Method {
                        name: "get_inPrivateBrowsing",
                        abi: "C",
                        params: &[Param { name: "aInPrivateBrowsing", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean requireInteraction; */
                    Method {
                        name: "get_requireInteraction",
                        abi: "C",
                        params: &[Param { name: "aRequireInteraction", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean actionable; */
                    Method {
                        name: "get_actionable",
                        abi: "C",
                        params: &[Param { name: "aActionable", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString source; */
                    Method {
                        name: "get_source",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsICancelable loadImage (in unsigned long aTimeout, in nsIAlertNotificationImageListener aListener, [optional] in nsISupports aUserData); */
                    Method {
                        name: "loadImage",
                        abi: "C",
                        params: &[Param { name: "aTimeout", ty: "libc::uint32_t" }, Param { name: "aListener", ty: "*const nsIAlertNotificationImageListener" }, Param { name: "aUserData", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut *const nsICancelable" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIAlertsService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void showPersistentNotification (in AString aPersistentData, in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener); */
                    Method {
                        name: "showPersistentNotification",
                        abi: "C",
                        params: &[Param { name: "aPersistentData", ty: "*const nsAString" }, Param { name: "aAlert", ty: "*const nsIAlertNotification" }, Param { name: "aAlertListener", ty: "*const nsIObserver" }],
                        ret: "nsresult",
                    },

                    /* void showAlert (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener); */
                    Method {
                        name: "showAlert",
                        abi: "C",
                        params: &[Param { name: "aAlert", ty: "*const nsIAlertNotification" }, Param { name: "aAlertListener", ty: "*const nsIObserver" }],
                        ret: "nsresult",
                    },

                    /* void showAlertNotification (in AString aImageURL, in AString aTitle, in AString aText, [optional] in boolean aTextClickable, [optional] in AString aCookie, [optional] in nsIObserver aAlertListener, [optional] in AString aName, [optional] in AString aDir, [optional] in AString aLang, [optional] in AString aData, [optional] in nsIPrincipal aPrincipal, [optional] in boolean aInPrivateBrowsing, [optional] in boolean aRequireInteraction); */
                    Method {
                        name: "showAlertNotification",
                        abi: "C",
                        params: &[Param { name: "aImageURL", ty: "*const nsAString" }, Param { name: "aTitle", ty: "*const nsAString" }, Param { name: "aText", ty: "*const nsAString" }, Param { name: "aTextClickable", ty: "bool" }, Param { name: "aCookie", ty: "*const nsAString" }, Param { name: "aAlertListener", ty: "*const nsIObserver" }, Param { name: "aName", ty: "*const nsAString" }, Param { name: "aDir", ty: "*const nsAString" }, Param { name: "aLang", ty: "*const nsAString" }, Param { name: "aData", ty: "*const nsAString" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aInPrivateBrowsing", ty: "bool" }, Param { name: "aRequireInteraction", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void closeAlert ([optional] in AString aName, [optional] in nsIPrincipal aPrincipal); */
                    Method {
                        name: "closeAlert",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIAlertsDoNotDisturb",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute bool manualDoNotDisturb; */
                    Method {
                        name: "get_manualDoNotDisturb",
                        abi: "C",
                        params: &[Param { name: "aManualDoNotDisturb", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_manualDoNotDisturb",
                        abi: "C",
                        params: &[Param { name: "aManualDoNotDisturb", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIAlertsIconData",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIAlertsIconURI",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void showAlertWithIconURI (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener, [optional] in nsIURI aIconURI); */
                    Method {
                        name: "showAlertWithIconURI",
                        abi: "C",
                        params: &[Param { name: "aAlert", ty: "*const nsIAlertNotification" }, Param { name: "aAlertListener", ty: "*const nsIObserver" }, Param { name: "aIconURI", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

