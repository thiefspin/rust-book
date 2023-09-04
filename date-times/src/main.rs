use chrono::prelude::*;

fn utc_date_time() -> DateTime<Utc> {
    let utc: DateTime<Utc> = Utc::now();
    return utc;
}

fn local_date_time() -> DateTime<Local> {
    let local: DateTime<Local> = Local::now();
    return local;
}

fn sast_date_time() -> DateTime<FixedOffset> {
    let utc: DateTime<Utc> = Utc::now();
    let offset = FixedOffset::east_opt(2*3600).unwrap();
    let sast: DateTime<FixedOffset> = DateTime::with_timezone(&utc, &offset);
    return sast;
}

fn main() {
    let utc = utc_date_time();
    let local = local_date_time();
    let sast = sast_date_time();
    println!("UTC time: {}", utc);
    println!("Local time: {}", local);
    println!("Sast time: {}", sast);
    let now = local_date_time();
    let after = local_date_time();
    println!("{} is before {} equals: {}", now, after, after.gt(&now))
    println!("Putting the time in json using serde: {}", )
}
