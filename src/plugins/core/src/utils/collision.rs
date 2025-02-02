use avian3d::prelude::*;
use bevy::prelude::*;

pub fn get_contactdata_global_position(
    entity: Entity,
    transform: &Transform,
    contacts: &Contacts,
    contact_data: &ContactData,
) -> Vec3 {
    if entity == contacts.entity1 {
        return contact_data.global_point1(
            &Position(transform.translation),
            &Rotation(transform.rotation),
        );
    } else if entity == contacts.entity2 {
        return contact_data.global_point2(
            &Position(transform.translation),
            &Rotation(transform.rotation),
        );
    }
    Vec3::ZERO
}
