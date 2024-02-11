use clap::{App, Arg};
use std::error::Error;
use chrono::{NaiveDate, Datelike, Local};
use std::str::FromStr;

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

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
        .arg(
            Arg::with_name("month")
                .value_name("MONTH")
                .short("m")
                .help("Month name or number")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("show_current_year")
                .value_name("SHOW_YEAR")
                .short("y")
                .long("year")
                .help("Show wole current year")
                .conflicts_with_all(&["month", "year"])
        )
        .arg(
            Arg::with_name("year")
                .value_name("YEAR")
                .help("Year (1-9999)"),
        )
        .get_matches();

    // let today = Local::today();
    let mut month = matches.value_of("month").map(parse_month).transpose()?;
    let mut year = matches.value_of("year").map(parse_year).transpose()?;
    let today = Local::today();
    if matches.is_present("show_current_year") {
        month = None;
        year = Some(today.year());
    } else if month.is_none() && year.is_none() {
        month = Some(today.month());
        year = Some(today.year());
    }

    Ok(Config {
        month,
        year: year.unwrap_or_else(|| today.year()),
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
    // yearを受け取る
    // yearをparse_intする
    // 1~9999の間に含まれていたら検証済みのnumを返す
    // 含まれたいなかったら例外を上げる
    parse_int(year).and_then(|num| {
        if (1..=9999).contains(&num) {
            Ok(num)
        } else {
            Err(format!("year \"{}\" not in the range 1 through 9999", year).into())
        }
    })
}


fn parse_month(month: &str) -> MyResult<u32> {
    // 数値のみの解析
    match parse_int(month) {
        // num が1~12だった場合はその値を返す
        // そうでなかった場合は例外
        Ok(num) => {
            if (1..=12).contains(&num) {
                Ok(num)
            } else {
                Err(format!("month \"{}\" not in the range 1 through 12", month).into())
            }
        }
        // 数値でなかった場合は小文字化して定数配列と比較していく
        // 配列は0-indexで月は1-indexなので、インデックスiが一致したらi+1が月となる
        // 引数とマッチしたものをmatchesに格納していく
        _ => {
            let lower = &month.to_lowercase();
            let matches: Vec<_> = MONTH_NAMES
                .iter()
                .enumerate()
                .filter_map(|(i, name)| {
                    if name.to_lowercase().starts_with(lower) {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .collect();
            if matches.len() == 1 {
                Ok(matches[0] as u32)
            } else {
                Err(format!("Invalid month \"{}\"", month).into())
            }
        }
    }
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

