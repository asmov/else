use std::borrow::Cow;
use gloo_console::log;
use asmov_else_model as model;
use model::{Descriptive, Identifiable, Routing};
use crate::error::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Target {
    None,
    Area(model::UID),
    Route(model::UID),
    Thing(model::UID),
}

impl Target {
    pub fn all() -> &'static [TargetType;3] {
        &TargetType::ALL
    }
}

pub enum TargetType {
    Area,
    Route,
    Thing
}

impl TargetType {
    const ALL: [Self;3] = [Self::Area, Self::Route, Self::Thing];

    pub fn all() -> &'static [Self;3] {
        &Self::ALL
    }

    /// Returns boolean: (area, route, thing)
    pub fn split(types: &[Self]) -> (bool, bool, bool) {
        let mut area = false;
        let mut route = false;
        let mut thing = false;
        for t in types {
            match t {
                Self::Area => area = true,
                Self::Route => route = true,
                Self::Thing => thing = true,
            }
        }

        (area, route, thing)
    }
}

/// Typical target strings:
/// - The keyword, name, or direction name of a nearby Route
/// - The keyword or name of a nearby Thing
/// Exact UIDs are also searched.
impl Target {
    /// Searches for all possible targets partially matching a string.
    /// Returns the target and its name for each match. 
    pub fn search<'w>(partial: &str, interface_view: &'w model::InterfaceView, types: &[TargetType]) -> Vec<(Cow<'w, str>, Self, bool)> {
        let world_view = interface_view.world_view();
        let area_view = world_view.area_view();
        let area_uid = area_view.uid();
        let (find_areas, find_routes, find_things) = TargetType::split(types);
        let mut results = Vec::new();

        // UIDs must be exact
        if let Ok(uid) = partial.parse::<model::UID>() {
            if let Some(route_uid) = area_view.route_uids().iter().find(|id| id == &&uid) {
                let route_name = area_view.indexed_route_name(*route_uid, world_view).unwrap();
                if find_routes {
                    return vec![(route_name, Self::Route(*route_uid), true)]; 
                } else {
                    return results;
                }
            } else if let Some(thing_uid) = area_view.occupant_uids().iter().find(|id| id == &&uid) {
                let thing_name = area_view.indexed_thing_name(*thing_uid, world_view).unwrap();
                if find_things {
                    return vec![(thing_name, Self::Thing(*thing_uid), true)];
                } else {
                    return results;
                }
            } else if area_uid == uid {
                if find_areas {
                    return vec![(Cow::Borrowed(area_view.name()), Self::Area(area_uid), true)];
                } else {
                    return results;
                }
            } else {
                return results;
            }
        }

        let partial = partial.to_lowercase();

        if find_routes {
            // Search routes 
            // - End directions (exact matches return immediately)
            // - End names and keywords
            // returns (name, target, is_exact_match)
            let route_results: Vec<_> = area_view.route_uids().iter()
                .filter_map(|route_uid| {
log!("route_uid: ", route_uid.to_string());
                    let route_uid = *route_uid;
                    let route = world_view.route(route_uid).unwrap(); // expected
                    let end = route.end_for_area(area_uid).unwrap(); // expected
                    
                    // search end direction
                    let direction = end.direction();
                    let direction_name = direction.name_lowercase();
log!("direction: ", direction_name);
log!("end name: ", end.name().to_lowercase());
                    if direction_name.contains(&partial) {
                        if direction_name == partial {
                            Some((Cow::Borrowed(direction.name()), Self::Route(route_uid), true))
                        } else {
                            Some((Cow::Borrowed(direction.name()), Self::Route(route_uid), true))
                        }
                    } else if end.name().to_lowercase().contains(&partial) {
                        let indexed_name = area_view.indexed_route_name(route_uid, world_view).unwrap(); // expected
                        if indexed_name.to_lowercase() == partial {
                            Some((indexed_name, Self::Route(route_uid), true))
                        } else {
log!("indexed end name: ", indexed_name.to_string());

                            Some((indexed_name, Self::Route(route_uid), false))
                        }
                    } else if end.keywords().iter().any(|k| k.contains(&partial)) {
                        let indexed_name = area_view.indexed_route_name(route_uid, world_view).unwrap(); // expected
                        if indexed_name.to_lowercase() == partial {
                            Some((indexed_name, Self::Route(route_uid), true))
                        } else {
                            Some((indexed_name, Self::Route(route_uid), false))
                        }
                    } else {
                        None
                    }
                })
                .collect();
log!("searching: {}", &partial);
log!("found: {}", route_results.len());
            results.extend(route_results);
        }
        if find_things {
            // Search occupants of area
            // - Thing names and keywords
            // returns (name, target, is_exact_match)
            let thing_results: Vec<_> = area_view.occupant_uids().iter()
                .filter_map(|thing_uid| {
                    let thing_uid = *thing_uid;
                    let thing_view = world_view.thing_view(thing_uid).unwrap(); // expected
                    if thing_view.name().to_lowercase().contains(&partial) {
                        let indexed_name = area_view.indexed_thing_name(thing_uid, world_view).unwrap(); // expected
                        if indexed_name.to_lowercase() == partial {
                            Some((indexed_name, Self::Thing(thing_uid), true))
                        } else {
                            Some((indexed_name, Self::Thing(thing_uid), false))
                        }
                    } else if thing_view.keywords().iter().any(|k| k.contains(&partial)) {
                        let indexed_name = area_view.indexed_thing_name(thing_uid, world_view).unwrap(); // expected
                        if indexed_name.to_lowercase() == partial {
                            Some((indexed_name, Self::Thing(thing_uid), true))
                        } else {
                            Some((indexed_name, Self::Thing(thing_uid), false))
                        }
                    } else {
                        None
                    }
                })
                .collect();

            results.extend(thing_results);
        }

        results
    }

    /// Finds a target by an attribute that is unique to it from the perspective of the world view
    /// If the attribute is not unique, an error is returned.
    pub fn find(unique: &str, interface_view: &model::InterfaceView, types: &[TargetType], cmd_field: &'static str) -> Result<Self> {
        let mut results: Vec<_> = Self::search(unique, interface_view, types);
        let total_results = results.len();

        if total_results == 1 {
            return Ok(results.pop().unwrap().1);
        }

        let mut exact_results: Vec<_> = results.into_iter()
            .filter(|(_, _, is_exact)| *is_exact)
            .collect();

        match exact_results.len() {
            0 if total_results == 0 => Err(Error::TargetNotFound { target: cmd_field, search: unique.to_string() }),
            0 if total_results > 0 => Err(Error::TargetNotUnique { target: cmd_field, search: unique.to_string() }),
            1 => Ok(exact_results.pop().unwrap().1),
            _ => Err(Error::TargetNotUnique { target: cmd_field, search: unique.to_string() })
        }  
    }
}
