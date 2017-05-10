//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamingProtocolController.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStreamingProtocolMetaData",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute uint32_t frameType; */
                    Method {
                        name: "get_frameType",
                        abi: "C",
                        params: &[Param { name: "aFrameType", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_frameType",
                        abi: "C",
                        params: &[Param { name: "aFrameType", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute uint32_t totalTracks; */
                    Method {
                        name: "get_totalTracks",
                        abi: "C",
                        params: &[Param { name: "aTotalTracks", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_totalTracks",
                        abi: "C",
                        params: &[Param { name: "aTotalTracks", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute ACString mimeType; */
                    Method {
                        name: "get_mimeType",
                        abi: "C",
                        params: &[Param { name: "aMimeType", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_mimeType",
                        abi: "C",
                        params: &[Param { name: "aMimeType", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long width; */
                    Method {
                        name: "get_width",
                        abi: "C",
                        params: &[Param { name: "aWidth", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_width",
                        abi: "C",
                        params: &[Param { name: "aWidth", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long height; */
                    Method {
                        name: "get_height",
                        abi: "C",
                        params: &[Param { name: "aHeight", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_height",
                        abi: "C",
                        params: &[Param { name: "aHeight", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long long duration; */
                    Method {
                        name: "get_duration",
                        abi: "C",
                        params: &[Param { name: "aDuration", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_duration",
                        abi: "C",
                        params: &[Param { name: "aDuration", ty: "libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long sampleRate; */
                    Method {
                        name: "get_sampleRate",
                        abi: "C",
                        params: &[Param { name: "aSampleRate", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_sampleRate",
                        abi: "C",
                        params: &[Param { name: "aSampleRate", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long long timeStamp; */
                    Method {
                        name: "get_timeStamp",
                        abi: "C",
                        params: &[Param { name: "aTimeStamp", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_timeStamp",
                        abi: "C",
                        params: &[Param { name: "aTimeStamp", ty: "libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long channelCount; */
                    Method {
                        name: "get_channelCount",
                        abi: "C",
                        params: &[Param { name: "aChannelCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_channelCount",
                        abi: "C",
                        params: &[Param { name: "aChannelCount", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute ACString esdsData; */
                    Method {
                        name: "get_esdsData",
                        abi: "C",
                        params: &[Param { name: "aEsdsData", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_esdsData",
                        abi: "C",
                        params: &[Param { name: "aEsdsData", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute ACString avccData; */
                    Method {
                        name: "get_avccData",
                        abi: "C",
                        params: &[Param { name: "aAvccData", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_avccData",
                        abi: "C",
                        params: &[Param { name: "aAvccData", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIStreamingProtocolListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onMediaDataAvailable (in uint8_t index, in ACString data, in uint32_t length, in uint32_t offset, in nsIStreamingProtocolMetaData meta); */
                    Method {
                        name: "onMediaDataAvailable",
                        abi: "C",
                        params: &[Param { name: "index", ty: "uint8_t" }, Param { name: "data", ty: "*const nsACString" }, Param { name: "length", ty: "uint32_t" }, Param { name: "offset", ty: "uint32_t" }, Param { name: "meta", ty: "*const nsIStreamingProtocolMetaData" }],
                        ret: "nsresult",
                    },

                    /* void onConnected (in uint8_t index, in nsIStreamingProtocolMetaData meta); */
                    Method {
                        name: "onConnected",
                        abi: "C",
                        params: &[Param { name: "index", ty: "uint8_t" }, Param { name: "meta", ty: "*const nsIStreamingProtocolMetaData" }],
                        ret: "nsresult",
                    },

                    /* void onDisconnected (in uint8_t index, in nsresult reason); */
                    Method {
                        name: "onDisconnected",
                        abi: "C",
                        params: &[Param { name: "index", ty: "uint8_t" }, Param { name: "reason", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIStreamingProtocolController",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in nsIURI aUri); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aUri", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* void asyncOpen (in nsIStreamingProtocolListener aListener); */
                    Method {
                        name: "asyncOpen",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIStreamingProtocolListener" }],
                        ret: "nsresult",
                    },

                    /* nsIStreamingProtocolMetaData getTrackMetaData (in octet index); */
                    Method {
                        name: "getTrackMetaData",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint8_t" }, Param { name: "_retval", ty: "*mut *const nsIStreamingProtocolMetaData" }],
                        ret: "nsresult",
                    },

                    /* void play (); */
                    Method {
                        name: "play",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void pause (); */
                    Method {
                        name: "pause",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void resume (); */
                    Method {
                        name: "resume",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void suspend (); */
                    Method {
                        name: "suspend",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void seek (in unsigned long long seekTimeUs); */
                    Method {
                        name: "seek",
                        abi: "C",
                        params: &[Param { name: "seekTimeUs", ty: "libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    /* void stop (); */
                    Method {
                        name: "stop",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void playbackEnded (); */
                    Method {
                        name: "playbackEnded",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute octet totalTracks; */
                    Method {
                        name: "get_totalTracks",
                        abi: "C",
                        params: &[Param { name: "aTotalTracks", ty: "*mut libc::uint8_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

