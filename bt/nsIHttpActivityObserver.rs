//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpActivityObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpActivityObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [must_use] void observeActivity (in nsISupports aHttpChannel, in uint32_t aActivityType, in uint32_t aActivitySubtype, in PRTime aTimestamp, in uint64_t aExtraSizeData, in ACString aExtraStringData); */
                    Method {
                        name: "observeActivity",
                        abi: "C",
                        params: &[Param { name: "aHttpChannel", ty: "*const nsISupports" }, Param { name: "aActivityType", ty: "uint32_t" }, Param { name: "aActivitySubtype", ty: "uint32_t" }, Param { name: "aTimestamp", ty: "PRTime" }, Param { name: "aExtraSizeData", ty: "uint64_t" }, Param { name: "aExtraStringData", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute boolean isActive; */
                    Method {
                        name: "get_isActive",
                        abi: "C",
                        params: &[Param { name: "aIsActive", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIHttpActivityDistributor",
            base: Some("nsIHttpActivityObserver"),
            methods: Some(&[
                    /* void addObserver (in nsIHttpActivityObserver aObserver); */
                    Method {
                        name: "addObserver",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIHttpActivityObserver" }],
                        ret: "nsresult",
                    },

                    /* void removeObserver (in nsIHttpActivityObserver aObserver); */
                    Method {
                        name: "removeObserver",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIHttpActivityObserver" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

