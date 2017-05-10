//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebSocketEventService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebSocketFrame",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [must_use] readonly attribute DOMHighResTimeStamp timeStamp; */
                    Method {
                        name: "get_timeStamp",
                        abi: "C",
                        params: &[Param { name: "aTimeStamp", ty: "*mut DOMHighResTimeStamp" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute boolean finBit; */
                    Method {
                        name: "get_finBit",
                        abi: "C",
                        params: &[Param { name: "aFinBit", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute boolean rsvBit1; */
                    Method {
                        name: "get_rsvBit1",
                        abi: "C",
                        params: &[Param { name: "aRsvBit1", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute boolean rsvBit2; */
                    Method {
                        name: "get_rsvBit2",
                        abi: "C",
                        params: &[Param { name: "aRsvBit2", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute boolean rsvBit3; */
                    Method {
                        name: "get_rsvBit3",
                        abi: "C",
                        params: &[Param { name: "aRsvBit3", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute unsigned short opCode; */
                    Method {
                        name: "get_opCode",
                        abi: "C",
                        params: &[Param { name: "aOpCode", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute boolean maskBit; */
                    Method {
                        name: "get_maskBit",
                        abi: "C",
                        params: &[Param { name: "aMaskBit", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute unsigned long mask; */
                    Method {
                        name: "get_mask",
                        abi: "C",
                        params: &[Param { name: "aMask", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute ACString payload; */
                    Method {
                        name: "get_payload",
                        abi: "C",
                        params: &[Param { name: "aPayload", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIWebSocketEventListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [must_use] void webSocketCreated (in unsigned long aWebSocketSerialID, in AString aURI, in ACString aProtocols); */
                    Method {
                        name: "webSocketCreated",
                        abi: "C",
                        params: &[Param { name: "aWebSocketSerialID", ty: "libc::uint32_t" }, Param { name: "aURI", ty: "*const nsAString" }, Param { name: "aProtocols", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void webSocketOpened (in unsigned long aWebSocketSerialID, in AString aEffectiveURI, in ACString aProtocols, in ACString aExtensions); */
                    Method {
                        name: "webSocketOpened",
                        abi: "C",
                        params: &[Param { name: "aWebSocketSerialID", ty: "libc::uint32_t" }, Param { name: "aEffectiveURI", ty: "*const nsAString" }, Param { name: "aProtocols", ty: "*const nsACString" }, Param { name: "aExtensions", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void webSocketMessageAvailable (in unsigned long aWebSocketSerialID, in ACString aMessage, in unsigned short aType); */
                    Method {
                        name: "webSocketMessageAvailable",
                        abi: "C",
                        params: &[Param { name: "aWebSocketSerialID", ty: "libc::uint32_t" }, Param { name: "aMessage", ty: "*const nsACString" }, Param { name: "aType", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void webSocketClosed (in unsigned long aWebSocketSerialID, in boolean aWasClean, in unsigned short aCode, in AString aReason); */
                    Method {
                        name: "webSocketClosed",
                        abi: "C",
                        params: &[Param { name: "aWebSocketSerialID", ty: "libc::uint32_t" }, Param { name: "aWasClean", ty: "bool" }, Param { name: "aCode", ty: "libc::uint16_t" }, Param { name: "aReason", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void frameReceived (in unsigned long aWebSocketSerialID, in nsIWebSocketFrame aFrame); */
                    Method {
                        name: "frameReceived",
                        abi: "C",
                        params: &[Param { name: "aWebSocketSerialID", ty: "libc::uint32_t" }, Param { name: "aFrame", ty: "*const nsIWebSocketFrame" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void frameSent (in unsigned long aWebSocketSerialID, in nsIWebSocketFrame aFrame); */
                    Method {
                        name: "frameSent",
                        abi: "C",
                        params: &[Param { name: "aWebSocketSerialID", ty: "libc::uint32_t" }, Param { name: "aFrame", ty: "*const nsIWebSocketFrame" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIWebSocketEventService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [must_use] void addListener (in unsigned long long aInnerWindowID, in nsIWebSocketEventListener aListener); */
                    Method {
                        name: "addListener",
                        abi: "C",
                        params: &[Param { name: "aInnerWindowID", ty: "libc::uint64_t" }, Param { name: "aListener", ty: "*const nsIWebSocketEventListener" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void removeListener (in unsigned long long aInnerWindowID, in nsIWebSocketEventListener aListener); */
                    Method {
                        name: "removeListener",
                        abi: "C",
                        params: &[Param { name: "aInnerWindowID", ty: "libc::uint64_t" }, Param { name: "aListener", ty: "*const nsIWebSocketEventListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

