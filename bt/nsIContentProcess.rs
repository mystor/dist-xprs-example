//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentProcess.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentProcessInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean isAlive; */
                    Method {
                        name: "get_isAlive",
                        abi: "C",
                        params: &[Param { name: "aIsAlive", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute int32_t processId; */
                    Method {
                        name: "get_processId",
                        abi: "C",
                        params: &[Param { name: "aProcessId", ty: "*mut int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIContentProcessInfo opener; */
                    Method {
                        name: "get_opener",
                        abi: "C",
                        params: &[Param { name: "aOpener", ty: "*mut *const nsIContentProcessInfo" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute int32_t tabCount; */
                    Method {
                        name: "get_tabCount",
                        abi: "C",
                        params: &[Param { name: "aTabCount", ty: "*mut int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIMessageSender messageManager; */
                    Method {
                        name: "get_messageManager",
                        abi: "C",
                        params: &[Param { name: "aMessageManager", ty: "*mut *const nsIMessageSender" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIContentProcessProvider",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

