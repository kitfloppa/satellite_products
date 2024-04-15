use table_macro::Property;

#[test]
fn test_auto_getter() {
    #[derive(Property)]
    struct User {
        id: i32,
    }

    let user = User { id: 0 };

    assert_eq!(user.get_id(), 0);
}

#[test]
fn test_auto_setter() {
    #[derive(Property)]
    struct User {
        id: i32,
    }

    let mut user = User { id: 0 };
    user.set_id(1);

    assert_eq!(user.get_id(), 1);
}

#[test]
fn test_full_getter() {
    #[derive(Property)]
    struct User {
        #[getter]
        #[setter]
        id: i32,
    }

    let user = User { id: 0 };

    assert_eq!(user.get_id(), 0);
}

#[test]
fn test_full_setter() {
    #[derive(Property)]
    struct User {
        #[getter]
        #[setter]
        id: i32,
    }

    let mut user = User { id: 0 };
    user.set_id(1);

    assert_eq!(user.get_id(), 1);
}

#[test]
fn test_none_side_effect() {
    #[derive(Property)]
    struct User {
        #[none]
        _id: i32,
        project_id: i32,
    }

    let mut user = User {
        _id: 0,
        project_id: 1,
    };

    assert_eq!(user.get_project_id(), 1);
    user.set_project_id(2);
    assert_eq!(user.get_project_id(), 2);
}

#[test]
fn test_only_getter() {
    #[derive(Property)]
    struct User {
        #[getter]
        id: i32,
    }

    let mut user = User { id: 0 };
    assert_eq!(user.get_id(), 0);
    user.id = 1;
    assert_eq!(user.get_id(), 1);
}

#[test]
fn test_only_getter_side_effect() {
    #[derive(Property)]
    struct User {
        #[getter]
        _id: i32,
        project_id: i32,
    }

    let mut user = User {
        _id: 0,
        project_id: 1,
    };

    assert_eq!(user.get_project_id(), 1);
    user.set_project_id(2);
    assert_eq!(user.get_project_id(), 2);
}

#[test]
fn test_only_setter() {
    #[derive(Property)]
    struct User {
        #[setter]
        id: i32,
    }

    let mut user = User { id: 0 };
    user.set_id(1);
    assert_eq!(user.id, 1);
}

#[test]
fn test_only_setter_side_effect() {
    #[derive(Property)]
    struct User {
        #[setter]
        _id: i32,
        project_id: i32,
    }

    let mut user = User {
        _id: 0,
        project_id: 1,
    };

    assert_eq!(user.get_project_id(), 1);
    user.set_project_id(2);
    assert_eq!(user.get_project_id(), 2);
}

// TODO: add tests for WELL_KNOWN_TYPES
