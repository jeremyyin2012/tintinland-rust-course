/// =SUM(number1,[number2],...)
macro_rules! sum {
    ($($x:expr),*) => {
        {
            0$(+$x)*
        }
    };
}

/// =IF(C2=1,”Yes”,”No”)
macro_rules! r#if {
    ($c:expr, $x:expr, $y: expr) => {
        {
            if $c {
                $x
            } else {
                $y
            }
        }
    };
}

/// =XLOOKUP(lookup_value, lookup_array, return_array, [if_not_found], [match_mode], [search_mode])
macro_rules! xlookup {
    ($v:expr,$la:expr,$ra:expr,$d:expr,$mm:expr,$sm:expr) => {
        {
            // todo mm sm
            let mut res = $d;
            for (idx, item) in $la.iter().enumerate() {
                if $v == *item {
                    res = $ra[idx];
                    break;
                }
            };
            res
        }
    };
}

fn main() {
    println!("Hello, world!");
    let s = sum!(1,2,3,4,3+4,100-98,2/1,4*8,9/3,3/2);
    println!("{s}");
    let r = r#if!(3>0, 1, 0);
    println!("{r}");
    let res = xlookup!(2, vec![1,2,3], vec!["a","b","c"], "a", 1, 1);
    println!("{res:?}");
}
