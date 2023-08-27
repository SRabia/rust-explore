use adder;

#[test]
fn integration_adds_two(){
        let result = adder::add(2, 2);
        assert_eq!(result, 4);
}
