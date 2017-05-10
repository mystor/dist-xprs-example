//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMSVGLength.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMSVGLength",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned short unitType; */
                    Method {
                        name: "get_unitType",
                        abi: "C",
                        params: &[Param { name: "aUnitType", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute float value; */
                    Method {
                        name: "get_value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* attribute float valueInSpecifiedUnits; */
                    Method {
                        name: "get_valueInSpecifiedUnits",
                        abi: "C",
                        params: &[Param { name: "aValueInSpecifiedUnits", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_valueInSpecifiedUnits",
                        abi: "C",
                        params: &[Param { name: "aValueInSpecifiedUnits", ty: "libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString valueAsString; */
                    Method {
                        name: "get_valueAsString",
                        abi: "C",
                        params: &[Param { name: "aValueAsString", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_valueAsString",
                        abi: "C",
                        params: &[Param { name: "aValueAsString", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void newValueSpecifiedUnits (in unsigned short unitType, in float valueInSpecifiedUnits); */
                    Method {
                        name: "newValueSpecifiedUnits",
                        abi: "C",
                        params: &[Param { name: "unitType", ty: "libc::uint16_t" }, Param { name: "valueInSpecifiedUnits", ty: "libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* void convertToSpecifiedUnits (in unsigned short unitType); */
                    Method {
                        name: "convertToSpecifiedUnits",
                        abi: "C",
                        params: &[Param { name: "unitType", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

