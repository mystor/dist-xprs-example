//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLOptionsCollection.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLOptionsCollection",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode item (in unsigned long index); */
                    Method {
                        name: "item",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode namedItem (in DOMString name); */
                    Method {
                        name: "namedItem",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* attribute long selectedIndex; */
                    Method {
                        name: "get_selectedIndex",
                        abi: "C",
                        params: &[Param { name: "aSelectedIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_selectedIndex",
                        abi: "C",
                        params: &[Param { name: "aSelectedIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript] readonly attribute nsIDOMHTMLSelectElement select; */
                    Method {
                        name: "get_select",
                        abi: "C",
                        params: &[Param { name: "aSelect", ty: "*mut *const nsIDOMHTMLSelectElement" }],
                        ret: "nsresult",
                    },

                    /* void add (in nsIDOMHTMLOptionElement option, [optional] in nsIVariant before); */
                    Method {
                        name: "add",
                        abi: "C",
                        params: &[Param { name: "option", ty: "*const nsIDOMHTMLOptionElement" }, Param { name: "before", ty: "*const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* void remove (in long index); */
                    Method {
                        name: "remove",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

