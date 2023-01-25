use bitflags::bitflags;

pub type Permission = i32;

trait PermissionTrait {
  fn is_valid() -> bool;
}

impl PermissionTrait for Permission {
  fn is_valid() -> bool {
    return true;
  }
}

bitflags! {
  pub struct Permissions: Permission {
    const CREATE_KEY_PAIR = 0x01;
    const DELETE_KEY_PAIR = 0x02;
  }
}