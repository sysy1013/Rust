<h2>1. Comments</h2>
   <p>// <-일반적인 주석</p>
   <p>/// <-documentation</p>
   <p>\* \* <-여러줄 주석</p>

<h2>2. Types</h2>
    <p>i8, i16, i32, i64, i28 이것은 isize =  Signed Integer</p>
    <p>u8, u16, u32, u64, u128 이것은 usize = Unsigned</p>
    <p>Unsigned는 -가 될 수 없는 숫자</p>
    <p>bits = 8 = 1byte</p>
    <p>Example -> let my_name = 100; 으로 설정을 해주지 않았다면 자동적으로 i32 즉 32bit할당</p>
    <p>만약 지정해주고싶다면, let my_name :u8 = 100</p>
    <p>let my_number: u8 = 100</p>
    <p>let my_other_number = 50 -> i32</p>
    <p>let third_number = my_number + my_other_number ->오류 생김. 그 이유는 u8과 i32는 계산을 할 수 없기 때문</p>

<h2>3. Chars</h2>
    <p>String은 무조건 ""으로 해주어야함</p>
    <p>Example -> 'Hello,world' X "Hello,world"</p>
    <p>Char -> let first_letter = 'A';</p>
    <p>Char는 이모지도 가능하고, 빈칸도 가능하다.</p>
    <p>모든 Char는 4bytes</p>
    <h3>Casting</h3>
    <p>i8과 i16은 더하기가 안된다. 이럴때 Casting이필요</p>
    <p>third_number = my_number + second_number as u 16 ->이런식으로 as를 사용하여서 캐스팅을 해준다면 더하기가 가능하다.</p>

<h2>4. Chars</h2>
    <p>Rust에서 .len()이라는것은 글자수를 나타내는것이 아니라 크기 즉 bytes를 나타낸다.</p>
    <p>숫자 뒤에는 그냥 바로 적어도됨.</p>
    <p>Example -> let my_number = 9u8;</p>
    <p>Example -> let my_number = 1_000_000u64;
    이런식으로 아래 _있더라도 무시하고 u64로 인식</p>
<h2>5. float</h2>
    <p>let my_number = 9. ->뒤에 작성하지 않아도 float으로 인식한다.</p>
    <p>f32 / f64 -> 대부분 f64로 사용한다.</p>
