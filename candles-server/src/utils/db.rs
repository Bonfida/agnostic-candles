use crate::structs::tradingview::Resolution;

pub fn make_query(resolution: Resolution) -> String {
    let query = format!(
        "
            SELECT
                time_bucket('{}', timestamp) as ts_start,
                first(open, timestamp) as open,
                MIN(low) as low,
                last(close, timestamp) as close,
                MAX(high) as high
            FROM
                candles
            WHERE
                timestamp > $1
                AND
                timestamp < $2
                AND
                name = $3
            GROUP BY
                ts_start
            ORDER BY
                ts_start
",
        resolution.to_interval()
    );
    query
}
