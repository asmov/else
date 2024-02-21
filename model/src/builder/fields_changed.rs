use crate::builder::*;

pub struct FieldsChanged {
    /// Created by a Builder::create or Builder::modify.  
    /// Only TreeNodeType::Root, ChangeTreeNode::RootModel, and either ChangeOp::Create or ChangeOp::Modify are valid.
    root: ChangeTreeNode
}

impl FieldsChanged {
    pub fn new(class_ident: &'static ClassIdent) -> Self {
        let root = ChangeTreeNode::RootModel(class_ident);

        Self {
            root 
        }
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
    // Only available to the root node
    RootModel,
    Field,
    Item,
}

pub trait ChangeNode {
    fn node_type(&self) -> TreeNodeType;
    fn op(&self) -> ChangeOp;
    fn subject(&self) -> ChangeSubject;
}

pub trait RootModelSubject {
    fn class_id(&self) -> ClassID;
}

pub trait FieldSubject {
    fn field(&self) -> &'static Field;
}

pub trait ItemSubject {
    fn uid(&self) -> UID;
}

pub trait BranchChangeNode {
    fn children(&self) -> &Vec<ChangeTreeNode>;
}

pub enum ChangeTreeNode {
    RootModel(&'static ClassIdent),
    Scalar(ScalarChangeNode),
    IdentityCollection(IdentityCollectionChangeNode),
    IdentityItem(IdentityItemChangeNode),
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