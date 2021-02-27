use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Mooshroom;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Mooshroom).add(EntityKind::Mooshroom);
}
