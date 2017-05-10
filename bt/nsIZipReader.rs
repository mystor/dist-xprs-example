//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIZipReader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIZipEntry",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned short compression; */
                    Method {
                        name: "get_compression",
                        abi: "C",
                        params: &[Param { name: "aCompression", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long size; */
                    Method {
                        name: "get_size",
                        abi: "C",
                        params: &[Param { name: "aSize", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long realSize; */
                    Method {
                        name: "get_realSize",
                        abi: "C",
                        params: &[Param { name: "aRealSize", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long CRC32; */
                    Method {
                        name: "get_CRC32",
                        abi: "C",
                        params: &[Param { name: "aCRC32", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isDirectory; */
                    Method {
                        name: "get_isDirectory",
                        abi: "C",
                        params: &[Param { name: "aIsDirectory", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute PRTime lastModifiedTime; */
                    Method {
                        name: "get_lastModifiedTime",
                        abi: "C",
                        params: &[Param { name: "aLastModifiedTime", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isSynthetic; */
                    Method {
                        name: "get_isSynthetic",
                        abi: "C",
                        params: &[Param { name: "aIsSynthetic", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long permissions; */
                    Method {
                        name: "get_permissions",
                        abi: "C",
                        params: &[Param { name: "aPermissions", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIZipReader",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void open (in nsIFile zipFile); */
                    Method {
                        name: "open",
                        abi: "C",
                        params: &[Param { name: "zipFile", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void openInner (in nsIZipReader zipReader, in AUTF8String zipEntry); */
                    Method {
                        name: "openInner",
                        abi: "C",
                        params: &[Param { name: "zipReader", ty: "*const nsIZipReader" }, Param { name: "zipEntry", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void openMemory (in voidPtr aData, in unsigned long aLength); */
                    Method {
                        name: "openMemory",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*const libc::c_void" }, Param { name: "aLength", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIFile file; */
                    Method {
                        name: "get_file",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void close (); */
                    Method {
                        name: "close",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void test (in AUTF8String aEntryName); */
                    Method {
                        name: "test",
                        abi: "C",
                        params: &[Param { name: "aEntryName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void extract (in AUTF8String zipEntry, in nsIFile outFile); */
                    Method {
                        name: "extract",
                        abi: "C",
                        params: &[Param { name: "zipEntry", ty: "*const nsACString" }, Param { name: "outFile", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* nsIZipEntry getEntry (in AUTF8String zipEntry); */
                    Method {
                        name: "getEntry",
                        abi: "C",
                        params: &[Param { name: "zipEntry", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIZipEntry" }],
                        ret: "nsresult",
                    },

                    /* boolean hasEntry (in AUTF8String zipEntry); */
                    Method {
                        name: "hasEntry",
                        abi: "C",
                        params: &[Param { name: "zipEntry", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIUTF8StringEnumerator findEntries (in AUTF8String aPattern); */
                    Method {
                        name: "findEntries",
                        abi: "C",
                        params: &[Param { name: "aPattern", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIUTF8StringEnumerator" }],
                        ret: "nsresult",
                    },

                    /* nsIInputStream getInputStream (in AUTF8String zipEntry); */
                    Method {
                        name: "getInputStream",
                        abi: "C",
                        params: &[Param { name: "zipEntry", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* nsIInputStream getInputStreamWithSpec (in AUTF8String aJarSpec, in AUTF8String zipEntry); */
                    Method {
                        name: "getInputStreamWithSpec",
                        abi: "C",
                        params: &[Param { name: "aJarSpec", ty: "*const nsACString" }, Param { name: "zipEntry", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* nsIX509Cert getSigningCert (in AUTF8String aEntryName); */
                    Method {
                        name: "getSigningCert",
                        abi: "C",
                        params: &[Param { name: "aEntryName", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIX509Cert" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t manifestEntriesCount; */
                    Method {
                        name: "get_manifestEntriesCount",
                        abi: "C",
                        params: &[Param { name: "aManifestEntriesCount", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIZipReaderCache",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

