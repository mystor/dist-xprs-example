//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISupportsIterators.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIOutputIterator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void putElement (in nsISupports anElementToPut); */
                    Method {
                        name: "putElement",
                        abi: "C",
                        params: &[Param { name: "anElementToPut", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void stepForward (); */
                    Method {
                        name: "stepForward",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIInputIterator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsISupports getElement (); */
                    Method {
                        name: "getElement",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void stepForward (); */
                    Method {
                        name: "stepForward",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean isEqualTo (in nsISupports anotherIterator); */
                    Method {
                        name: "isEqualTo",
                        abi: "C",
                        params: &[Param { name: "anotherIterator", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsISupports clone (); */
                    Method {
                        name: "clone",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIForwardIterator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsISupports getElement (); */
                    Method {
                        name: "getElement",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void putElement (in nsISupports anElementToPut); */
                    Method {
                        name: "putElement",
                        abi: "C",
                        params: &[Param { name: "anElementToPut", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void stepForward (); */
                    Method {
                        name: "stepForward",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean isEqualTo (in nsISupports anotherIterator); */
                    Method {
                        name: "isEqualTo",
                        abi: "C",
                        params: &[Param { name: "anotherIterator", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsISupports clone (); */
                    Method {
                        name: "clone",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIBidirectionalIterator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsISupports getElement (); */
                    Method {
                        name: "getElement",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void putElement (in nsISupports anElementToPut); */
                    Method {
                        name: "putElement",
                        abi: "C",
                        params: &[Param { name: "anElementToPut", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void stepForward (); */
                    Method {
                        name: "stepForward",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void stepBackward (); */
                    Method {
                        name: "stepBackward",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean isEqualTo (in nsISupports anotherIterator); */
                    Method {
                        name: "isEqualTo",
                        abi: "C",
                        params: &[Param { name: "anotherIterator", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsISupports clone (); */
                    Method {
                        name: "clone",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIRandomAccessIterator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsISupports getElement (); */
                    Method {
                        name: "getElement",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* nsISupports getElementAt (in int32_t anOffset); */
                    Method {
                        name: "getElementAt",
                        abi: "C",
                        params: &[Param { name: "anOffset", ty: "int32_t" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void putElement (in nsISupports anElementToPut); */
                    Method {
                        name: "putElement",
                        abi: "C",
                        params: &[Param { name: "anElementToPut", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void putElementAt (in int32_t anOffset, in nsISupports anElementToPut); */
                    Method {
                        name: "putElementAt",
                        abi: "C",
                        params: &[Param { name: "anOffset", ty: "int32_t" }, Param { name: "anElementToPut", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void stepForward (); */
                    Method {
                        name: "stepForward",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void stepForwardBy (in int32_t anOffset); */
                    Method {
                        name: "stepForwardBy",
                        abi: "C",
                        params: &[Param { name: "anOffset", ty: "int32_t" }],
                        ret: "nsresult",
                    },

                    /* void stepBackward (); */
                    Method {
                        name: "stepBackward",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void stepBackwardBy (in int32_t anOffset); */
                    Method {
                        name: "stepBackwardBy",
                        abi: "C",
                        params: &[Param { name: "anOffset", ty: "int32_t" }],
                        ret: "nsresult",
                    },

                    /* boolean isEqualTo (in nsISupports anotherIterator); */
                    Method {
                        name: "isEqualTo",
                        abi: "C",
                        params: &[Param { name: "anotherIterator", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsISupports clone (); */
                    Method {
                        name: "clone",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

