#![allow(warnings)]
use cssparser::{Parser, ParserInput, RuleListParser};

use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use crate::entity::Entity;
use crate::hierarchy::Hierarchy;
use crate::state::storage::animatable_storage::AnimatableStorage;
use crate::state::storage::dense_storage::DenseStorage;
use crate::state::storage::style_storage::StyleStorage;

use crate::state::animator::AnimationState;

pub mod theme;

pub mod prop;
pub use prop::PropSet;

pub mod flexbox;
pub use flexbox::*;

// pub mod grid;
// pub use grid::*;

pub mod layout;
pub use layout::*;

pub mod shape;
pub use shape::*;

pub mod text;
pub use text::*;

pub mod display;
pub use display::*;

pub mod property;
pub use property::*;

// pub mod stylesheet;
// pub use stylesheet::StyleSheet;

pub mod selector;
pub use selector::*;

pub mod specificity;
pub use specificity::*;

pub mod rule;
pub use rule::*;

pub mod color;
pub use color::Color;

pub mod trans;
pub use trans::Scale;

pub struct Style {
    pub style_rules: Vec<StyleRule>,

    //pub rules: Vec<usize>,
    pub rule_selectors: Vec<Vec<Selector>>,

    pub ids: DenseStorage<u64>,

    pub elements: DenseStorage<u64>,

    //replace with combinator storage at some point
    pub classes: DenseStorage<HashSet<String>>,

    //replace with custom bitmask storage for pseudoclasses
    //pub pseudo_classes: DenseStorage<HashSet<PseudoClass>>,

    // Merge these
    //pub enabled: DenseStorage<bool>,
    //pub checked: DenseStorage<bool>,
    //pub over: DenseStorage<bool>,
    pub pseudo_classes: DenseStorage<PseudoClasses>,

    pub z_order: DenseStorage<i32>,

    // Transform
    pub rotate: AnimatableStorage<f32>, // in degrees
    pub scaley: AnimatableStorage<Scale>,

    // General
    pub display: StyleStorage<Display>,
    pub visibility: StyleStorage<Visibility>,
    pub opacity: AnimatableStorage<Opacity>,

    pub overflow: StyleStorage<Overflow>,

    pub scroll: DenseStorage<Scroll>,

    // Positioning
    pub position: StyleStorage<Position>,
    pub left: AnimatableStorage<Length>,
    pub right: AnimatableStorage<Length>,
    pub top: AnimatableStorage<Length>,
    pub bottom: AnimatableStorage<Length>,

    // Size
    pub width: AnimatableStorage<Length>,
    pub height: AnimatableStorage<Length>,

    // Size Constraints
    pub max_width: StyleStorage<Length>,
    pub max_height: StyleStorage<Length>,
    pub min_width: StyleStorage<Length>,
    pub min_height: StyleStorage<Length>,

    // Margin
    pub margin_left: AnimatableStorage<Length>,
    pub margin_right: AnimatableStorage<Length>,
    pub margin_top: AnimatableStorage<Length>,
    pub margin_bottom: AnimatableStorage<Length>,

    // Padding
    pub padding_left: StyleStorage<Length>,
    pub padding_right: StyleStorage<Length>,
    pub padding_top: StyleStorage<Length>,
    pub padding_bottom: StyleStorage<Length>,

    // Border
    pub border_width: StyleStorage<f32>,
    pub border_color: StyleStorage<Color>,
    pub border_radius: StyleStorage<BorderRadius>,

    pub clip_widget: DenseStorage<Entity>,

    pub focus_order: DenseStorage<FocusOrder>,

    pub align_self: StyleStorage<AlignSelf>,
    pub flex_grow: AnimatableStorage<f32>,
    pub flex_shrink: StyleStorage<f32>,
    pub flex_basis: StyleStorage<f32>,

    //pub grid_item: DenseStorage<GridItem>,

    //pub justification: DenseStorage<Justification>,
    //pub alignment: DenseStorage<Alignment>,

    //pub flex_container: DenseStorage<FlexContainer>,
    pub flex_direction: StyleStorage<FlexDirection>,
    pub justify_content: StyleStorage<JustifyContent>,
    pub align_items: StyleStorage<AlignItems>,
    pub align_content: StyleStorage<AlignContent>,

    // pub area_container: DenseStorage<AreaContainer>,
    //pub grid_container: DenseStorage<GridContainer>,

    // Shape
    //pub background: DenseStorage<Background>,
    pub background_color: AnimatableStorage<Color>,
    pub background_image: StyleStorage<String>,

    pub box_shadow: DenseStorage<BoxShadow>,

    // Layout Properties
    //pub size_constraints: DenseStorage<SizeConstraints>,
    //pub resize: CascadeStorage<Resize>,

    //Text Properties
    pub text: DenseStorage<Text>,

    pub font_color: AnimatableStorage<Color>,

    pub text_align: StyleStorage<Align>,
    pub text_justify: StyleStorage<Justify>,
}

impl Style {
    pub fn new() -> Self {
        Style {
            style_rules: Vec::new(),

            //rules: Vec::new(),
            rule_selectors: Vec::new(),

            ids: DenseStorage::new(),
            elements: DenseStorage::new(),
            classes: DenseStorage::new(),
            pseudo_classes: DenseStorage::new(),

            //enabled: DenseStorage::new(),
            //checked: DenseStorage::new(),
            //over: DenseStorage::new(),
            opacity: AnimatableStorage::new(),

            z_order: DenseStorage::new(),

            // Transform
            rotate: AnimatableStorage::new(),
            scaley: AnimatableStorage::new(),

            // Positioning
            position: StyleStorage::new(),
            left: AnimatableStorage::new(),
            right: AnimatableStorage::new(),
            top: AnimatableStorage::new(),
            bottom: AnimatableStorage::new(),

            // Size
            width: AnimatableStorage::new(),
            height: AnimatableStorage::new(),

            // Size Constraints
            max_width: StyleStorage::new(),
            max_height: StyleStorage::new(),
            min_width: StyleStorage::new(),
            min_height: StyleStorage::new(),

            // Margin
            margin_left: AnimatableStorage::new(),
            margin_right: AnimatableStorage::new(),
            margin_top: AnimatableStorage::new(),
            margin_bottom: AnimatableStorage::new(),

            // Padding
            padding_left: StyleStorage::new(),
            padding_right: StyleStorage::new(),
            padding_top: StyleStorage::new(),
            padding_bottom: StyleStorage::new(),

            // Border
            border_width: StyleStorage::new(),
            border_color: StyleStorage::new(),
            border_radius: StyleStorage::new(),

            // Flex Container
            flex_direction: StyleStorage::new(),
            justify_content: StyleStorage::new(),
            align_items: StyleStorage::new(),
            align_content: StyleStorage::new(),

            // Text
            text_align: StyleStorage::new(),
            text_justify: StyleStorage::new(),

            font_color: AnimatableStorage::new(),

            overflow: StyleStorage::new(),
            scroll: DenseStorage::new(),

            // area_container: DenseStorage::new(),
            // area_item: DenseStorage::new(),
            display: StyleStorage::new(),
            visibility: StyleStorage::new(),
            clip_widget: DenseStorage::new(),
            focus_order: DenseStorage::new(),
            box_shadow: DenseStorage::new(),

            background_color: AnimatableStorage::new(),
            background_image: StyleStorage::new(),

            //justification: DenseStorage::new(),
            //alignment: DenseStorage::new(),

            //flex_container: DenseStorage::new(),
            //flex_item: DenseStorage::new(),
            align_self: StyleStorage::new(),
            flex_grow: AnimatableStorage::new(),
            flex_shrink: StyleStorage::new(),
            flex_basis: StyleStorage::new(),

            //grid_container: DenseStorage::new(),
            //grid_item: DenseStorage::new(),
            //size_constraints: DenseStorage::new(),
            text: DenseStorage::new(),
        }
    }

    pub fn parse_theme(&mut self, stylesheet: &str) {
        let mut input = ParserInput::new(stylesheet);
        let mut parser = Parser::new(&mut input);
        let rule_parser = theme::RuleParser::new();

        let rules = {
            let rule_list_parser =
                cssparser::RuleListParser::new_for_stylesheet(&mut parser, rule_parser);
            rule_list_parser.collect::<Vec<_>>()
        };

        let mut rule_list: Vec<StyleRule> =
            rules.into_iter().filter_map(|rule| rule.ok()).collect();
        rule_list.sort_by_key(|rule| rule.specificity());
        rule_list.reverse();

        for rule in rule_list.iter() {
            let rule_id = self.rule_selectors.len();
            //println!("Rule: {}, Specificity: {:?}, rule: {:?}", rule_id, rule.specificity(), rule);
            self.rule_selectors.push(rule.selectors.clone());
            //self.rules.push(rule_id);
            for property in rule.properties.clone() {
                match property {
                    Property::Display(value) => {
                        self.display.insert_rule(rule_id, value);
                    }

                    Property::Visibility(value) => {
                        self.visibility.insert_rule(rule_id, value);
                    }

                    Property::Opacity(value) => {
                        self.opacity.insert_rule(rule_id, Opacity(value));
                    }

                    Property::Overflow(value) => {
                        self.overflow.insert_rule(rule_id, value);
                    }

                    Property::TextAlign(value) => {
                        self.text_align.insert_rule(rule_id, value);
                    }

                    Property::TextJustify(value) => {
                        self.text_justify.insert_rule(rule_id, value);
                    }

                    Property::Position(value) => {
                        self.position.insert_rule(rule_id, value);
                    }

                    Property::Left(value) => {
                        self.left.insert_rule(rule_id, value);
                    }

                    Property::Right(value) => {
                        self.right.insert_rule(rule_id, value);
                    }

                    Property::Top(value) => {
                        self.top.insert_rule(rule_id, value);
                    }

                    Property::Bottom(value) => {
                        self.bottom.insert_rule(rule_id, value);
                    }

                    Property::Width(value) => {
                        self.width.insert_rule(rule_id, value);
                    }

                    Property::Height(value) => {
                        self.height.insert_rule(rule_id, value);
                    }

                    Property::MaxWidth(value) => {
                        self.max_width.insert_rule(rule_id, value);
                    }

                    Property::MinWidth(value) => {
                        self.min_width.insert_rule(rule_id, value);
                    }

                    Property::MaxHeight(value) => {
                        self.max_height.insert_rule(rule_id, value);
                    }

                    Property::MinHeight(value) => {
                        self.min_height.insert_rule(rule_id, value);
                    }

                    Property::Margin(value) => {
                        self.margin_left.insert_rule(rule_id, value);
                        self.margin_right.insert_rule(rule_id, value);
                        self.margin_top.insert_rule(rule_id, value);
                        self.margin_bottom.insert_rule(rule_id, value);
                    }

                    Property::MarginLeft(value) => {
                        self.margin_left.insert_rule(rule_id, value);
                    }

                    Property::MarginRight(value) => {
                        self.margin_right.insert_rule(rule_id, value);
                    }

                    Property::MarginTop(value) => {
                        self.margin_top.insert_rule(rule_id, value);
                    }

                    Property::MarginBottom(value) => {
                        self.margin_bottom.insert_rule(rule_id, value);
                    }

                    Property::Padding(value) => {
                        self.padding_left.insert_rule(rule_id, value);
                        self.padding_right.insert_rule(rule_id, value);
                        self.padding_top.insert_rule(rule_id, value);
                        self.padding_bottom.insert_rule(rule_id, value);
                    }

                    Property::PaddingLeft(value) => {
                        self.padding_left.insert_rule(rule_id, value);
                    }

                    Property::PaddingRight(value) => {
                        self.padding_right.insert_rule(rule_id, value);
                    }

                    Property::PaddingTop(value) => {
                        self.padding_top.insert_rule(rule_id, value);
                    }

                    Property::PaddingBottom(value) => {
                        self.padding_bottom.insert_rule(rule_id, value);
                    }

                    // Border
                    Property::BorderWidth(value) => {
                        self.border_width.insert_rule(rule_id, value);
                    }

                    Property::BorderColor(value) => {
                        self.border_color.insert_rule(rule_id, value);
                    }

                    Property::BorderRadius(value) => {
                        self.border_radius.insert_rule(
                            rule_id,
                            BorderRadius {
                                top_left: value,
                                top_right: value,
                                bottom_left: value,
                                bottom_right: value,
                            },
                        );
                    }

                    Property::FontColor(value) => {
                        println!("Val: {:?}", value);
                        self.font_color.insert_rule(rule_id, value);
                    }

                    Property::BackgroundColor(value) => {
                        self.background_color.insert_rule(rule_id, value);
                    }

                    Property::BackgroundImage(value) => {
                        self.background_image.insert_rule(rule_id, value);
                    }

                    // Flex Container
                    Property::FlexDirection(value) => {
                        self.flex_direction.insert_rule(rule_id, value);
                    }
                    Property::JustifyContent(value) => {
                        self.justify_content.insert_rule(rule_id, value);
                    }
                    Property::AlignContent(value) => {
                        self.align_content.insert_rule(rule_id, value);
                    }
                    Property::AlignItems(value) => {
                        self.align_items.insert_rule(rule_id, value);
                    }

                    Property::AlignSelf(value) => {
                        self.align_self.insert_rule(rule_id, value);
                    }

                    // Flex Item
                    Property::FlexGrow(value) => {
                        self.flex_grow.insert_rule(rule_id, value);
                    }

                    Property::FlexShrink(value) => {
                        self.flex_shrink.insert_rule(rule_id, value);
                    }

                    Property::FlexBasis(value) => {
                        self.flex_basis.insert_rule(rule_id, value);
                    }

                    Property::Transition(transitions) => {
                        for transition in transitions {
                            match transition.property.as_ref() {
                                "background-color" => {
                                    self.background_color.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(transition.delay))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "left" => {
                                    self.left.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(transition.delay))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "width" => {
                                    self.width.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(transition.delay))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "opacity" => {
                                    self.opacity.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(transition.delay))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Add style data to an entity
    pub fn add(&mut self, entity: Entity) {
        self.pseudo_classes.insert(entity, PseudoClasses::default());

        //self.z_order.insert(entity, 0);

        self.overflow.insert(entity, Default::default());
        self.scroll.insert(entity, Default::default());

        self.visibility.insert(entity, Default::default());
        //self.clip_widget.insert(entity, Entity::new(0, 0));
        self.focus_order.insert(entity, Default::default());
        self.box_shadow.insert(entity, Default::default());
    }

    pub fn set_margin(&mut self, entity: Entity, value: f32) {
        // Check if there is already a rule with the same entity id
    }

    pub fn remove(&mut self, entity: Entity) {}

    pub fn insert_style_rule(&mut self, style_rule: StyleRule) -> &mut Self {
        self.style_rules.push(style_rule);

        self
    }

    pub fn insert_id(&mut self, entity: Entity, id: &str) -> &mut Self {
        let mut s = DefaultHasher::new();
        id.hash(&mut s);
        self.ids.insert(entity, s.finish());

        self
    }

    pub fn insert_element(&mut self, entity: Entity, element: &str) -> &mut Self {
        let mut s = DefaultHasher::new();
        element.hash(&mut s);
        self.elements.insert(entity, s.finish());

        self
    }

    pub fn insert_class(&mut self, entity: Entity, class: &str) -> &mut Self {
        if let Some(class_list) = self.classes.get_mut(entity) {
            class_list.insert(class.to_string());
        } else {
            let mut class_list = HashSet::new();
            class_list.insert(class.to_string());
            self.classes.insert(entity, class_list);
        }

        self
    }
}
