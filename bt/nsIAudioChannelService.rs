//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAudioChannelService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAudioChannelService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* float getAudioChannelVolume (in mozIDOMWindowProxy window, in unsigned short audioChannel); */
                    Method {
                        name: "getAudioChannelVolume",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }, Param { name: "audioChannel", ty: "libc::uint16_t" }, Param { name: "_retval", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* void setAudioChannelVolume (in mozIDOMWindowProxy window, in unsigned short audioChannel, in float volume); */
                    Method {
                        name: "setAudioChannelVolume",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }, Param { name: "audioChannel", ty: "libc::uint16_t" }, Param { name: "volume", ty: "libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* boolean getAudioChannelMuted (in mozIDOMWindowProxy window, in unsigned short audioChannel); */
                    Method {
                        name: "getAudioChannelMuted",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }, Param { name: "audioChannel", ty: "libc::uint16_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void setAudioChannelMuted (in mozIDOMWindowProxy window, in unsigned short audioChannel, in boolean muted); */
                    Method {
                        name: "setAudioChannelMuted",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }, Param { name: "audioChannel", ty: "libc::uint16_t" }, Param { name: "muted", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isAudioChannelActive (in mozIDOMWindowProxy window, in unsigned short audioChannel); */
                    Method {
                        name: "isAudioChannelActive",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }, Param { name: "audioChannel", ty: "libc::uint16_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

