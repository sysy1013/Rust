fn give_age() -> i32 {
    26
}
fn main() {
    let my_name = "SiHyeong";
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
    );
}
//macro = 코드를 쓰는 함수.
//{}안에서 expression 허용 x 뒤에서는 가능, 즉 1+5 못함
//ex) {year + 2} is impossible but {},year+2 is possible
