/// Marker trait for protocol types that can signal experimental usage.
pub trait ExperimentalApi {
    /// Returns a short reason identifier when an experimental method or field is
    /// used, or `None` when the value is entirely stable.
    fn experimental_reason(&self) -> Option<&'static str>;
}

/// Describes an experimental field on a specific type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExperimentalField {
    pub type_name: &'static str,
    pub field_name: &'static str,
    /// Stable identifier returned when this field is used.
    /// Convention: `<method>` for method-level gates or `<method>.<field>` for
    /// field-level gates.
    pub reason: &'static str,
}

inventory::collect!(ExperimentalField);

/// Returns all experimental fields registered across the protocol types.
pub fn experimental_fields() -> Vec<&'static ExperimentalField> {
    inventory::iter::<ExperimentalField>.into_iter().collect()
}

/// Constructs a consistent error message for experimental gating.
pub fn experimental_required_message(reason: &str) -> String {
    format!("{reason} requires experimentalApi capability")
}

#[cfg(test)]
mod tests {
    use super::ExperimentalApi as ExperimentalApiTrait;
    use codex_experimental_api_macros::ExperimentalApi;
    use pretty_assertions::assert_eq;

    #[derive(ExperimentalApi)]
    enum EnumVariantShapes {
        #[experimental("enum/unit")]
        Unit,
        #[experimental("enum/tuple")]
        Tuple(u8),
        #[experimental("enum/named")]
        Named {
            value: u8,
        },
        StableTuple(u8),
    }

    #[test]
    fn derive_supports_all_enum_variant_shapes() {
        let unit = EnumVariantShapes::Unit;
        let tuple = EnumVariantShapes::Tuple(1);
        let named = EnumVariantShapes::Named { value: 1 };
        let stable_tuple = EnumVariantShapes::StableTuple(1);

        assert_eq!(ExperimentalApiTrait::experimental_reason(&unit), Some("enum/unit"));
        assert_eq!(ExperimentalApiTrait::experimental_reason(&tuple), Some("enum/tuple"));
        assert_eq!(ExperimentalApiTrait::experimental_reason(&named), Some("enum/named"));
        assert_eq!(
            ExperimentalApiTrait::experimental_reason(&stable_tuple),
            None
        );

        match tuple {
            EnumVariantShapes::Tuple(value) => assert_eq!(value, 1),
            _ => panic!("expected tuple variant"),
        }
        match named {
            EnumVariantShapes::Named { value } => assert_eq!(value, 1),
            _ => panic!("expected named variant"),
        }
        match stable_tuple {
            EnumVariantShapes::StableTuple(value) => assert_eq!(value, 1),
            _ => panic!("expected stable tuple variant"),
        }
    }
}
