//! Chart and data visualization types.
//!
//! This module provides types for representing charts and their data
//! in a format-agnostic way that can be rendered by different backends.

/// Chart kinds supported by the AST.
///
/// Defines the type of chart that should be rendered. Different backends may
/// support different subsets of these chart types, with graceful degradation
/// to tables or other representations when necessary.
///
/// # Variants
///
/// * `Bar` - Vertical or horizontal bar chart
/// * `Line` - Line chart with connected data points
/// * `Pie` - Pie chart showing proportional data
/// * `Scatter` - Scatter plot with x,y coordinate points
/// * `Area` - Area chart (filled line chart)
///
/// # Examples
///
/// ```rust
/// use lontar::core::*;
///
/// let chart = Block::Chart {
///     title: Some("Sales Data".to_string()),
///     kind: ChartKind::Bar,
///     data: ChartData {
///         categories: vec!["Q1".to_string(), "Q2".to_string()],
///         series: vec![ChartSeries {
///             name: "Revenue".to_string(),
///             values: vec![1000.0, 1500.0],
///         }],
///     },
///     style: None,
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartKind {
    Bar,
    Line,
    Pie,
    Scatter,
    Area,
}

/// Chart data representation (categories + series values).
///
/// Contains the data for a chart, organized as categories (x-axis labels) and
/// one or more data series. This structure allows for both simple single-series
/// charts and complex multi-series visualizations.
///
/// # Fields
///
/// * `categories` - Labels for each data point (x-axis values)
/// * `series` - One or more data series with their corresponding values
///
/// # Examples
///
/// ```rust
/// use lontar::core::*;
///
/// let data = ChartData {
///     categories: vec![
///         "January".to_string(),
///         "February".to_string(),
///         "March".to_string(),
///     ],
///     series: vec![
///         ChartSeries {
///             name: "Sales".to_string(),
///             values: vec![1000.0, 1200.0, 1100.0],
///         },
///         ChartSeries {
///             name: "Expenses".to_string(),
///             values: vec![800.0, 900.0, 850.0],
///         },
///     ],
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ChartData {
    pub categories: Vec<String>,
    pub series: Vec<ChartSeries>,
}

/// A single chart series.
///
/// Represents one data series in a chart, containing a name for the series
/// and the corresponding data values. The number of values should match
/// the number of categories in the parent `ChartData`.
///
/// # Fields
///
/// * `name` - Display name for this series (used in legends)
/// * `values` - Numeric values for each category in the series
///
/// # Examples
///
/// ```rust
/// use lontar::core::*;
///
/// let series = ChartSeries {
///     name: "Revenue 2024".to_string(),
///     values: vec![1000.0, 1500.0, 1200.0, 1800.0],
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ChartSeries {
    pub name: String,
    pub values: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_data_creation() {
        let data = ChartData {
            categories: vec!["Q1".to_string(), "Q2".to_string(), "Q3".to_string()],
            series: vec![ChartSeries {
                name: "Sales".to_string(),
                values: vec![1000.0, 1200.0, 1100.0],
            }],
        };

        assert_eq!(data.categories.len(), 3);
        assert_eq!(data.series.len(), 1);
        assert_eq!(data.series[0].name, "Sales");
        assert_eq!(data.series[0].values, vec![1000.0, 1200.0, 1100.0]);
    }

    #[test]
    fn test_chart_series_creation() {
        let series = ChartSeries {
            name: "Revenue".to_string(),
            values: vec![100.0, 200.0, 150.0, 300.0],
        };

        assert_eq!(series.name, "Revenue");
        assert_eq!(series.values.len(), 4);
        assert_eq!(series.values[0], 100.0);
        assert_eq!(series.values[3], 300.0);
    }

    #[test]
    fn test_chart_kinds() {
        let kinds = vec![
            ChartKind::Bar,
            ChartKind::Line,
            ChartKind::Pie,
            ChartKind::Scatter,
            ChartKind::Area,
        ];

        // Test that all chart kinds are distinct
        for (i, kind1) in kinds.iter().enumerate() {
            for (j, kind2) in kinds.iter().enumerate() {
                if i != j {
                    assert_ne!(kind1, kind2);
                }
            }
        }
    }

    #[test]
    fn test_multi_series_chart_data() {
        let data = ChartData {
            categories: vec!["Jan".to_string(), "Feb".to_string(), "Mar".to_string()],
            series: vec![
                ChartSeries {
                    name: "Revenue".to_string(),
                    values: vec![1000.0, 1200.0, 1100.0],
                },
                ChartSeries {
                    name: "Expenses".to_string(),
                    values: vec![800.0, 900.0, 850.0],
                },
                ChartSeries {
                    name: "Profit".to_string(),
                    values: vec![200.0, 300.0, 250.0],
                },
            ],
        };

        assert_eq!(data.categories.len(), 3);
        assert_eq!(data.series.len(), 3);

        // Verify each series has the same number of data points as categories
        for series in &data.series {
            assert_eq!(series.values.len(), data.categories.len());
        }
    }

    #[test]
    fn test_empty_chart_data() {
        let data = ChartData {
            categories: vec![],
            series: vec![],
        };

        assert!(data.categories.is_empty());
        assert!(data.series.is_empty());
    }

    #[test]
    fn test_chart_data_with_empty_categories() {
        let data = ChartData {
            categories: vec![],
            series: vec![ChartSeries {
                name: "Empty Series".to_string(),
                values: vec![],
            }],
        };

        assert!(data.categories.is_empty());
        assert_eq!(data.series.len(), 1);
        assert!(data.series[0].values.is_empty());
    }
}
