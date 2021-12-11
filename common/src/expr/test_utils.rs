use pb::data::DataType as DataTypeProst;
use pb::data::DataType;

pub fn data_type2(a: i32) -> DataTypeProst {
    DataTypeProst { a }
}

pub fn data_type(a: i32) -> DataType {
    DataType { a }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        data_type(1);
    }
}
