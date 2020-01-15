use std::ops::Range;
use gtmpl_value::Value;
use std::collections::HashMap;

// Add 1
gtmpl_fn!(
    #[doc = r#"Add 1 to a value."#]
    fn add1_i64(n: i64) -> Result<String, String> {
        Ok((n + 1).to_string())
    }
);

gtmpl_fn!(
    #[doc = r#"Add 1 to a value."#]
    fn add1_f64(n: f64) -> Result<String, String> {
        Ok((n + 1.00).to_string())
    }
);

// Add
gtmpl_fn!(
    #[doc = r#"Add 1 value to another."#]
    fn add_i64(n1: i64, n2: i64) -> Result<String, String> {
        Ok((n1 + n2).to_string())
    }
);

gtmpl_fn!(
    #[doc = r#"Add 1 value to another."#]
    fn add_f64(n1: f64, n2: f64) -> Result<String, String> {
        Ok((n1 + n2).to_string())
    }
);

// Subtract
gtmpl_fn!(
    #[doc = r#"Subtract 1 value from another."#]
    fn sub_i64(n1: i64, n2: i64) -> Result<String, String> {
        Ok((n1 - n2).to_string())
    }
);

gtmpl_fn!(
    #[doc = r#"Subtract 1 value from another."#]
    fn sub_f64(n1: f64, n2: f64) -> Result<String, String> {
        Ok((n1 - n2).to_string())
    }
);

// Divide
gtmpl_fn!(
    #[doc = r#"Divide 1 value by another."#]
    fn div_i64(n1: i64, n2: i64) -> Result<String, String> {
        Ok((n1 / n2).to_string())
    }
);

gtmpl_fn!(
    #[doc = r#"Divide 1 value by another."#]
    fn div_f64(n1: f64, n2: f64) -> Result<String, String> {
        Ok((n1 / n2).to_string())
    }
);

// Modulus
gtmpl_fn!(
    #[doc = r#"Mod when 1 value divided by another."#]
    fn mod_i64(n1: i64, n2: i64) -> Result<String, String> {
        Ok((n1 % n2).to_string())
    }
);

gtmpl_fn!(
    #[doc = r#"Mod when 1 value divided by another."#]
    fn mod_f64(n1: f64, n2: f64) -> Result<String, String> {
        Ok((n1 % n2).to_string())
    }
);

// Multiply
gtmpl_fn!(
    #[doc = r#"Multiply one value by another."#]
    fn mul_i64(n1: i64, n2: i64) -> Result<String, String> {
        Ok((n1 * n2).to_string())
    }
);

gtmpl_fn!(
    #[doc = r#"Multiply one value by another."#]
    fn mul_f64(n1: f64, n2: f64) -> Result<String, String> {
        Ok((n1 * n2).to_string())
    }
);

gtmpl_fn!(
    #[doc = r#"Create a sequence until a number."#]
    fn until(n: u64) -> Result<Vec<Value>, String> {
        let range = Range { start: 0, end: n };
//        let mut map = HashMap::new();
//        for v in range {
//            map.insert(
//                v.to_string(),
//                v.to_string(),
//            );
//        };
//        Ok(map)

        let mut vec = Vec::new();
        for v in range {
            vec.push(Value::from(v.to_string()));
        }
        Ok(vec)
    }
);


#[cfg(test)]
mod test {
    use super::*;
    use gtmpl_value::Value;

    // Add 1
    #[test]
    fn test_add1_i64() {
        test_fn!(add1_i64, vval!(1), "2");
    }

    #[test]
    fn test_add1_f64() {
        test_fn!(add1_f64, vval!(1.5), "2.5");
    }

    // Add
    #[test]
    fn test_add_i64() {
        test_fn!(add_i64, vval!(1,2), "3");
    }

    #[test]
    fn test_add_f64() {
        test_fn!(add_f64, vval!(1.5,1.0), "2.5");
    }

    // Subtract
    #[test]
    fn test_sub_i64() {
        test_fn!(sub_i64, vval!(5,2), "3");
    }

    #[test]
    fn test_sub_f64() {
        test_fn!(sub_f64, vval!(1.5,1.0), "0.5");
    }

    // Divide
    #[test]
    fn test_div_i64() {
        test_fn!(div_i64, vval!(6,2), "3");
    }

    #[test]
    fn test_div_f64() {
        test_fn!(div_f64, vval!(5.0,2.0), "2.5");
    }

    // Modulus
    #[test]
    fn test_mod_i64() {
        test_fn!(mod_i64, vval!(5,3), "2");
    }

    #[test]
    fn test_mod_f64() {
        test_fn!(mod_f64, vval!(8.0,2.5), "0.5");
    }

    // Until
    #[test]
    fn test_until() {

        let mut vec = Vec::new();
        vec.push(0.to_string());
        vec.push(1.to_string());
        vec.push(2.to_string());

//        let mut map = HashMap::new();
//        map.insert(
//            1.to_string(),
//            1.to_string(),
//        );
//        map.insert(
//            2.to_string(),
//            2.to_string(),
//        );
//        map.insert(
//            3.to_string(),
//            3.to_string(),
//        );


        test_fn!(until, vval!(3), vec);
    }

}