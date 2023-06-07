use gtest::{Log, Program, System};
use tamagotchi::{TmgAction, TmgEvent};

#[test]
fn creation_test() {
    // initialize test environment
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("Hello"));
    // test tamagotchi initialization logic
    let expected_log = Log::builder().dest(2).payload(String::from("Tamagotchi created!"));
    assert!(res.contains(&expected_log));
    
    // test tamagotchi name request logic
    let res_name = program.send(2, TmgAction::Name);
    let expected_name = Log::builder().dest(2).payload(TmgEvent::Name(String::from("Tamasheepie")));
    assert!(res_name.contains(&expected_name));

    // test tamagotchi age request logic
    let _res_age = program.send(2, TmgAction::Age);

}