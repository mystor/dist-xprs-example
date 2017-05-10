//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFContainer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRDFContainer",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIRDFDataSource DataSource; */
                    Method {
                        name: "get_DataSource",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*mut *const nsIRDFDataSource" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIRDFResource Resource; */
                    Method {
                        name: "get_Resource",
                        abi: "C",
                        params: &[Param { name: "aResource", ty: "*mut *const nsIRDFResource" }],
                        ret: "nsresult",
                    },

                    /* void Init (in nsIRDFDataSource aDataSource, in nsIRDFResource aContainer); */
                    Method {
                        name: "Init",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }, Param { name: "aContainer", ty: "*const nsIRDFResource" }],
                        ret: "nsresult",
                    },

                    /* long GetCount (); */
                    Method {
                        name: "GetCount",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator GetElements (); */
                    Method {
                        name: "GetElements",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* void AppendElement (in nsIRDFNode aElement); */
                    Method {
                        name: "AppendElement",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIRDFNode" }],
                        ret: "nsresult",
                    },

                    /* void RemoveElement (in nsIRDFNode aElement, in boolean aRenumber); */
                    Method {
                        name: "RemoveElement",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIRDFNode" }, Param { name: "aRenumber", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void InsertElementAt (in nsIRDFNode aElement, in long aIndex, in boolean aRenumber); */
                    Method {
                        name: "InsertElementAt",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIRDFNode" }, Param { name: "aIndex", ty: "libc::int32_t" }, Param { name: "aRenumber", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* nsIRDFNode RemoveElementAt (in long aIndex, in boolean aRenumber); */
                    Method {
                        name: "RemoveElementAt",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::int32_t" }, Param { name: "aRenumber", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIRDFNode" }],
                        ret: "nsresult",
                    },

                    /* long IndexOf (in nsIRDFNode aElement); */
                    Method {
                        name: "IndexOf",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIRDFNode" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

