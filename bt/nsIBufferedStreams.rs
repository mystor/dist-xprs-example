//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBufferedStreams.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIBufferedInputStream",
            base: Some("nsIInputStream"),
            methods: Some(&[
                    /* void init (in nsIInputStream fillFromStream, in unsigned long bufferSize); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "fillFromStream", ty: "*const nsIInputStream" }, Param { name: "bufferSize", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIBufferedOutputStream",
            base: Some("nsIOutputStream"),
            methods: Some(&[
                    /* void init (in nsIOutputStream sinkToStream, in unsigned long bufferSize); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "sinkToStream", ty: "*const nsIOutputStream" }, Param { name: "bufferSize", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

