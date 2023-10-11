use std::str::FromStr;

pub fn get_env<T: FromStr>(name: &str) -> Option<T>
where
    T::Err: std::fmt::Debug,
{
    let v = std::env::var(name);
    let Ok(v) = v else {
        return None;
    };

    Some(v.parse::<T>().expect(&format!("{} is invalid.", name)))
}

pub fn get_env_or<T: FromStr>(name: &str, default_value: T) -> T
where
    T::Err: std::fmt::Debug,
{
    let v = std::env::var(name);
    let Ok(v) = v else {
        return default_value;
    };

    v.parse::<T>().unwrap_or(default_value)
}
