//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULStore.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXULStore",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setValue (in AString doc, in AString id, in AString attr, in AString value); */
                    Method {
                        name: "setValue",
                        abi: "C",
                        params: &[Param { name: "doc", ty: "*const nsAString" }, Param { name: "id", ty: "*const nsAString" }, Param { name: "attr", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* bool hasValue (in AString doc, in AString id, in AString attr); */
                    Method {
                        name: "hasValue",
                        abi: "C",
                        params: &[Param { name: "doc", ty: "*const nsAString" }, Param { name: "id", ty: "*const nsAString" }, Param { name: "attr", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* AString getValue (in AString doc, in AString id, in AString attr); */
                    Method {
                        name: "getValue",
                        abi: "C",
                        params: &[Param { name: "doc", ty: "*const nsAString" }, Param { name: "id", ty: "*const nsAString" }, Param { name: "attr", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void removeValue (in AString doc, in AString id, in AString attr); */
                    Method {
                        name: "removeValue",
                        abi: "C",
                        params: &[Param { name: "doc", ty: "*const nsAString" }, Param { name: "id", ty: "*const nsAString" }, Param { name: "attr", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsIStringEnumerator getIDsEnumerator (in AString doc); */
                    Method {
                        name: "getIDsEnumerator",
                        abi: "C",
                        params: &[Param { name: "doc", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIStringEnumerator" }],
                        ret: "nsresult",
                    },

                    /* nsIStringEnumerator getAttributeEnumerator (in AString doc, in AString id); */
                    Method {
                        name: "getAttributeEnumerator",
                        abi: "C",
                        params: &[Param { name: "doc", ty: "*const nsAString" }, Param { name: "id", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIStringEnumerator" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

