//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAudioChannelAgent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISuspendedTypes",
            base: Some("nsISupports"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "nsIAudioChannelAgentCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void windowVolumeChanged (in float aVolume, in bool aMuted); */
                    Method {
                        name: "windowVolumeChanged",
                        abi: "C",
                        params: &[Param { name: "aVolume", ty: "libc::c_float" }, Param { name: "aMuted", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void windowSuspendChanged (in uint32_t aSuspend); */
                    Method {
                        name: "windowSuspendChanged",
                        abi: "C",
                        params: &[Param { name: "aSuspend", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void windowAudioCaptureChanged (in bool aCapture); */
                    Method {
                        name: "windowAudioCaptureChanged",
                        abi: "C",
                        params: &[Param { name: "aCapture", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIAudioChannelAgent",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

