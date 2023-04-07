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
