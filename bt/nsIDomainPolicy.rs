//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDomainPolicy.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDomainPolicy",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIDomainSet",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [noscript] readonly attribute uint32_t type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void add (in nsIURI aDomain); */
                    Method {
                        name: "add",
                        abi: "C",
                        params: &[Param { name: "aDomain", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* void remove (in nsIURI aDomain); */
                    Method {
                        name: "remove",
                        abi: "C",
                        params: &[Param { name: "aDomain", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* void clear (); */
                    Method {
                        name: "clear",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* bool contains (in nsIURI aDomain); */
                    Method {
                        name: "contains",
                        abi: "C",
                        params: &[Param { name: "aDomain", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* bool containsSuperDomain (in nsIURI aDomain); */
                    Method {
                        name: "containsSuperDomain",
                        abi: "C",
                        params: &[Param { name: "aDomain", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

