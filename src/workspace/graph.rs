use anyhow::{Result, anyhow};
use std::collections::{HashMap, HashSet};

use crate::workspace::executor::{DependencySource, PackageReference, ScriptResult};

/// Edge in the dependency graph
#[derive(Debug, Clone)]
pub struct DependencyEdge {
    /// The package that declares the dependency
    pub from: String,
    /// The package being depended on
    pub to: String,
    /// Version constraint (if specified)
    pub version_constraint: Option<String>,
}

/// Dependency graph representing internal package dependencies
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// All packages in the workspace
    pub packages: HashSet<String>,
    /// Internal dependencies (both packages are in workspace)
    pub internal_edges: Vec<DependencyEdge>,
    /// Topologically sorted build order
    pub build_order: Vec<String>,
}

impl DependencyGraph {
    /// Build a dependency graph from script execution results
    /// packages: all package names in the workspace
    /// results: script execution results for each package
    pub fn build(
        packages: &HashSet<String>,
        results: &HashMap<String, ScriptResult>,
    ) -> Result<Self> {
        let mut graph = Self {
            packages: packages.clone(),
            internal_edges: Vec::new(),
            build_order: Vec::new(),
        };

        // Build internal dependency edges
        for (pkg_name, result) in results {
            for dep in &result.dependencies {
                // Only record internal dependencies (both packages in workspace)
                if packages.contains(&dep.name) {
                    graph.internal_edges.push(DependencyEdge {
                        from: pkg_name.clone(),
                        to: dep.name.clone(),
                        version_constraint: dep.version.clone(),
                    });
                }
            }
        }

        // Detect circular dependencies
        graph.detect_cycles()?;

        // Compute build order via topological sort
        graph.topological_sort()?;

        Ok(graph)
    }

    /// Detect circular dependencies using DFS
    fn detect_cycles(&self) -> Result<()> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path: Vec<String> = Vec::new();

        // Build adjacency list for easier traversal
        let adj = self.build_adjacency_list();

        for pkg in &self.packages {
            if !visited.contains(pkg) {
                if let Some(cycle) =
                    self.dfs_detect_cycle(pkg, &adj, &mut visited, &mut rec_stack, &mut path)
                {
                    return Err(anyhow!(
                        "Circular dependency detected: {}",
                        cycle.join(" -> ")
                    ));
                }
            }
        }

        Ok(())
    }

    /// DFS helper to detect cycles
    /// Returns Some(cycle_path) if a cycle is found, None otherwise
    fn dfs_detect_cycle(
        &self,
        pkg: &str,
        adj: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(pkg.to_string());
        rec_stack.insert(pkg.to_string());
        path.push(pkg.to_string());

        if let Some(neighbors) = adj.get(pkg) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if let Some(cycle) =
                        self.dfs_detect_cycle(neighbor, adj, visited, rec_stack, path)
                    {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(neighbor) {
                    // Found a cycle - extract the cycle path
                    let cycle_start = path.iter().position(|p| p == neighbor).unwrap();
                    let mut cycle = path[cycle_start..].to_vec();
                    cycle.push(neighbor.clone());
                    return Some(cycle);
                }
            }
        }

        path.pop();
        rec_stack.remove(pkg);
        None
    }

    /// Build adjacency list from edges
    fn build_adjacency_list(&self) -> HashMap<String, Vec<String>> {
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        for edge in &self.internal_edges {
            adj.entry(edge.from.clone())
                .or_insert_with(Vec::new)
                .push(edge.to.clone());
        }
        adj
    }

    /// Topological sort using Kahn's algorithm
    fn topological_sort(&mut self) -> Result<()> {
        let adj = self.build_adjacency_list();

        // Calculate out-degree (number of dependencies) for each node
        let mut out_degree: HashMap<String, usize> = HashMap::new();
        for pkg in &self.packages {
            out_degree.insert(pkg.clone(), 0);
        }
        for edge in &self.internal_edges {
            *out_degree.entry(edge.from.clone()).or_insert(0) += 1;
        }

        // Initialize queue with nodes having no dependencies (zero out-degree)
        let mut queue: Vec<String> = out_degree
            .iter()
            .filter(|&(_, &degree)| degree == 0)
            .map(|(pkg, _)| pkg.clone())
            .collect();

        // Process nodes in topological order
        let mut result = Vec::new();
        while let Some(pkg) = queue.pop() {
            result.push(pkg.clone());

            // Find packages that depend on this package and reduce their dependency count
            for edge in &self.internal_edges {
                if edge.to == pkg {
                    if let Some(degree) = out_degree.get_mut(&edge.from) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(edge.from.clone());
                        }
                    }
                }
            }
        }

        // Check if all nodes were processed (no cycles)
        if result.len() != self.packages.len() {
            return Err(anyhow!(
                "Failed to compute build order - possible circular dependency"
            ));
        }

        self.build_order = result;
        Ok(())
    }

    /// Get dependencies for a specific package
    pub fn get_dependencies(&self, pkg: &str) -> Vec<&DependencyEdge> {
        self.internal_edges
            .iter()
            .filter(|edge| edge.from == pkg)
            .collect()
    }

    /// Get dependents (packages that depend on this package)
    pub fn get_dependents(&self, pkg: &str) -> Vec<&DependencyEdge> {
        self.internal_edges
            .iter()
            .filter(|edge| edge.to == pkg)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::executor::{PackageReference, ScriptResult};

    #[test]
    fn test_simple_dag() {
        let mut packages = HashSet::new();
        packages.insert("root".to_string());
        packages.insert("lib1".to_string());
        packages.insert("lib2".to_string());

        let mut results = HashMap::new();
        results.insert(
            "root".to_string(),
            ScriptResult {
                package_name: "root".to_string(),
                dependencies: vec![
                    PackageReference {
                        name: "lib1".to_string(),
                        version: None,
                        source: DependencySource::Explicit,
                        attributes: HashMap::new(),
                    },
                    PackageReference {
                        name: "lib2".to_string(),
                        version: None,
                        source: DependencySource::Explicit,
                        attributes: HashMap::new(),
                    },
                ],
                plugins: vec![],
                config: HashMap::new(),
                tasks: vec![],
            },
        );
        results.insert(
            "lib1".to_string(),
            ScriptResult {
                package_name: "lib1".to_string(),
                dependencies: vec![],
                plugins: vec![],
                config: HashMap::new(),
                tasks: vec![],
            },
        );
        results.insert(
            "lib2".to_string(),
            ScriptResult {
                package_name: "lib2".to_string(),
                dependencies: vec![],
                plugins: vec![],
                config: HashMap::new(),
                tasks: vec![],
            },
        );

        let graph = DependencyGraph::build(&packages, &results).unwrap();

        // lib1 and lib2 should come before root
        let root_idx = graph.build_order.iter().position(|x| x == "root").unwrap();
        let lib1_idx = graph.build_order.iter().position(|x| x == "lib1").unwrap();
        let lib2_idx = graph.build_order.iter().position(|x| x == "lib2").unwrap();

        assert!(lib1_idx < root_idx);
        assert!(lib2_idx < root_idx);
    }

    #[test]
    fn test_circular_dependency() {
        let mut packages = HashSet::new();
        packages.insert("a".to_string());
        packages.insert("b".to_string());

        let mut results = HashMap::new();
        results.insert(
            "a".to_string(),
            ScriptResult {
                package_name: "a".to_string(),
                dependencies: vec![PackageReference {
                    name: "b".to_string(),
                    version: None,
                    source: DependencySource::Explicit,
                    attributes: HashMap::new(),
                }],
                plugins: vec![],
                config: HashMap::new(),
                tasks: vec![],
            },
        );
        results.insert(
            "b".to_string(),
            ScriptResult {
                package_name: "b".to_string(),
                dependencies: vec![PackageReference {
                    name: "a".to_string(),
                    version: None,
                    source: DependencySource::Explicit,
                    attributes: HashMap::new(),
                }],
                plugins: vec![],
                config: HashMap::new(),
                tasks: vec![],
            },
        );

        let result = DependencyGraph::build(&packages, &results);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Circular dependency")
        );
    }

    #[test]
    fn test_external_dependencies_ignored() {
        let mut packages = HashSet::new();
        packages.insert("root".to_string());

        let mut results = HashMap::new();
        results.insert(
            "root".to_string(),
            ScriptResult {
                package_name: "root".to_string(),
                dependencies: vec![
                    // External dependency - should be ignored in graph
                    PackageReference {
                        name: "lodash".to_string(),
                        version: Some("4.17.21".to_string()),
                        source: DependencySource::Explicit,
                        attributes: HashMap::new(),
                    },
                ],
                plugins: vec![],
                config: HashMap::new(),
                tasks: vec![],
            },
        );

        let graph = DependencyGraph::build(&packages, &results).unwrap();
        assert_eq!(graph.internal_edges.len(), 0);
        assert_eq!(graph.build_order.len(), 1);
    }
}
