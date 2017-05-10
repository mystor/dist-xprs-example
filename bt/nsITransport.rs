//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransport.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITransport",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIInputStream openInputStream (in unsigned long aFlags, in unsigned long aSegmentSize, in unsigned long aSegmentCount); */
                    Method {
                        name: "openInputStream",
                        abi: "C",
                        params: &[Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "aSegmentSize", ty: "libc::uint32_t" }, Param { name: "aSegmentCount", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* nsIOutputStream openOutputStream (in unsigned long aFlags, in unsigned long aSegmentSize, in unsigned long aSegmentCount); */
                    Method {
                        name: "openOutputStream",
                        abi: "C",
                        params: &[Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "aSegmentSize", ty: "libc::uint32_t" }, Param { name: "aSegmentCount", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIOutputStream" }],
                        ret: "nsresult",
                    },

                    /* void close (in nsresult aReason); */
                    Method {
                        name: "close",
                        abi: "C",
                        params: &[Param { name: "aReason", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void setEventSink (in nsITransportEventSink aSink, in nsIEventTarget aEventTarget); */
                    Method {
                        name: "setEventSink",
                        abi: "C",
                        params: &[Param { name: "aSink", ty: "*const nsITransportEventSink" }, Param { name: "aEventTarget", ty: "*const nsIEventTarget" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsITransportEventSink",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onTransportStatus (in nsITransport aTransport, in nsresult aStatus, in long long aProgress, in long long aProgressMax); */
                    Method {
                        name: "onTransportStatus",
                        abi: "C",
                        params: &[Param { name: "aTransport", ty: "*const nsITransport" }, Param { name: "aStatus", ty: "nsresult" }, Param { name: "aProgress", ty: "libc::int64_t" }, Param { name: "aProgressMax", ty: "libc::int64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

