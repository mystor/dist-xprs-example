//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISynthVoiceRegistry.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISynthVoiceRegistry",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addVoice (in nsISpeechService aService, in DOMString aUri, in DOMString aName, in DOMString aLang, in boolean aLocalService, in boolean aQueuesUtterances); */
                    Method {
                        name: "addVoice",
                        abi: "C",
                        params: &[Param { name: "aService", ty: "*const nsISpeechService" }, Param { name: "aUri", ty: "*const nsAString" }, Param { name: "aName", ty: "*const nsAString" }, Param { name: "aLang", ty: "*const nsAString" }, Param { name: "aLocalService", ty: "bool" }, Param { name: "aQueuesUtterances", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void removeVoice (in nsISpeechService aService, in DOMString aUri); */
                    Method {
                        name: "removeVoice",
                        abi: "C",
                        params: &[Param { name: "aService", ty: "*const nsISpeechService" }, Param { name: "aUri", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void notifyVoicesChanged (); */
                    Method {
                        name: "notifyVoicesChanged",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void setDefaultVoice (in DOMString aUri, in boolean aIsDefault); */
                    Method {
                        name: "setDefaultVoice",
                        abi: "C",
                        params: &[Param { name: "aUri", ty: "*const nsAString" }, Param { name: "aIsDefault", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t voiceCount; */
                    Method {
                        name: "get_voiceCount",
                        abi: "C",
                        params: &[Param { name: "aVoiceCount", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* AString getVoice (in uint32_t aIndex); */
                    Method {
                        name: "getVoice",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* bool isDefaultVoice (in DOMString aUri); */
                    Method {
                        name: "isDefaultVoice",
                        abi: "C",
                        params: &[Param { name: "aUri", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* bool isLocalVoice (in DOMString aUri); */
                    Method {
                        name: "isLocalVoice",
                        abi: "C",
                        params: &[Param { name: "aUri", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* AString getVoiceLang (in DOMString aUri); */
                    Method {
                        name: "getVoiceLang",
                        abi: "C",
                        params: &[Param { name: "aUri", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getVoiceName (in DOMString aUri); */
                    Method {
                        name: "getVoiceName",
                        abi: "C",
                        params: &[Param { name: "aUri", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

