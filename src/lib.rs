#[macro_export]
macro_rules! rv {
    ()=>{
Vec::new()
    };
    ($($element:expr),+$(,)?)=>{
        {
        let  mut vs= Vec::new();
        $(vs.push($element);)+
        vs
        }
    };
}
#[test]
fn empty_vec(){
let x:Vec<u32>= rv![];
assert!(x.is_empty());
}

#[test]
fn single (){
    let x:Vec<u32>= rv![10];
    assert!(!x.is_empty());
    assert_eq!(x[0],10);
    assert_eq!(x.len(), 1);
}

#[test]
fn double (){
    let x:Vec<u32>= rv![10,20,];
    assert!(!x.is_empty());
    assert_eq!(x[0],10);
     assert_eq!(x[1],20);
    assert_eq!(x.len(), 2);
}