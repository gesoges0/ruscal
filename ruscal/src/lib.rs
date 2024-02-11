use clap::{App, Arg};
use std::error::Error;
use chrono::{NaiveDate, Datelike, Local};
use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
    today: NaiveDate,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("ruscal")
        .version("0.1.0")
        .author("gesoges0")
        .about("Rust cal")
        //
        .get_matches();

    let today = Local::today();

    Ok(Config {
        month: Some(today.month()),
        year: today.year(),
        today: today.naive_local(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

fn parse_int<T: FromStr>(val: &str) -> MyResult<T> {
    unimplemented!()
}

fn parse_year(year: &str) -> MyResult<i32> {
    unimplemented!();
}


fn parse_month(month: &str) -> MyResult<u32> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::{parse_int, parse_month, parse_year};

    #[test]
    fn test_parse_int() {
        // 正の整数をusizeとして解析する
        let res = parse_int::<usize>("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1usize);

        // 負の整数をi32として解析する
        let res = parse_int::<i32>("-1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), -1i32);

        // 数値以外の文字列を解析すると失敗する
        let res = parse_int::<i64>("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid integer \"foo\"");
    }

    #[test]
    fn test_parse_year() {
        let res = parse_year("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1i32);

        let res = parse_year("9999");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 9999i32);

        let res = parse_year("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "year \"0\" not in the range 1 through 9999");

        let res = parse_year("10000");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "yaer \"10000\" not in the range 1 through 9999");

        let res = parse_year("foo");
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_month() {
        let res = parse_month("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("12");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 12u32);

        let res = parse_month("jan");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "month \"0\" not in the range 1 through 12");

        let res = parse_month("13");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "month \"13\" not in the range 1 through 13");

        let res = parse_month("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid month \"foo\"");
    }
}

