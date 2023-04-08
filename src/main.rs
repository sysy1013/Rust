fn give_number(one: i16, two: i16) -> i16 {
    /*one * two*/
    let multiplied_by_ten = {
        let first_number = 10;
        first_number * one * two
        //이 안에 내용은 길게 써도 무방하다.
        //마지막에 세미콜론이 없는 이유는 return이기 때문.
    };
    multiplied_by_ten
    //multiplied_by_ten에 세미콜론을 하지않는이유는 return이기 때문.
}
/*fn give_age() -> i32 {
    26
}*/
/*fn empty_tuple() {}*/
fn main() {
    let my_number = give_number(8, 9);
    println!("My number is {my_number}");
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
