//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMutableArray.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMutableArray",
            base: Some("nsIArrayExtensions"),
            methods: Some(&[
                    /* void appendElement (in nsISupports element, [optional] in boolean weak); */
                    Method {
                        name: "appendElement",
                        abi: "C",
                        params: &[Param { name: "element", ty: "*const nsISupports" }, Param { name: "weak", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void removeElementAt (in unsigned long index); */
                    Method {
                        name: "removeElementAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void insertElementAt (in nsISupports element, in unsigned long index, in boolean weak); */
                    Method {
                        name: "insertElementAt",
                        abi: "C",
                        params: &[Param { name: "element", ty: "*const nsISupports" }, Param { name: "index", ty: "libc::uint32_t" }, Param { name: "weak", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void replaceElementAt (in nsISupports element, in unsigned long index, in boolean weak); */
                    Method {
                        name: "replaceElementAt",
                        abi: "C",
                        params: &[Param { name: "element", ty: "*const nsISupports" }, Param { name: "index", ty: "libc::uint32_t" }, Param { name: "weak", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void clear (); */
                    Method {
                        name: "clear",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

