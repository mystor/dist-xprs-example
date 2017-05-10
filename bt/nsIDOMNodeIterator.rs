//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMNodeIterator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMNodeIterator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMNode root; */
                    Method {
                        name: "get_root",
                        abi: "C",
                        params: &[Param { name: "aRoot", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long whatToShow; */
                    Method {
                        name: "get_whatToShow",
                        abi: "C",
                        params: &[Param { name: "aWhatToShow", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMNodeFilter filter; */
                    Method {
                        name: "get_filter",
                        abi: "C",
                        params: &[Param { name: "aFilter", ty: "*mut *const nsIDOMNodeFilter" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode nextNode () raises (DOMException); */
                    Method {
                        name: "nextNode",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode previousNode () raises (DOMException); */
                    Method {
                        name: "previousNode",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void detach (); */
                    Method {
                        name: "detach",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMNode referenceNode; */
                    Method {
                        name: "get_referenceNode",
                        abi: "C",
                        params: &[Param { name: "aReferenceNode", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean pointerBeforeReferenceNode; */
                    Method {
                        name: "get_pointerBeforeReferenceNode",
                        abi: "C",
                        params: &[Param { name: "aPointerBeforeReferenceNode", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

