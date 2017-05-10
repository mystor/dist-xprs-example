//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIZipWriter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIZipWriter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute ACString comment; */
                    Method {
                        name: "get_comment",
                        abi: "C",
                        params: &[Param { name: "aComment", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_comment",
                        abi: "C",
                        params: &[Param { name: "aComment", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean inQueue; */
                    Method {
                        name: "get_inQueue",
                        abi: "C",
                        params: &[Param { name: "aInQueue", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIFile file; */
                    Method {
                        name: "get_file",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void open (in nsIFile aFile, in int32_t aIoFlags); */
                    Method {
                        name: "open",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "aIoFlags", ty: "int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIZipEntry getEntry (in AUTF8String aZipEntry); */
                    Method {
                        name: "getEntry",
                        abi: "C",
                        params: &[Param { name: "aZipEntry", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIZipEntry" }],
                        ret: "nsresult",
                    },

                    /* boolean hasEntry (in AUTF8String aZipEntry); */
                    Method {
                        name: "hasEntry",
                        abi: "C",
                        params: &[Param { name: "aZipEntry", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void addEntryDirectory (in AUTF8String aZipEntry, in PRTime aModTime, in boolean aQueue); */
                    Method {
                        name: "addEntryDirectory",
                        abi: "C",
                        params: &[Param { name: "aZipEntry", ty: "*const nsACString" }, Param { name: "aModTime", ty: "PRTime" }, Param { name: "aQueue", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void addEntryFile (in AUTF8String aZipEntry, in int32_t aCompression, in nsIFile aFile, in boolean aQueue); */
                    Method {
                        name: "addEntryFile",
                        abi: "C",
                        params: &[Param { name: "aZipEntry", ty: "*const nsACString" }, Param { name: "aCompression", ty: "int32_t" }, Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "aQueue", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void addEntryChannel (in AUTF8String aZipEntry, in PRTime aModTime, in int32_t aCompression, in nsIChannel aChannel, in boolean aQueue); */
                    Method {
                        name: "addEntryChannel",
                        abi: "C",
                        params: &[Param { name: "aZipEntry", ty: "*const nsACString" }, Param { name: "aModTime", ty: "PRTime" }, Param { name: "aCompression", ty: "int32_t" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aQueue", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void addEntryStream (in AUTF8String aZipEntry, in PRTime aModTime, in int32_t aCompression, in nsIInputStream aStream, in boolean aQueue); */
                    Method {
                        name: "addEntryStream",
                        abi: "C",
                        params: &[Param { name: "aZipEntry", ty: "*const nsACString" }, Param { name: "aModTime", ty: "PRTime" }, Param { name: "aCompression", ty: "int32_t" }, Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aQueue", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void removeEntry (in AUTF8String aZipEntry, in boolean aQueue); */
                    Method {
                        name: "removeEntry",
                        abi: "C",
                        params: &[Param { name: "aZipEntry", ty: "*const nsACString" }, Param { name: "aQueue", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void processQueue (in nsIRequestObserver aObserver, in nsISupports aContext); */
                    Method {
                        name: "processQueue",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIRequestObserver" }, Param { name: "aContext", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void close (); */
                    Method {
                        name: "close",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void alignStoredFiles (in uint16_t aAlignSize); */
                    Method {
                        name: "alignStoredFiles",
                        abi: "C",
                        params: &[Param { name: "aAlignSize", ty: "uint16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

