//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebPageDescriptor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebPageDescriptor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void loadPage (in nsISupports aPageDescriptor, in unsigned long aDisplayType); */
                    Method {
                        name: "loadPage",
                        abi: "C",
                        params: &[Param { name: "aPageDescriptor", ty: "*const nsISupports" }, Param { name: "aDisplayType", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsISupports currentDescriptor; */
                    Method {
                        name: "get_currentDescriptor",
                        abi: "C",
                        params: &[Param { name: "aCurrentDescriptor", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

