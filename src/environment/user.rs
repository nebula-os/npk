use users::{get_effective_gid, get_effective_uid, get_group_by_gid, get_user_by_uid, Group, User};

pub fn get_effective_user() -> User {
    let current_uid = get_effective_uid();
    get_user_by_uid(current_uid).expect("Cannot find a user with the current effective unique id")
}

pub fn get_effective_group() -> Group {
    let current_gid = get_effective_gid();
    get_group_by_gid(current_gid).expect("Cannot find a group with the current effective unique id")
}
