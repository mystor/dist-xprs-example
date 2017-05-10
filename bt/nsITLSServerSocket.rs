//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITLSServerSocket.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITLSServerSocket",
            base: Some("nsIServerSocket"),
            methods: None,
        },


        Interface {
            name: "nsITLSClientStatus",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIX509Cert peerCert; */
                    Method {
                        name: "get_peerCert",
                        abi: "C",
                        params: &[Param { name: "aPeerCert", ty: "*mut *const nsIX509Cert" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute short tlsVersionUsed; */
                    Method {
                        name: "get_tlsVersionUsed",
                        abi: "C",
                        params: &[Param { name: "aTlsVersionUsed", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString cipherName; */
                    Method {
                        name: "get_cipherName",
                        abi: "C",
                        params: &[Param { name: "aCipherName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long keyLength; */
                    Method {
                        name: "get_keyLength",
                        abi: "C",
                        params: &[Param { name: "aKeyLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long macLength; */
                    Method {
                        name: "get_macLength",
                        abi: "C",
                        params: &[Param { name: "aMacLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsITLSServerConnectionInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setSecurityObserver (in nsITLSServerSecurityObserver observer); */
                    Method {
                        name: "setSecurityObserver",
                        abi: "C",
                        params: &[Param { name: "observer", ty: "*const nsITLSServerSecurityObserver" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsITLSServerSocket serverSocket; */
                    Method {
                        name: "get_serverSocket",
                        abi: "C",
                        params: &[Param { name: "aServerSocket", ty: "*mut *const nsITLSServerSocket" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsITLSClientStatus status; */
                    Method {
                        name: "get_status",
                        abi: "C",
                        params: &[Param { name: "aStatus", ty: "*mut *const nsITLSClientStatus" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsITLSServerSecurityObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onHandshakeDone (in nsITLSServerSocket aServer, in nsITLSClientStatus aStatus); */
                    Method {
                        name: "onHandshakeDone",
                        abi: "C",
                        params: &[Param { name: "aServer", ty: "*const nsITLSServerSocket" }, Param { name: "aStatus", ty: "*const nsITLSClientStatus" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

