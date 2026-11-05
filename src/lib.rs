#[derive(Debug, Clone, PartialEq)]
pub struct DatabaseField {
    pub name: String,
    pub data_type: String,
}

#[derive(Debug, Clone)]
pub struct BaseTableSchema {
    pub table_name: String,
    pub fields: Vec<DatabaseField>,
}

impl BaseTableSchema {
    pub fn new(table_name: impl Into<String>) -> Self {
        Self { table_name: table_name.into(), fields: Vec::new() }
    }

    pub fn add_field(&mut self, name: impl Into<String>, data_type: impl Into<String>) {
        self.fields.push(DatabaseField { name: name.into(), data_type: data_type.into() });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_schema() {
        // Arrange
        let mut schema = BaseTableSchema::new("employees");
        // Act
        schema.add_field("id", "INTEGER");
        schema.add_field("name", "VARCHAR(100)");
        // Assert
        assert_eq!(schema.fields.len(), 2);
    }
}
