use rayon::prelude::*;

use crate::chart::ChartData;

pub fn extract_ohlcv_data(chart_data: &ChartData) -> Vec<(f64, f64, f64, f64, f64, f64)> {
    chart_data
        .series
        .par_iter()
        .map(|series_data_point| series_data_point.value)
        .collect()
}

#[allow(dead_code)]
pub fn sort_ohlcv_tuples(tuples: &mut Vec<(f64, f64, f64, f64, f64, f64)>) {
    tuples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
}

#[allow(dead_code)]
pub fn update_ohlcv_data(
    old_data: &mut Vec<(f64, f64, f64, f64, f64, f64)>,
    new_data: &Vec<(f64, f64, f64, f64, f64, f64)>,
) {
    for new_tuple in new_data {
        match old_data.binary_search_by(|&x| x.0.partial_cmp(&new_tuple.0).unwrap()) {
            Ok(index) => {
                old_data[index] = *new_tuple;
            }
            Err(index) => {
                old_data.insert(index, *new_tuple);
            }
        }
    }
    sort_ohlcv_tuples(old_data);
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::{ChartData, SeriesDataPoint};

    #[test]
    fn test_process_chart_data() {
        let chart_data = ChartData {
            node: None,
            series: vec![
                SeriesDataPoint {
                    value: (1.0, 2.0, 3.0, 4.0, 5.0, 6.0),
                    index: 1,
                },
                SeriesDataPoint {
                    index: 2,
                    value: (2.0, 3.0, 4.0, 5.0, 6.0, 7.0),
                },
                SeriesDataPoint {
                    index: 3,
                    value: (3.0, 4.0, 5.0, 6.0, 7.0, 8.0),
                },
                SeriesDataPoint {
                    index: 4,
                    value: (4.0, 5.0, 6.0, 7.0, 8.0, 9.0),
                },
                SeriesDataPoint {
                    index: 5,
                    value: (5.0, 6.0, 7.0, 8.0, 9.0, 10.0),
                },
                SeriesDataPoint {
                    index: 6,
                    value: (6.0, 7.0, 8.0, 9.0, 10.0, 11.0),
                },
                SeriesDataPoint {
                    index: 7,
                    value: (7.0, 8.0, 9.0, 10.0, 11.0, 12.0),
                },
                SeriesDataPoint {
                    index: 8,
                    value: (8.0, 9.0, 10.0, 11.0, 12.0, 13.0),
                },
                SeriesDataPoint {
                    index: 9,
                    value: (9.0, 10.0, 11.0, 12.0, 13.0, 14.0),
                },
                SeriesDataPoint {
                    index: 10,
                    value: (10.0, 11.0, 12.0, 13.0, 14.0, 15.0),
                },
            ],
        };
        let expected_output = vec![
            (1.0, 2.0, 3.0, 4.0, 5.0, 6.0),
            (2.0, 3.0, 4.0, 5.0, 6.0, 7.0),
            (3.0, 4.0, 5.0, 6.0, 7.0, 8.0),
            (4.0, 5.0, 6.0, 7.0, 8.0, 9.0),
            (5.0, 6.0, 7.0, 8.0, 9.0, 10.0),
            (6.0, 7.0, 8.0, 9.0, 10.0, 11.0),
            (7.0, 8.0, 9.0, 10.0, 11.0, 12.0),
            (8.0, 9.0, 10.0, 11.0, 12.0, 13.0),
            (9.0, 10.0, 11.0, 12.0, 13.0, 14.0),
            (10.0, 11.0, 12.0, 13.0, 14.0, 15.0),
        ];
        let output = extract_ohlcv_data(&chart_data);
        assert_eq!(output, expected_output);
    }
    #[test]
    fn test_update_ohlcv_data() {
        let mut old_data = vec![
            (1691560800.0, 83800.0, 83900.0, 83000.0, 83100.0, 708100.0),
            (1691647200.0, 82600.0, 82800.0, 82200.0, 82200.0, 784500.0),
            (1691733600.0, 81600.0, 83000.0, 81500.0, 82000.0, 625500.0),
            (1691632800.0, 83100.0, 83300.0, 82300.0, 82600.0, 558800.0),
            (1691719200.0, 82000.0, 82200.0, 81600.0, 81600.0, 517400.0),
        ];

        let new_data = vec![
            (1691647200.0, 82700.0, 82900.0, 82100.0, 82300.0, 800000.0),
            (1691733600.0, 81700.0, 83100.0, 81600.0, 83000.0, 700000.0),
            (1691632800.0, 83200.0, 83400.0, 82400.0, 82700.0, 600000.0),
            (1691805600.0, 82500.0, 82700.0, 82000.0, 82100.0, 900000.0),
        ];

        update_ohlcv_data(&mut old_data, &new_data);

        let expected_output = vec![
            (1691560800.0, 83800.0, 83900.0, 83000.0, 83100.0, 708100.0),
            (1691632800.0, 83200.0, 83400.0, 82400.0, 82700.0, 600000.0),
            (1691647200.0, 82700.0, 82900.0, 82100.0, 82300.0, 800000.0),
            (1691719200.0, 82000.0, 82200.0, 81600.0, 81600.0, 517400.0),
            (1691733600.0, 81700.0, 83100.0, 81600.0, 83000.0, 700000.0),
            (1691805600.0, 82500.0, 82700.0, 82000.0, 82100.0, 900000.0),
        ];

        assert_eq!(old_data, expected_output);
    }
}
