use crate::{codebase::*, modeling::*};

/// A tree representation of every field changed during a model's creation or operation, including those of nested 
/// models.
#[derive(Debug)]
pub struct FieldsChanged {
    /// Created by a Builder::create or Builder::modify.  
    /// Only TreeNodeType::Root, ChangeTreeNode::RootModel, and either ChangeOp::Create or ChangeOp::Modify are valid.
    root: ModelChangeNode
}

impl FieldsChanged {
    pub fn new(class_ident: &'static ClassIdent, op: ChangeOp) -> Self {
        let root = ModelChangeNode::new(class_ident, op);

        Self {
            root 
        }
    }

    pub fn from_builder(builder: &impl Builder) -> Self {
        let op = match builder.builder_mode() {
            BuilderMode::Creator => ChangeOp::Create,
            BuilderMode::Editor => ChangeOp::Modify
        };

        let root = ModelChangeNode::new(builder.class_ident(), op);

        Self {
            root 
        }
    }

    pub fn extend(&mut self, field: &'static Field, op: ChangeOp, rh: FieldsChanged) {
        let node = match field.value_type() {
            FieldValueType::Model(class_ident) => {
                dbg!(rh.root.class_ident().class_id(), class_ident.class_id());
                assert!(rh.root.class_ident().class_id() == class_ident.class_id());
                let mut model_node = ModelChangeNode::new(class_ident, op);
                model_node.children.extend(rh.root.children);
                ChangeTreeNode::Model(model_node)
            },
            FieldValueType::UIDList(_class_ident) => todo!(),
            FieldValueType::ModelList(_class_ident) => todo!(),
            FieldValueType::StringList => todo!(),
            _ => panic!("FieldsChanged::extend() expects fieild types of Model or Collections")
        };

        self.root.children.push(node);
    }
}

const INVALID_CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Invalid as ClassID, "Invalid"); 

// needed for Serde
impl Default for FieldsChanged {
    fn default() -> Self {
        Self {
            root: ModelChangeNode::new(&INVALID_CLASS_IDENT, ChangeOp::Create)
        }
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

pub trait ModelSubject {
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

#[derive(Debug)]
pub enum ChangeTreeNode {
    Model(ModelChangeNode),
    Scalar(ScalarChangeNode),
    IdentityCollection(IdentityCollectionChangeNode),
    IdentityItem(IdentityItemChangeNode),
}

#[derive(Debug)]
pub struct ModelChangeNode {
    class_ident: &'static ClassIdent,
    op: ChangeOp,
    children: Vec<ChangeTreeNode>
}

impl ChangeNode for ModelChangeNode {
    fn node_type(&self) -> TreeNodeType {
        TreeNodeType::Branch
    }

    fn op(&self) -> ChangeOp {
        self.op
    }

    fn subject(&self) -> ChangeSubject {
        ChangeSubject::Root
    }
}

impl BranchChangeNode for ModelChangeNode {
    fn children(&self) -> &Vec<ChangeTreeNode> {
        &self.children
    }
}

impl ModelSubject for ModelChangeNode {
    fn class_ident(&self) -> &'static ClassIdent {
        self.class_ident
    }
}

impl ModelChangeNode {
    pub fn new(class_ident: &'static ClassIdent, op: ChangeOp) -> Self {
        Self {
            class_ident,
            op,
            children: Vec::new()
        }
    }
}

#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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