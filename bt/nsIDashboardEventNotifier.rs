//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDashboardEventNotifier.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDashboardEventNotifier",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addHost (in ACString aHost, in unsigned long aSerial, in boolean aEncrypted); */
                    Method {
                        name: "addHost",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*const nsACString" }, Param { name: "aSerial", ty: "libc::uint32_t" }, Param { name: "aEncrypted", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void removeHost (in ACString aHost, in unsigned long aSerial); */
                    Method {
                        name: "removeHost",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*const nsACString" }, Param { name: "aSerial", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void newMsgSent (in ACString aHost, in unsigned long aSerial, in unsigned long aLength); */
                    Method {
                        name: "newMsgSent",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*const nsACString" }, Param { name: "aSerial", ty: "libc::uint32_t" }, Param { name: "aLength", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void newMsgReceived (in ACString aHost, in unsigned long aSerial, in unsigned long aLength); */
                    Method {
                        name: "newMsgReceived",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*const nsACString" }, Param { name: "aSerial", ty: "libc::uint32_t" }, Param { name: "aLength", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

