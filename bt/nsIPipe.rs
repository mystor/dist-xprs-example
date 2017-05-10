//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPipe.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPipe",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [must_use] void init (in boolean nonBlockingInput, in boolean nonBlockingOutput, in unsigned long segmentSize, in unsigned long segmentCount); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "nonBlockingInput", ty: "bool" }, Param { name: "nonBlockingOutput", ty: "bool" }, Param { name: "segmentSize", ty: "libc::uint32_t" }, Param { name: "segmentCount", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute nsIAsyncInputStream inputStream; */
                    Method {
                        name: "get_inputStream",
                        abi: "C",
                        params: &[Param { name: "aInputStream", ty: "*mut *const nsIAsyncInputStream" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute nsIAsyncOutputStream outputStream; */
                    Method {
                        name: "get_outputStream",
                        abi: "C",
                        params: &[Param { name: "aOutputStream", ty: "*mut *const nsIAsyncOutputStream" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISearchableInputStream",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void search (in string forString, in boolean ignoreCase, out boolean found, out unsigned long offsetSearchedTo); */
                    Method {
                        name: "search",
                        abi: "C",
                        params: &[Param { name: "forString", ty: "*const libc::c_char" }, Param { name: "ignoreCase", ty: "bool" }, Param { name: "found", ty: "*mut bool" }, Param { name: "offsetSearchedTo", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

