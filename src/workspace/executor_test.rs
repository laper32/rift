#[cfg(test)]
mod tests {
    use crate::workspace::executor::{DependencySource, PackageReference};
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_simple_package() {
        let pkg = PackageReference::new("lodash", None);
        assert_eq!(pkg.name, "lodash");
        assert_eq!(pkg.version, None);
        assert_eq!(pkg.source, DependencySource::Explicit);
        assert!(pkg.attributes.is_empty());
    }

    #[test]
    fn test_package_with_version() {
        let pkg = PackageReference::new("lodash", Some("4.17.21".to_string()));
        assert_eq!(pkg.name, "lodash");
        assert_eq!(pkg.version, Some("4.17.21".to_string()));
        assert_eq!(pkg.source, DependencySource::Explicit);
    }

    #[test]
    fn test_with_source_workspace() {
        let pkg = PackageReference::with_source("shared-utils", DependencySource::Workspace);
        assert_eq!(pkg.name, "shared-utils");
        assert_eq!(pkg.version, None);
        assert_eq!(pkg.source, DependencySource::Workspace);
        assert!(pkg.attributes.is_empty());
    }

    #[test]
    fn test_with_source_inherit() {
        let pkg = PackageReference::with_source("config", DependencySource::Inherit);
        assert_eq!(pkg.name, "config");
        assert_eq!(pkg.version, None);
        assert_eq!(pkg.source, DependencySource::Inherit);
        assert!(pkg.attributes.is_empty());
    }

    #[test]
    fn test_parse_with_at_version() {
        let pkg = PackageReference::parse("lodash@4.17.21");
        assert_eq!(pkg.name, "lodash");
        assert_eq!(pkg.version, Some("4.17.21".to_string()));
        assert_eq!(pkg.source, DependencySource::Explicit);
    }

    #[test]
    fn test_language_specific_formats_passed_through() {
        // npm scoped format - passed through as-is
        let pkg = PackageReference::new("angular/core", Some("16.0.0".to_string()));
        assert_eq!(pkg.name, "angular/core");
        assert_eq!(pkg.version, Some("16.0.0".to_string()));

        // Maven format - passed through as-is
        let pkg = PackageReference::new("org.apache.commons", None);
        assert_eq!(pkg.name, "org.apache.commons");

        // Go format - passed through as-is
        let pkg = PackageReference::parse("github.com/user/repo@v1.0.0");
        assert_eq!(pkg.name, "github.com/user/repo");
        assert_eq!(pkg.version, Some("v1.0.0".to_string()));
    }

    #[test]
    fn test_from_string() {
        let pkg: PackageReference = "react@18.2.0".into();
        assert_eq!(pkg.name, "react");
        assert_eq!(pkg.version, Some("18.2.0".to_string()));
        assert_eq!(pkg.source, DependencySource::Explicit);
    }

    #[test]
    fn test_from_tuple() {
        let pkg: PackageReference = ("lodash", Some("4.17.21".to_string())).into();
        assert_eq!(pkg.name, "lodash");
        assert_eq!(pkg.version, Some("4.17.21".to_string()));
        assert_eq!(pkg.source, DependencySource::Explicit);
    }

    #[test]
    fn test_with_attributes() {
        let mut attrs = HashMap::new();
        attrs.insert("features".to_string(), json!(["full", "macros"]));
        attrs.insert("default-features".to_string(), json!(false));

        let pkg = PackageReference::with_attributes("tokio", Some("1.28.0".to_string()), attrs);
        assert_eq!(pkg.name, "tokio");
        assert_eq!(pkg.version, Some("1.28.0".to_string()));
        assert_eq!(pkg.attributes.len(), 2);
        assert_eq!(pkg.source, DependencySource::Explicit);
    }

    #[test]
    fn test_with_attribute_builder() {
        let pkg = PackageReference::new("serde", Some("1.0.0".to_string()))
            .with_attribute("derive", json!(true))
            .with_attribute("features", json!(["derive"]));

        assert_eq!(pkg.attributes.len(), 2);
        assert_eq!(pkg.attributes.get("derive"), Some(&json!(true)));
        assert_eq!(pkg.source, DependencySource::Explicit);
    }

    #[test]
    fn test_from_dependency_value_explicit() {
        use crate::vm::DependencyValue;
        let mut attrs = HashMap::new();
        attrs.insert("dev".to_string(), json!(true));

        let dep_val = DependencyValue {
            name: "lodash".to_string(),
            version: Some("4.17.21".to_string()),
            attributes: attrs.clone(),
        };

        let pkg_ref = PackageReference::from_dependency_value(dep_val);
        assert_eq!(pkg_ref.name, "lodash");
        assert_eq!(pkg_ref.version, Some("4.17.21".to_string()));
        assert_eq!(pkg_ref.source, DependencySource::Explicit);
        assert_eq!(pkg_ref.attributes.get("dev"), Some(&json!(true)));
    }

    #[test]
    fn test_from_dependency_value_workspace() {
        use crate::vm::DependencyValue;
        let mut attrs = HashMap::new();
        attrs.insert("source".to_string(), json!("workspace"));

        let dep_val = DependencyValue {
            name: "shared-utils".to_string(),
            version: None,
            attributes: attrs,
        };

        let pkg_ref = PackageReference::from_dependency_value(dep_val);
        assert_eq!(pkg_ref.name, "shared-utils");
        assert_eq!(pkg_ref.version, None);
        assert_eq!(pkg_ref.source, DependencySource::Workspace);
    }

    #[test]
    fn test_from_dependency_value_inherit() {
        use crate::vm::DependencyValue;
        let mut attrs = HashMap::new();
        attrs.insert("source".to_string(), json!("inherit"));

        let dep_val = DependencyValue {
            name: "config".to_string(),
            version: None,
            attributes: attrs,
        };

        let pkg_ref = PackageReference::from_dependency_value(dep_val);
        assert_eq!(pkg_ref.name, "config");
        assert_eq!(pkg_ref.version, None);
        assert_eq!(pkg_ref.source, DependencySource::Inherit);
    }

    #[test]
    fn test_from_dependency_value_excluded() {
        use crate::vm::DependencyValue;
        let mut attrs = HashMap::new();
        attrs.insert("excluded".to_string(), json!(true));

        let dep_val = DependencyValue {
            name: "unwanted".to_string(),
            version: None,
            attributes: attrs,
        };

        let pkg_ref = PackageReference::from_dependency_value(dep_val);
        assert_eq!(pkg_ref.name, "unwanted");
        assert!(pkg_ref.attributes.contains_key("excluded"));
        assert_eq!(pkg_ref.attributes.get("excluded"), Some(&json!(true)));
    }

    // Note: Integration tests for resolve_dependencies require actual MaybePackage objects
    // which are complex to construct. These will be tested in integration tests.
    // The unit tests above verify the core PackageReference functionality.
}
