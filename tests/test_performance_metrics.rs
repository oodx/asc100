#[cfg(feature = "metrics")]
mod metrics_tests {
    use asc100::char::versions::V1_STANDARD;
    use asc100::char::extensions::CoreStrategy;
    use asc100::metrics::{timed_encode, EncodingMetrics};
    use std::time::Duration;

    #[test]
    fn test_metrics_collection() {
        let input = "Hello, World!";
        let strategy = CoreStrategy::strict();
        
        let (result, metrics) = timed_encode(input, || {
            V1_STANDARD.encode_with(input, &strategy).expect("Should encode")
        });
        
        assert!(metrics.is_some());
        let m = metrics.unwrap();
        assert_eq!(m.input_length, input.len());
        assert_eq!(m.output_length, result.len());
        assert!(m.encoding_time_nanos > 0);
        assert!(m.compression_ratio > 0.0);
    }

    #[test]
    fn test_metrics_format_summary() {
        let metrics = EncodingMetrics::new(100, 115, Duration::from_millis(2));
        let summary = metrics.format_summary();
        
        assert!(summary.contains("115%"));
        assert!(summary.contains("2.00ms"));
        assert!(summary.contains("chars/ms"));
    }

    #[test]
    fn test_with_metrics_macro() {
        let input = "Test input for metrics";
        
        let (result, metrics) = asc100::with_metrics!(input.len(), {
            V1_STANDARD.encode(input)
        });
        
        assert!(result.is_ok());
        assert!(metrics.is_some());
        
        let m = metrics.unwrap();
        assert_eq!(m.input_length, input.len());
        assert!(m.encoding_time_nanos > 0);
    }
}

#[cfg(not(feature = "metrics"))]
mod no_metrics_tests {
    use asc100::char::versions::V1_STANDARD;
    use asc100::char::extensions::CoreStrategy;

    #[test]
    fn test_metrics_disabled() {
        let input = "Hello, World!";
        let strategy = CoreStrategy::strict();
        
        let (result, metrics) = asc100::metrics::timed_encode(input, || {
            V1_STANDARD.encode_with(input, &strategy).expect("Should encode")
        });
        
        assert!(result.len() > 0);
        assert!(metrics.is_none()); // Metrics should be disabled
    }

    #[test]
    fn test_with_metrics_macro_disabled() {
        let input = "Test input";
        
        let (result, metrics): (Result<String, asc100::Asc100Error>, Option<asc100::metrics::EncodingMetrics>) = asc100::with_metrics!(input.len(), {
            V1_STANDARD.encode(input)
        });
        
        assert!(result.is_ok());
        assert!(metrics.is_none()); // Metrics should be disabled
    }
}