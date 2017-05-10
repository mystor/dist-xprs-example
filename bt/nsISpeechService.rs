//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISpeechService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISpeechTaskCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onPause (); */
                    Method {
                        name: "onPause",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void onResume (); */
                    Method {
                        name: "onResume",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void onCancel (); */
                    Method {
                        name: "onCancel",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void onVolumeChanged (in float aVolume); */
                    Method {
                        name: "onVolumeChanged",
                        abi: "C",
                        params: &[Param { name: "aVolume", ty: "libc::c_float" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISpeechTask",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsISpeechService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void speak (in DOMString aText, in DOMString aUri, in float aVolume, in float aRate, in float aPitch, in nsISpeechTask aTask); */
                    Method {
                        name: "speak",
                        abi: "C",
                        params: &[Param { name: "aText", ty: "*const nsAString" }, Param { name: "aUri", ty: "*const nsAString" }, Param { name: "aVolume", ty: "libc::c_float" }, Param { name: "aRate", ty: "libc::c_float" }, Param { name: "aPitch", ty: "libc::c_float" }, Param { name: "aTask", ty: "*const nsISpeechTask" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute SpeechServiceType serviceType; */
                    Method {
                        name: "get_serviceType",
                        abi: "C",
                        params: &[Param { name: "aServiceType", ty: "*mut SpeechServiceType" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

