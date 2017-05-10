//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIArray.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIArray",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void queryElementAt (in unsigned long index, in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
                    Method {
                        name: "queryElementAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "uuid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* unsigned long indexOf (in unsigned long startIndex, in nsISupports element); */
                    Method {
                        name: "indexOf",
                        abi: "C",
                        params: &[Param { name: "startIndex", ty: "libc::uint32_t" }, Param { name: "element", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator enumerate (); */
                    Method {
                        name: "enumerate",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

