pub fn format_millis(millis: i64) -> String {
    let minutes = millis / 60000;
    let seconds = (millis % 60000) / 1000;
    let seconds_display = if seconds < 10 {
        format!("0{}", seconds)
    } else {
        format!("{}", seconds)
    };

    if seconds == 60 {
        format!("{}:00", minutes + 1)
    } else {
        format!("{}:{}", minutes, seconds_display)
    }
}

#[macro_export]
macro_rules! early_return_option {
    ($option:expr) => {
        match $option {
            Some(x) => x,
            None => return,
        }
    };
}

/// returns unit if the expression evaluates to true
#[macro_export]
macro_rules! early_return_bool {
    ($b:expr) => {
        if $b {
            return;
        }
    };
}
