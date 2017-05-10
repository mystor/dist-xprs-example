//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleRelation.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleRelation",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long relationType; */
                    Method {
                        name: "get_relationType",
                        abi: "C",
                        params: &[Param { name: "aRelationType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long targetsCount; */
                    Method {
                        name: "get_targetsCount",
                        abi: "C",
                        params: &[Param { name: "aTargetsCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessible getTarget (in unsigned long index); */
                    Method {
                        name: "getTarget",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* nsIArray getTargets (); */
                    Method {
                        name: "getTargets",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

