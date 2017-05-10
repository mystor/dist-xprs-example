//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIASN1Tree.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIASN1Tree",
            base: Some("nsITreeView"),
            methods: Some(&[
                    /* void loadASN1Structure (in nsIASN1Object asn1Object); */
                    Method {
                        name: "loadASN1Structure",
                        abi: "C",
                        params: &[Param { name: "asn1Object", ty: "*const nsIASN1Object" }],
                        ret: "nsresult",
                    },

                    /* AString getDisplayData (in unsigned long index); */
                    Method {
                        name: "getDisplayData",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

