use crate::builder::*;

/// A tree representation of every field changed during a model's creation or operation, including those of nested 
/// models.
pub struct FieldsChanged {
    /// Created by a Builder::create or Builder::modify.  
    /// Only TreeNodeType::Root, ChangeTreeNode::RootModel, and either ChangeOp::Create or ChangeOp::Modify are valid.
    root: RootChangeNode
}

impl FieldsChanged {
    pub fn new(class_ident: &'static ClassIdent, op: ChangeOp) -> Self {
        let root = RootChangeNode {
            class_ident,
            op,
            children: Vec::new()
        };

        Self {
            root 
        }
    }

    pub fn extend(&mut self, rh: FieldsChanged) {
        self.root.children.extend(rh.root.children);
    }
}

impl ChangeNode for FieldsChanged {
    fn node_type(&self) -> TreeNodeType {
        self.root.node_type()
    }

    fn op(&self) -> ChangeOp {
        self.root.op()
    }

    fn subject(&self) -> ChangeSubject {
        self.root.subject()
    }
}

impl BranchChangeNode for FieldsChanged {
    fn children(&self) -> &Vec<ChangeTreeNode> {
        self.root.children()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TreeNodeType {
    Root,
    Leaf,
    Branch
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ChangeOp {
    Create,
    Add,
    Modify,
    Remove,
    Delete
}

pub enum ChangeSubject {
    /// Only available to the root node
    Root,
    Field,
    Item,
}

pub trait ChangeNode {
    fn node_type(&self) -> TreeNodeType;
    fn op(&self) -> ChangeOp;
    fn subject(&self) -> ChangeSubject;
}

pub trait RootSubject {
    fn class_ident(&self) -> &'static ClassIdent;
}

pub trait FieldSubject {
    fn field(&self) -> &'static Field;
}

pub trait ItemSubject {
    fn uid(&self) -> UID;
}

pub trait BranchChangeNode: ChangeNode {
    fn children(&self) -> &Vec<ChangeTreeNode>;
}

pub enum ChangeTreeNode {
    Root(RootChangeNode),
    Scalar(ScalarChangeNode),
    IdentityCollection(IdentityCollectionChangeNode),
    IdentityItem(IdentityItemChangeNode),
}

pub struct RootChangeNode {
    class_ident: &'static ClassIdent,
    op: ChangeOp,
    children: Vec<ChangeTreeNode>
}

impl ChangeNode for RootChangeNode {
    fn node_type(&self) -> TreeNodeType {
        TreeNodeType::Root
    }

    fn op(&self) -> ChangeOp {
        self.op
    }

    fn subject(&self) -> ChangeSubject {
        ChangeSubject::Root
    }
}

impl BranchChangeNode for RootChangeNode {
    fn children(&self) -> &Vec<ChangeTreeNode> {
        &self.children
    }
}

impl RootSubject for RootChangeNode {
    fn class_ident(&self) -> &'static ClassIdent {
        self.class_ident
    }
}

pub struct ScalarChangeNode {
    field: &'static Field,
    op: ChangeOp
}

impl ChangeNode for ScalarChangeNode {
    fn node_type(&self) -> TreeNodeType {
        TreeNodeType::Leaf
    }

    fn subject(&self) -> ChangeSubject {
        ChangeSubject::Field
    }

    fn op(&self) -> ChangeOp {
        self.op
    }
}

impl FieldSubject for ScalarChangeNode {
    fn field(&self) -> &'static Field {
        self.field 
    }
}

impl ScalarChangeNode {
    pub fn new(field: &'static Field, op: ChangeOp) -> Self {
        Self {
            field,
            op
        }
    }
}

/// Represent a Vector or Map of `impl Identifiable`.
/// Only Modify operations are valid for it. Only Add and Remove operations are valid for its children.
pub struct IdentityCollectionChangeNode {
    field: &'static Field,
    op: ChangeOp,
    /// Can only be ChangeTreeNode::IdentityItem
    items: Vec<ChangeTreeNode>
}

impl FieldSubject for IdentityCollectionChangeNode {
    fn field(&self) -> &'static Field {
        self.field 
    }
}

impl ChangeNode for IdentityCollectionChangeNode {
    fn node_type(&self) -> TreeNodeType {
        TreeNodeType::Branch
    }

    fn subject(&self) -> ChangeSubject {
        ChangeSubject::Item
    }

    fn op(&self) -> ChangeOp {
        self.op
    }
}

impl BranchChangeNode for IdentityCollectionChangeNode {
    fn children(&self) -> &Vec<ChangeTreeNode> {
        &self.items
    }
}

impl IdentityCollectionChangeNode {
    pub fn new(field: &'static Field, op: ChangeOp) -> Self {
        assert!(op == ChangeOp::Modify);

        Self {
            field,
            op,
            items: Vec::new()
        }
    }

    pub fn insert(&mut self, item: IdentityItemChangeNode) {
        self.items.push(ChangeTreeNode::IdentityItem(item));
    }

    pub fn new_item(&mut self, uid: UID, op: ChangeOp) {
        let item = IdentityItemChangeNode::new(uid, op);
        self.items.push(ChangeTreeNode::IdentityItem(item));
    }

    pub fn items(&self) -> &Vec<ChangeTreeNode> {
        &self.children()
    }
}

/// Represents a single `impl Identifiable` item in a `IdentityCollectionChangeNode`.
/// Only Add and Remove operations are valid for it.
pub struct IdentityItemChangeNode {
    uid: UID,
    op: ChangeOp
}

impl ChangeNode for IdentityItemChangeNode {
    fn node_type(&self) -> TreeNodeType {
        TreeNodeType::Leaf
    }

    fn subject(&self) -> ChangeSubject {
        ChangeSubject::Item
    }

    fn op(&self) -> ChangeOp {
        self.op
    }
}

impl ItemSubject for IdentityItemChangeNode {
    fn uid(&self) -> UID {
        self.uid
    }
}

impl IdentityItemChangeNode {
    pub fn new(uid: UID, op: ChangeOp) -> Self {
        Self {
            uid,
            op
        }
    }
}