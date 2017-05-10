//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStartupCache.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStartupCache",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* uint32_t getBuffer (in string aID, out charPtr aBuffer); */
                    Method {
                        name: "getBuffer",
                        abi: "C",
                        params: &[Param { name: "aID", ty: "*const libc::c_char" }, Param { name: "aBuffer", ty: "*mut *const u8" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void putBuffer (in string aID, in string aBuffer, in uint32_t aLength); */
                    Method {
                        name: "putBuffer",
                        abi: "C",
                        params: &[Param { name: "aID", ty: "*const libc::c_char" }, Param { name: "aBuffer", ty: "*const libc::c_char" }, Param { name: "aLength", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void invalidateCache (); */
                    Method {
                        name: "invalidateCache",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* nsIObjectOutputStream getDebugObjectOutputStream (in nsIObjectOutputStream aStream); */
                    Method {
                        name: "getDebugObjectOutputStream",
                        abi: "C",
                        params: &[Param { name: "aStream", ty: "*const nsIObjectOutputStream" }, Param { name: "_retval", ty: "*mut *const nsIObjectOutputStream" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIObserver observer; */
                    Method {
                        name: "get_observer",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*mut *const nsIObserver" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

