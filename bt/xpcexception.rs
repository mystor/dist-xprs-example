//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpcexception.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXPCException",
            base: Some("nsIException"),
            methods: Some(&[
                    /* void initialize (in AUTF8String aMessage, in nsresult aResult, in AUTF8String aName, in nsIStackFrame aLocation, in nsISupports aData); */
                    Method {
                        name: "initialize",
                        abi: "C",
                        params: &[Param { name: "aMessage", ty: "*const nsACString" }, Param { name: "aResult", ty: "nsresult" }, Param { name: "aName", ty: "*const nsACString" }, Param { name: "aLocation", ty: "*const nsIStackFrame" }, Param { name: "aData", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

