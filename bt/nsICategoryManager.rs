//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICategoryManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICategoryManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* string getCategoryEntry (in string aCategory, in string aEntry); */
                    Method {
                        name: "getCategoryEntry",
                        abi: "C",
                        params: &[Param { name: "aCategory", ty: "*const libc::c_char" }, Param { name: "aEntry", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* string addCategoryEntry (in string aCategory, in string aEntry, in string aValue, in boolean aPersist, in boolean aReplace); */
                    Method {
                        name: "addCategoryEntry",
                        abi: "C",
                        params: &[Param { name: "aCategory", ty: "*const libc::c_char" }, Param { name: "aEntry", ty: "*const libc::c_char" }, Param { name: "aValue", ty: "*const libc::c_char" }, Param { name: "aPersist", ty: "bool" }, Param { name: "aReplace", ty: "bool" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void deleteCategoryEntry (in string aCategory, in string aEntry, in boolean aPersist); */
                    Method {
                        name: "deleteCategoryEntry",
                        abi: "C",
                        params: &[Param { name: "aCategory", ty: "*const libc::c_char" }, Param { name: "aEntry", ty: "*const libc::c_char" }, Param { name: "aPersist", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void deleteCategory (in string aCategory); */
                    Method {
                        name: "deleteCategory",
                        abi: "C",
                        params: &[Param { name: "aCategory", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator enumerateCategory (in string aCategory); */
                    Method {
                        name: "enumerateCategory",
                        abi: "C",
                        params: &[Param { name: "aCategory", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator enumerateCategories (); */
                    Method {
                        name: "enumerateCategories",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

