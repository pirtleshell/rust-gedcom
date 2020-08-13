/// Macro for displaying `Option`s in debug mode without the text wrapping.
#[macro_export]
macro_rules! fmt_optional_value {
    ($debug_struct: ident, $prop: literal, $val: expr) => {
        if let Some(value) = $val {
            $debug_struct.field($prop, value);
        } else {
            $debug_struct.field($prop, &"None");
        }
    };
}
