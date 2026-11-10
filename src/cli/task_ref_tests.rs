use crate::cli::task_ref::TaskRef;

#[test]
fn ticket_ids_are_recognized() {
    assert_eq!(TaskRef::parse("KAN-12"), TaskRef::Ticket("KAN-12"));
    assert_eq!(TaskRef::parse("vlc-7"), TaskRef::Ticket("vlc-7"));
    assert_eq!(
        TaskRef::parse("EVLMS-FRONT-141"),
        TaskRef::Ticket("EVLMS-FRONT-141")
    );
    assert_eq!(TaskRef::parse(" SB2-3 "), TaskRef::Ticket("SB2-3"));
}

#[test]
fn raw_ids_stay_ids() {
    assert_eq!(
        TaskRef::parse("mjQXp4D8gdu6HOeMymkPy51fHg9bnBmE"),
        TaskRef::Id("mjQXp4D8gdu6HOeMymkPy51fHg9bnBmE")
    );
    assert_eq!(TaskRef::parse("KAN-"), TaskRef::Id("KAN-"));
    assert_eq!(TaskRef::parse("-12"), TaskRef::Id("-12"));
    assert_eq!(TaskRef::parse("12-12"), TaskRef::Id("12-12"));
    assert_eq!(TaskRef::parse("KAN-12a"), TaskRef::Id("KAN-12a"));
    assert_eq!(TaskRef::parse("KAN_X-1"), TaskRef::Id("KAN_X-1"));
}
