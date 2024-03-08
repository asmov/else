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

    pub fn modify_value<T: Clone>(builder_option: &Option<T>, value: &mut T, fields_changed: &mut FieldsChanged, field: impl Fields) -> Result<()> {
        if !builder_option.is_some() {
            return Ok(())
        }

        let field = field.field();
        *value = builder_option
            .as_ref()
            .expect("Calls to Build::modify_value() should be made after a guard against Option::is_some()")
            .clone();

        Ok(())
    }

    pub fn modify_uid(builder_option: &Option<IdentityBuilder>, value: &mut UID, fields_changed: &mut FieldsChanged, field: impl Fields) -> Result<()> {
        if !builder_option.is_some() {
            return Ok(())
        }

        let field = field.field();
        *value = builder_option
            .as_ref()
            .expect("Calls to Build::modify_value(builder_option) should be made after a guard against Option::is_some()")
            .try_uid()?;

        Ok(())
    }

    pub fn create<B, M>(builder_option: &mut Option<B>, fields_changed: &mut FieldsChanged, field: impl Fields) -> Result<M>
    where
        B: Builder<BuilderType = B, ModelType = M>
    {
        let field = field.field();
        let builder = builder_option.take()
            .ok_or_else(|| Error::FieldNotSet {class: field.classname(), field: field.name()})?;

        debug_assert!(builder.mode_matches(BuilderMode::Creator), "BuilderMode::Editor is not allowed for Build::create()");

        let creation = builder.create()?;
        let (builder, model) = creation.split();
        let _= builder_option.insert(builder);
        //todo: fields_changed
        Ok(model)
    }

    pub fn create_uid(builder_option: &mut Option<IdentityBuilder>, fields_changed: &mut FieldsChanged, field: impl Fields) -> Result<UID> {
        Self::create(builder_option, fields_changed, field)
            .map(|identity| identity.to_uid())
    }

    pub fn create_option<B, M>(builder_option_op: &mut OptionOp<B>, fields_changed: &mut FieldsChanged, field: impl Fields) -> Result<Option<M>>
    where
        B: Builder<BuilderType = B, ModelType = M>
    {
        match builder_option_op {
            OptionOp::None => Ok(None),
            OptionOp::Set(_) => {
                let builder = builder_option_op.take().unwrap();
                debug_assert!(builder.mode_matches(BuilderMode::Creator), "BuilderMode::Editor is not allowed for Build::create_option()");

                let creation = builder.create()?;
                let (builder, model) = creation.split();
                *builder_option_op = OptionOp::Set(builder);
                //todo: fields_changed
                Ok(Some(model))
            },
            OptionOp::Edit(_) => unreachable!("OptionOp::Edit is not allowed in Build::create_option()"),
            OptionOp::Unset => unreachable!("OptionOp::Unset is not allowed in Build::create_option()"),
        }
    }

    pub fn create_uid_option(builder_option_op: &mut OptionOp<IdentityBuilder>, fields_changed: &mut FieldsChanged, field: impl Fields) -> Result<Option<UID>> {
        match builder_option_op {
            OptionOp::None => Ok(None),
            OptionOp::Set(_) => {
                let builder = builder_option_op.take().unwrap();
                debug_assert!(builder.mode_matches(BuilderMode::Creator), "BuilderMode::Editor is not allowed for Build::create_option()");

                let creation = builder.create()?;
                let (builder, model) = creation.split();
                *builder_option_op = OptionOp::Set(builder);
                //todo: fields_changed
                Ok(Some(model.into_uid()))
            },
            OptionOp::Edit(_) => unreachable!("OptionOp::Edit is not allowed in Build::create_option()"),
            OptionOp::Unset => unreachable!("OptionOp::Unset is not allowed in Build::create_option()"),
        }
    }


 
    pub fn modify<B, M>(builder_option: &mut Option<B>, existing: &mut M, fields_changed: &mut FieldsChanged, field: impl Fields) -> Result<()>
    where
        B: Builder<BuilderType = B, ModelType = M>
    {
        if !builder_option.is_some() {
            return Ok(())
        }

        let field = field.field();
        let builder = builder_option.take()
            .ok_or_else(|| Error::FieldNotSet {class: field.classname(), field: field.name()})?;

        if builder.builder_mode() == BuilderMode::Editor {
            let modification = builder.modify(existing)?;
            let (builder, built_fields_changed) = modification.split();
            let _ = builder_option.insert(builder);
            fields_changed.extend(field, ChangeOp::Modify, built_fields_changed);
        } else {
            let creation = builder.create()?;
            let (builder, model) = creation.split();
            *existing = model;
            let _ = builder_option.insert(builder);
            //todo: fields_changed.extend(field, ChangeOp::Create, built_fields_changed);
        }

        Ok(())
    }

    pub fn modify_option<B, M>(builder_option_op: &mut OptionOp<B>, existing_option: &mut Option<M>, fields_changed: &mut FieldsChanged, field: impl Fields) -> Result<()>
    where
        B: Builder<BuilderType = B, ModelType = M>
    {
        match builder_option_op {
            OptionOp::None => Ok(()),
            OptionOp::Set(_) => {
                let value = Self::create_option(builder_option_op, fields_changed, field)?;
                *existing_option = value;
                Ok(())
            },
            OptionOp::Edit(_) => {
                let builder = builder_option_op.take().unwrap();
                let existing = existing_option.as_mut()
                    .expect("Expected existing_option to be Some when OptionOp::Edit");
                let modification = builder.modify(existing)?;
                let (builder, built_fields_changed) = modification.split();
                *builder_option_op = OptionOp::Edit(builder);
                fields_changed.extend(field.field(), ChangeOp::Modify, built_fields_changed);
                Ok(())
            },
            OptionOp::Unset => {
                *existing_option = None;
                //todo: fields changed
                Ok(())
            },
        }
    }

    pub fn modify_uid_option(
        builder_option_op: &mut OptionOp<IdentityBuilder>,
        existing_option: &mut Option<UID>,
        fields_changed: &mut FieldsChanged,
        field: impl Fields
    ) -> Result<()> {
        match builder_option_op {
            OptionOp::None => Ok(()),
            OptionOp::Set(_) => {
                let uid_option = Self::create_uid_option(builder_option_op, fields_changed, field)?;
                *existing_option = uid_option;
                Ok(())
            },
            OptionOp::Edit(_) => unreachable!("OptionOp::Edit is not allowed in Build::modify_uid_option()"),
            OptionOp::Unset => {
                *existing_option = None;
                //todo: fields changed
                Ok(())
            },
        }
    }


    pub fn create_vec<B,M,R>(builder_vec: &mut Vec<ListOp<B,R>>, fields_changed: &mut FieldsChanged, field: impl Fields) -> Result<Vec<M>>
    where
        B: Builder<BuilderType = B, ModelType = M> + MaybeIdentifiable,
        M: Identifiable,
        R: MaybeIdentifiable
    {
        let mut existing_vec = Vec::new();
        Self::modify_vec(builder_vec, &mut existing_vec, fields_changed, field)?;
        Ok(existing_vec)
    }

    pub fn create_uid_vec(builder_vec: &mut Vec<ListOp<UID,UID>>, fields_changed: &mut FieldsChanged, field: impl Fields) -> Result<Vec<UID>> {
        let mut existing_vec = Vec::new();
        Self::modify_uid_vec(builder_vec, &mut existing_vec, fields_changed, field)?;
        Ok(existing_vec)
    }

    // Modifies an existing Vec of models using a Builder's list of VecOps (Add, Modify, Remove)
    pub fn modify_vec<B,M,R>(
        builder_vec: &mut Vec<ListOp<B,R>>,
        existing_vec: &mut Vec<M>,
        fields_changed: &mut FieldsChanged,
        field: impl Fields) -> Result<()>
    where
        B: Builder<BuilderType = B, ModelType = M> + MaybeIdentifiable,
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
                                .ok_or_else(|| Error::ModelNotFound { model: field.field().subject_classname(), uid: builder_uid })?;
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
                                .ok_or_else(|| Error::ModelNotFound { model: field.field().subject_classname(), uid: builder_uid })?;
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
                        .ok_or_else(|| Error::ModelNotFound { model: field.field().subject_classname(), uid: builder_uid })?;
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

    pub fn modify_uid_vec(
        builder_vec: &mut Vec<ListOp<UID,UID>>,
        existing_vec: &mut Vec<UID>,
        fields_changed: &mut FieldsChanged,
        field: impl Fields
    ) -> Result<()> {
        if builder_vec.is_empty() {
            return Ok(())
        }

        builder_vec
            .drain(0..)
            .filter_map(|list_op| { match list_op {
                ListOp::Add(uid) => {
                    if !existing_vec.contains(&uid) {
                        existing_vec.push(uid);
                        Some(Ok(ListOp::Add(uid)))
                    } else {
                        None
                    }
                },
                ListOp::Edit(_) => panic!("Edit not allowed in Build::modify_uid_vec()"),
                ListOp::Remove(uid) => {
                    let index_found = existing_vec
                        .iter()
                        .position(|existing_uid| existing_uid == &uid);
                    if let Some(index) = index_found {
                        existing_vec.remove(index);
                        Some(Ok(ListOp::Remove(uid)))
                    } else {
                        None
                    }
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

    pub fn prepare_modify_composite<B,M>(builder: &mut B, existing: &mut M) -> Result<FieldsChanged>
    where
        B: Builder
    {
        let mut fields_changed = FieldsChanged::from_builder(builder);
        Ok(fields_changed)
    }

    pub fn prepare_modify<B,M>(builder: &mut B, existing: &mut M) -> Result<FieldsChanged>
    where
        B: Builder<ModelType = M> + BuildableIdentity,
        M: Identifiable
    {
        if builder.get_identity().is_none() {
            builder.identity(IdentityBuilder::from_existing(builder, existing))?;
        } else {
            assert_eq!(builder.try_uid()?, existing.uid());
        }
        
        let mut fields_changed = FieldsChanged::from_builder(builder);
        Ok(fields_changed)
    }

    pub fn add_uid_to_listops(uid: UID, listops: &mut Vec<ListOp<UID,UID>>, field: impl Fields) -> Result<()> {
        let ruid = &uid;
        for listop in &*listops {
            match listop {
                ListOp::Add(op_uid) if op_uid == ruid  => {
                    return Ok(())
                },
                ListOp::Edit(op_uid) if op_uid == ruid => {
                    panic!("Edit not allowed in UID ListOp")
                },
                ListOp::Remove(op_uid) if op_uid == ruid => return Err(Error::ListOpRace {
                    op: "add", model: field.field().subject_classname(), uid, whiled: "removed" 
                }),
                _ => {}
            }
        }

        listops.push(ListOp::Add(uid));
        Ok(())
    }

    pub fn remove_uid_from_listops(uid: UID, listops: &mut Vec<ListOp<UID,UID>>, field: impl Fields) -> Result<()> {
        let ruid = &uid;
        for listop in &*listops {
            match listop {
                ListOp::Add(op_uid) if op_uid == ruid  => return Err(Error::ListOpRace {
                    op: "remove", model: field.field().subject_classname(), uid, whiled: "added" 
                }),
                ListOp::Edit(op_uid) if op_uid == ruid => {
                    panic!("Edit not allowed in UID ListOp")
                },
                ListOp::Remove(op_uid) if op_uid == ruid => {
                    return Ok(())
                }
                _ => {}
            }
        }

        listops.push(ListOp::Remove(uid));
        Ok(())
    }
}