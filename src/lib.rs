// View assembly generated via https://godbolt.org/
use core::ops::Add;

#[derive(Debug)]
struct Value {
    data: f64,
    gradient: f64,
    _previous: Vec<Value>,
    _op: String,
    label: String,
}

impl Value {
    fn new(data: f64, children: Vec<Value>, op: Option<String>, label: Option<String>) -> Value {
        Value {
            data: data,
            gradient: 0.0,
            _previous: children,
            _op: op.unwrap_or("".to_string()),
            label: label.unwrap_or("".to_string()),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        let data = self.data + other.data;
        let children = vec![self, other];
        let output = Value::new(data, children, Some(("+").to_string()), None);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization() {
        let data: f64 = 1.2;
        const CHILDREN: Vec<Value> = vec![];

        let result = Value::new(data, CHILDREN, None, Some("first".to_string()));

        assert_eq!(result.data, 1.2);
        assert_eq!(result.gradient, 0.0);
        assert_eq!(result.gradient, 0.0);
    }

    #[test]
    fn add_op() {
        let a = Value::new(1.3, vec![], None, None);
        let b = Value::new(1.2, vec![], None, None);

        let c = a + b;

        assert_eq!(c.data, 2.5);
        assert_eq!(c._op, "+");
    }
}
