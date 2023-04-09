/*fn give_number(one: i16, two: i16) -> i16 {
    /*one * two*/
    let multiplied_by_ten = {
        let first_number = 10;
        first_number * one * two
        //이 안에 내용은 길게 써도 무방하다.
        //마지막에 세미콜론이 없는 이유는 return이기 때문.
    };
    multiplied_by_ten
    //multiplied_by_ten에 세미콜론을 하지않는이유는 return이기 때문.
}*/
/*fn give_age() -> i32 {
    26
}*/
/*fn empty_tuple() {}*/

//mutability라는것은 가변이 가능하다 라는것.
fn main() {
    let my_value = 10;
    let my_value = "My value";
    println!("My value is {my_value}");

    let my_variable = 9;
    print!("{}\n", my_variable);
    {
        let my_variable = "Some String";
        println!("{}", my_variable);
    }
    println!("{}", my_variable);
    //Shadowing 이라는것은 이름을 다시 쓰는것. 즉 이전것을 날려버리는게 아니라 임시적으로 block 해놓고 다시 쓰는것

    /*let mut my_number = 10;
    my_number = 9;
    */

    /*let my_number = 10;
    my_number = 9; //이럴때 변경 불가능하다. 그 이유는 immutable이 default이기 때문에 변경이 불가능. 만약 변경하고 싶다면 mut my_number 해줘야함
    */
    //let my_number = give_number(8, 9);
    //println!("My number is {my_number}");
    /*let tuple = empty_tuple();
    println!("{:?}", tuple);*/
    // {} -> Display {:?} -> Debug print

    /* let my_name = "SiHyeong";
    //let my_age = 26;
    println!("My name is {my_name} and my age is {}", give_age());

    let my_city = "Seoul";
    let year = 1998;
    let population = 89898989;
    println!(
        "The city of {city} in {year} had a population of {population}",
        city = my_city,
        year = year,
        population = population
    );
    println!(
        "The city of {0} in {1} had a population of {2}",
        my_city, year, population
    );*/
}
//macro = 코드를 쓰는 함수.
//{}안에서 expression 허용 x 뒤에서는 가능, 즉 1+5 못함
//ex) {year + 2} is impossible but {},year+2 is possible
