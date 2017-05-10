//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURLParser.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURLParser",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void parseURL (in string spec, in long specLen, out unsigned long schemePos, out long schemeLen, out unsigned long authorityPos, out long authorityLen, out unsigned long pathPos, out long pathLen); */
                    Method {
                        name: "parseURL",
                        abi: "C",
                        params: &[Param { name: "spec", ty: "*const libc::c_char" }, Param { name: "specLen", ty: "libc::int32_t" }, Param { name: "schemePos", ty: "*mut libc::uint32_t" }, Param { name: "schemeLen", ty: "*mut libc::int32_t" }, Param { name: "authorityPos", ty: "*mut libc::uint32_t" }, Param { name: "authorityLen", ty: "*mut libc::int32_t" }, Param { name: "pathPos", ty: "*mut libc::uint32_t" }, Param { name: "pathLen", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void parseAuthority (in string authority, in long authorityLen, out unsigned long usernamePos, out long usernameLen, out unsigned long passwordPos, out long passwordLen, out unsigned long hostnamePos, out long hostnameLen, out long port); */
                    Method {
                        name: "parseAuthority",
                        abi: "C",
                        params: &[Param { name: "authority", ty: "*const libc::c_char" }, Param { name: "authorityLen", ty: "libc::int32_t" }, Param { name: "usernamePos", ty: "*mut libc::uint32_t" }, Param { name: "usernameLen", ty: "*mut libc::int32_t" }, Param { name: "passwordPos", ty: "*mut libc::uint32_t" }, Param { name: "passwordLen", ty: "*mut libc::int32_t" }, Param { name: "hostnamePos", ty: "*mut libc::uint32_t" }, Param { name: "hostnameLen", ty: "*mut libc::int32_t" }, Param { name: "port", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void parseUserInfo (in string userinfo, in long userinfoLen, out unsigned long usernamePos, out long usernameLen, out unsigned long passwordPos, out long passwordLen); */
                    Method {
                        name: "parseUserInfo",
                        abi: "C",
                        params: &[Param { name: "userinfo", ty: "*const libc::c_char" }, Param { name: "userinfoLen", ty: "libc::int32_t" }, Param { name: "usernamePos", ty: "*mut libc::uint32_t" }, Param { name: "usernameLen", ty: "*mut libc::int32_t" }, Param { name: "passwordPos", ty: "*mut libc::uint32_t" }, Param { name: "passwordLen", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void parseServerInfo (in string serverinfo, in long serverinfoLen, out unsigned long hostnamePos, out long hostnameLen, out long port); */
                    Method {
                        name: "parseServerInfo",
                        abi: "C",
                        params: &[Param { name: "serverinfo", ty: "*const libc::c_char" }, Param { name: "serverinfoLen", ty: "libc::int32_t" }, Param { name: "hostnamePos", ty: "*mut libc::uint32_t" }, Param { name: "hostnameLen", ty: "*mut libc::int32_t" }, Param { name: "port", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void parsePath (in string path, in long pathLen, out unsigned long filepathPos, out long filepathLen, out unsigned long queryPos, out long queryLen, out unsigned long refPos, out long refLen); */
                    Method {
                        name: "parsePath",
                        abi: "C",
                        params: &[Param { name: "path", ty: "*const libc::c_char" }, Param { name: "pathLen", ty: "libc::int32_t" }, Param { name: "filepathPos", ty: "*mut libc::uint32_t" }, Param { name: "filepathLen", ty: "*mut libc::int32_t" }, Param { name: "queryPos", ty: "*mut libc::uint32_t" }, Param { name: "queryLen", ty: "*mut libc::int32_t" }, Param { name: "refPos", ty: "*mut libc::uint32_t" }, Param { name: "refLen", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void parseFilePath (in string filepath, in long filepathLen, out unsigned long directoryPos, out long directoryLen, out unsigned long basenamePos, out long basenameLen, out unsigned long extensionPos, out long extensionLen); */
                    Method {
                        name: "parseFilePath",
                        abi: "C",
                        params: &[Param { name: "filepath", ty: "*const libc::c_char" }, Param { name: "filepathLen", ty: "libc::int32_t" }, Param { name: "directoryPos", ty: "*mut libc::uint32_t" }, Param { name: "directoryLen", ty: "*mut libc::int32_t" }, Param { name: "basenamePos", ty: "*mut libc::uint32_t" }, Param { name: "basenameLen", ty: "*mut libc::int32_t" }, Param { name: "extensionPos", ty: "*mut libc::uint32_t" }, Param { name: "extensionLen", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void parseFileName (in string filename, in long filenameLen, out unsigned long basenamePos, out long basenameLen, out unsigned long extensionPos, out long extensionLen); */
                    Method {
                        name: "parseFileName",
                        abi: "C",
                        params: &[Param { name: "filename", ty: "*const libc::c_char" }, Param { name: "filenameLen", ty: "libc::int32_t" }, Param { name: "basenamePos", ty: "*mut libc::uint32_t" }, Param { name: "basenameLen", ty: "*mut libc::int32_t" }, Param { name: "extensionPos", ty: "*mut libc::uint32_t" }, Param { name: "extensionLen", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

