//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSPrimitiveValue.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMCSSPrimitiveValue",
            base: Some("nsIDOMCSSValue"),
            methods: Some(&[
                    /* readonly attribute unsigned short primitiveType; */
                    Method {
                        name: "get_primitiveType",
                        abi: "C",
                        params: &[Param { name: "aPrimitiveType", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* void setFloatValue (in unsigned short unitType, in float floatValue) raises (DOMException); */
                    Method {
                        name: "setFloatValue",
                        abi: "C",
                        params: &[Param { name: "unitType", ty: "libc::uint16_t" }, Param { name: "floatValue", ty: "libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* float getFloatValue (in unsigned short unitType) raises (DOMException); */
                    Method {
                        name: "getFloatValue",
                        abi: "C",
                        params: &[Param { name: "unitType", ty: "libc::uint16_t" }, Param { name: "_retval", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* void setStringValue (in unsigned short stringType, in DOMString stringValue) raises (DOMException); */
                    Method {
                        name: "setStringValue",
                        abi: "C",
                        params: &[Param { name: "stringType", ty: "libc::uint16_t" }, Param { name: "stringValue", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* DOMString getStringValue () raises (DOMException); */
                    Method {
                        name: "getStringValue",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMCounter getCounterValue () raises (DOMException); */
                    Method {
                        name: "getCounterValue",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMCounter" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMRect getRectValue () raises (DOMException); */
                    Method {
                        name: "getRectValue",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMRect" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

