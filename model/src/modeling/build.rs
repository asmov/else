use crate::{error::*, modeling::*};

pub struct Build;
impl Build {
    pub fn create_value<T: Clone>(builder_option: &Option<T>, fields_changed: &mut FieldsChanged, field: impl Fields) -> Result<T> {
        let field = field.field();
        let value = builder_option
            .as_ref()
            .ok_or_else(|| Error::FieldNotSet {class: field.classname(), field: field.name()})?
            .clone();

        //todo: fields_changed
        Ok(value)
    }

    pub fn modify_value<T: Clone>(builder_option: &Option<T>, fields_changed: &mut FieldsChanged, field: impl Fields) -> Result<T> {
        let field = field.field();
        let value = builder_option
            .as_ref()
            .expect("Calls to Build::modify_value() should be made after a guard against Option::is_some()")
            .clone();

        //todo: fields_changed
        Ok(value)
    }

    pub fn create<B, M>(builder_option: &mut Option<B>, fields_changed: &mut FieldsChanged, field: impl Fields) -> Result<M>
    where
        B: Builder<BuilderType = B, ModelType = M>
    {
        let field = field.field();
        let builder = builder_option.take()
            .ok_or_else(|| Error::FieldNotSet {class: field.classname(), field: field.name()})?;

        if builder.builder_mode() == BuilderMode::Editor {
            panic!("BuilderMode::Editor is not allowed for Build::create()")
        }

        let creation = builder.create()?;
        let (builder, model) = creation.split();
        builder_option.insert(builder);
        //todo: fields_changed
        Ok(model)
    }

    pub fn modify<B, M>(builder_option: &mut Option<B>, existing: &mut M, fields_changed: &mut FieldsChanged, field: impl Fields) -> Result<()>
    where
        B: Builder<BuilderType = B, ModelType = M>
    {
        let field = field.field();
        let builder = builder_option.take()
            .ok_or_else(|| Error::FieldNotSet {class: field.classname(), field: field.name()})?;

        if builder.builder_mode() == BuilderMode::Creator {
            panic!("BuilderMode::Creator is not allowed for Build::modify()")
        }

        let modification = builder.modify(existing)?;
        let (builder, built_fields_changed) = modification.split();
        let _ = builder_option.insert(builder);
        fields_changed.extend(field, ChangeOp::Modify, built_fields_changed);
        Ok(())
    }


    pub fn create_vec<B,M,R>(builder_vec: &mut Vec<ListOp<B,R>>, fields_changed: &mut FieldsChanged, field: impl Fields) -> Result<Vec<M>>
    where
        B: Builder<BuilderType = B, ModelType = M> + BuildableIdentity,
        M: Identifiable,
        R: MaybeIdentifiable
    {
        let mut existing_vec = Vec::new();
        Self::modify_vec(builder_vec, &mut existing_vec, fields_changed, field)?;
        Ok(existing_vec)
    }

    // Modifies an existing Vec of models using a Builder's list of VecOps (Add, Modify, Remove)
    pub fn modify_vec<B,M,R>(
        builder_vec: &mut Vec<ListOp<B,R>>,
        existing_vec: &mut Vec<M>,
        fields_changed: &mut FieldsChanged,
        field: impl Fields) -> Result<()>
    where
        B: Builder<BuilderType = B, ModelType = M> + BuildableIdentity,
        M: Identifiable,
        R: MaybeIdentifiable
    {
        builder_vec
            .drain(0..)
            .map(|list_op| { match list_op {
                ListOp::Add(builder) => {
                    match builder.builder_mode() {
                        BuilderMode::Creator => {
                            let creation = builder.create()?;
                            let (builder, model) = creation.split();
                            existing_vec.push(model);
                            Ok(ListOp::Add(builder))
                        },
                        BuilderMode::Editor => {
                            let builder_uid = builder.try_uid()?;
                            let existing = existing_vec
                                .iter_mut()
                                .find(|existing| existing.uid() == builder_uid)
                                .ok_or_else(|| Error::ModelNotFound { model: field.field().classname(), uid: builder_uid })?;
                            let modification = builder.modify(existing)?;
                            let builder = modification.take_builder();
                            
                            Ok(ListOp::Add(builder))
                        }
                    }
                },
                ListOp::Edit(builder) => {
                    match builder.builder_mode() {
                        BuilderMode::Editor => {
                            let builder_uid = builder.try_uid()?;
                            let existing = existing_vec
                                .iter_mut()
                                .find(|existing| existing.uid() == builder_uid)
                                .ok_or_else(|| Error::ModelNotFound { model: field.field().classname(), uid: builder_uid })?;
                            let modification = builder.modify(existing)?;
                            let builder = modification.take_builder();
                            Ok(ListOp::Add(builder))
                        },
                        BuilderMode::Creator => unreachable!("BuilderMode::Creator is not allowed for VecOp::Modify in Build::modify_vec()")
                    }
                },
                ListOp::Remove(maybe_identifiable) => {
                    let builder_uid = maybe_identifiable.try_uid()?;
                    let index = existing_vec
                        .iter()
                        .position(|existing| existing.uid() == builder_uid)
                        .ok_or_else(|| Error::ModelNotFound { model: field.field().classname(), uid: builder_uid })?;
                    existing_vec.remove(index);
                    Ok(ListOp::Remove(maybe_identifiable))
                }
            }
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .for_each(|list_op| {
            builder_vec.push(list_op);
        });

        Ok(())
    }
}