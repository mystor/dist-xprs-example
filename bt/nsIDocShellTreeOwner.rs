//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocShellTreeOwner.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDocShellTreeOwner",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void contentShellAdded (in nsIDocShellTreeItem aContentShell, in boolean aPrimary); */
                    Method {
                        name: "contentShellAdded",
                        abi: "C",
                        params: &[Param { name: "aContentShell", ty: "*const nsIDocShellTreeItem" }, Param { name: "aPrimary", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void contentShellRemoved (in nsIDocShellTreeItem aContentShell); */
                    Method {
                        name: "contentShellRemoved",
                        abi: "C",
                        params: &[Param { name: "aContentShell", ty: "*const nsIDocShellTreeItem" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDocShellTreeItem primaryContentShell; */
                    Method {
                        name: "get_primaryContentShell",
                        abi: "C",
                        params: &[Param { name: "aPrimaryContentShell", ty: "*mut *const nsIDocShellTreeItem" }],
                        ret: "nsresult",
                    },

                    /* void tabParentAdded (in nsITabParent aTab, in boolean aPrimary); */
                    Method {
                        name: "tabParentAdded",
                        abi: "C",
                        params: &[Param { name: "aTab", ty: "*const nsITabParent" }, Param { name: "aPrimary", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void tabParentRemoved (in nsITabParent aTab); */
                    Method {
                        name: "tabParentRemoved",
                        abi: "C",
                        params: &[Param { name: "aTab", ty: "*const nsITabParent" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsITabParent primaryTabParent; */
                    Method {
                        name: "get_primaryTabParent",
                        abi: "C",
                        params: &[Param { name: "aPrimaryTabParent", ty: "*mut *const nsITabParent" }],
                        ret: "nsresult",
                    },

                    /* void sizeShellTo (in nsIDocShellTreeItem shell, in long cx, in long cy); */
                    Method {
                        name: "sizeShellTo",
                        abi: "C",
                        params: &[Param { name: "shell", ty: "*const nsIDocShellTreeItem" }, Param { name: "cx", ty: "libc::int32_t" }, Param { name: "cy", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void getPrimaryContentSize (out long width, out long height); */
                    Method {
                        name: "getPrimaryContentSize",
                        abi: "C",
                        params: &[Param { name: "width", ty: "*mut libc::int32_t" }, Param { name: "height", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setPrimaryContentSize (in long width, in long height); */
                    Method {
                        name: "setPrimaryContentSize",
                        abi: "C",
                        params: &[Param { name: "width", ty: "libc::int32_t" }, Param { name: "height", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void getRootShellSize (out long width, out long height); */
                    Method {
                        name: "getRootShellSize",
                        abi: "C",
                        params: &[Param { name: "width", ty: "*mut libc::int32_t" }, Param { name: "height", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setRootShellSize (in long width, in long height); */
                    Method {
                        name: "setRootShellSize",
                        abi: "C",
                        params: &[Param { name: "width", ty: "libc::int32_t" }, Param { name: "height", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setPersistence (in boolean aPersistPosition, in boolean aPersistSize, in boolean aPersistSizeMode); */
                    Method {
                        name: "setPersistence",
                        abi: "C",
                        params: &[Param { name: "aPersistPosition", ty: "bool" }, Param { name: "aPersistSize", ty: "bool" }, Param { name: "aPersistSizeMode", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void getPersistence (out boolean aPersistPosition, out boolean aPersistSize, out boolean aPersistSizeMode); */
                    Method {
                        name: "getPersistence",
                        abi: "C",
                        params: &[Param { name: "aPersistPosition", ty: "*mut bool" }, Param { name: "aPersistSize", ty: "*mut bool" }, Param { name: "aPersistSizeMode", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long tabCount; */
                    Method {
                        name: "get_tabCount",
                        abi: "C",
                        params: &[Param { name: "aTabCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute bool hasPrimaryContent; */
                    Method {
                        name: "get_hasPrimaryContent",
                        abi: "C",
                        params: &[Param { name: "aHasPrimaryContent", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

