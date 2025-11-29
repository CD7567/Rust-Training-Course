// This chapter is dedicated to the smart pointers: Box, Rc and RefCell.

// Box
// ================================================================================================

// ----- 1 --------------------------------------
// Implement a recursive `BinaryTreeNode` which have:
// - fields:
//   - `value: i32`
//   - `left_child: Option<BinaryTreeNode>`
//   - `right_child: Option<BinaryTreeNode>`
// - methods:
//   - `new(value: i32)`, which creates a note with provided value and without any children
//   - `with_children(value: i32, left_child: BinaryTreeNode, right_child: BinaryTreeNode)` which
//     creates a note using the provided values
//   - `sum(&self)` which computes the sum of all values in the tree
//
// Use `Box` if needed

use std::rc::Rc;

// IMPLEMENT HERE:
pub struct BinaryTreeNode {
    value: i32,
    left_child: Option<Box<BinaryTreeNode>>,
    right_child: Option<Box<BinaryTreeNode>>,
}

impl BinaryTreeNode {
    pub fn new(value: i32) -> Self {
        BinaryTreeNode {
            value,
            left_child: None,
            right_child: None,
        }
    }

    pub fn with_children(
        value: i32,
        left_child: BinaryTreeNode,
        right_child: BinaryTreeNode,
    ) -> Self {
        BinaryTreeNode {
            value,
            left_child: Some(Box::new(left_child)),
            right_child: Some(Box::new(right_child)),
        }
    }

    pub fn sum(&self) -> i32 {
        let left_sum = self.left_child.as_ref().map_or(0, |node| node.sum());
        let right_sum = self.right_child.as_ref().map_or(0, |node| node.sum());
        self.value + left_sum + right_sum
    }
}

// Rc
// ================================================================================================

// ----- 2 --------------------------------------
// Implement a package dependency tree where multiple packages can depend on the same shared
// library.
//
// Implement the `Package` struct with `name: String` and `dependencies: Vec<Package>` fields.
// Implement methods:
// - `new(name: &str) -> Self` which creates a new package with provided name and without any
//   dependencies.
// - `with_dependencies(name: &str, dependencies: Vec<Package>) -> Self` which creates a new package
//   with provided name and dependencies.
// - `list_dependencies(package: Package) -> Vec<String>` which return a vector of all dependencies
//   of this package (including all recursive dependencies).
//
// Write a test which will reuse the created Packages in several other Packages as dependencies.
// Use `Rc` in the `Package` struct where needed to avoid deep clone.

// IMPLEMENT HERE:
pub struct Package {
    name: String,
    dependencies: Vec<Rc<Package>>,
}

impl Package {
    pub fn new(name: &str) -> Self {
        Package {
            name: name.to_string(),
            dependencies: Vec::new(),
        }
    }

    pub fn with_dependencies(name: &str, dependencies: Vec<Rc<Package>>) -> Self {
        Package { name: name.to_string(), dependencies }
    }

    pub fn list_dependencies(&self) -> Vec<String> {
        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();

        self._collect_dependencies(&mut result, &mut visited);
        result
    }

    fn _collect_dependencies(
        &self,
        result: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
    ) {
        for dep in &self.dependencies {
            if !visited.contains(&dep.name) {
                visited.insert(dep.name.clone());
                result.push(dep.name.clone());
                dep._collect_dependencies(result, visited);
            }
        }
    }
}

#[test]
fn test_list_dependencies() {
    let std_lib = Rc::new(Package::new("std"));
    let net_lib = Rc::new(Package::new("net"));

    let web_server =
        Rc::new(Package::with_dependencies("web-server", vec![std_lib.clone(), net_lib.clone()]));

    let database = Rc::new(Package::with_dependencies("database", vec![std_lib.clone()]));

    let app = Package::with_dependencies("app", vec![web_server.clone(), database.clone()]);

    let dependencies = app.list_dependencies();

    assert!(dependencies.contains(&"web-server".to_string()));
    assert!(dependencies.contains(&"database".to_string()));
    assert!(dependencies.contains(&"std".to_string()));
    assert!(dependencies.contains(&"net".to_string()));

    let std_count = dependencies.iter().filter(|&s| s == "std").count();
    assert_eq!(std_count, 1, "std library should appear only once");
}

// RefCell
// ================================================================================================

// ----- 3 --------------------------------------
// Create a simple `SharedCounter` where multiple owners can increment its value without mutable
// reference.
//
// Implement `new() -> Self` constructor, `increment(&self)` and `get(&self) -> i32` methods.
// Use `RefCell` where needed.

// IMPLEMENT HERE:
pub struct SharedCounter {
    value: i32,
}

impl SharedCounter {
    pub fn new() -> Self {
        !unimplemented!()
    }

    pub fn increment(&self) {
        !unimplemented!()
    }

    pub fn get(&self) -> i32 {
        !unimplemented!()
    }
}
