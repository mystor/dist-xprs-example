//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMTreeWalker.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMTreeWalker",
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

                    /* attribute nsIDOMNode currentNode; */
                    Method {
                        name: "get_currentNode",
                        abi: "C",
                        params: &[Param { name: "aCurrentNode", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_currentNode",
                        abi: "C",
                        params: &[Param { name: "aCurrentNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode parentNode (); */
                    Method {
                        name: "parentNode",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode firstChild (); */
                    Method {
                        name: "firstChild",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode lastChild (); */
                    Method {
                        name: "lastChild",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode previousSibling (); */
                    Method {
                        name: "previousSibling",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode nextSibling (); */
                    Method {
                        name: "nextSibling",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode previousNode (); */
                    Method {
                        name: "previousNode",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode nextNode (); */
                    Method {
                        name: "nextNode",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

