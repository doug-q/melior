use super::{Type};
use mlir_sys::{
    mlirValueDump, mlirValueGetType, mlirValueIsABlockArgument, mlirValueIsAOpResult, MlirValue
};

/// Trait for value-like types.
pub trait ValueLike<'c> {
    /// Converts a value into a raw value.
    fn to_raw(&self) -> MlirValue;

    /// Gets a type.
    fn r#type(&self) -> Type<'c> {
        unsafe { Type::from_raw(mlirValueGetType(self.to_raw())) }
    }

    // /// Gets a location.
    // fn loc(&self) -> Location<'c> {
    //     unsafe { Type::from_raw(mlirValueGetLoc(self.to_raw())) }
    // }

    /// Returns `true` if a value is a block argument.
    fn is_block_argument(&self) -> bool {
        unsafe { mlirValueIsABlockArgument(self.to_raw()) }
    }

    /// Returns `true` if a value is an operation result.
    fn is_operation_result(&self) -> bool {
        unsafe { mlirValueIsAOpResult(self.to_raw()) }
    }

    /// Dumps a value.
    fn dump(&self) {
        unsafe { mlirValueDump(self.to_raw()) }
    }
}
