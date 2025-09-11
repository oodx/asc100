//! Performance metrics collection for ASC100 encoding/decoding operations
//! 
//! This module provides optional performance instrumentation that can be enabled
//! with the `metrics` feature flag. When disabled, all metrics operations compile
//! to zero-cost abstractions.

use std::time::{Duration, Instant};

/// Performance metrics for encoding/decoding operations
#[derive(Debug, Clone)]
pub struct EncodingMetrics {
    pub input_length: usize,
    pub output_length: usize,
    pub encoding_time_nanos: u64,
    pub compression_ratio: f64,
    pub throughput_chars_per_ms: f64,
}

impl EncodingMetrics {
    pub fn new(input_length: usize, output_length: usize, duration: Duration) -> Self {
        let nanos = duration.as_nanos() as u64;
        let compression_ratio = if input_length == 0 { 
            1.0 
        } else { 
            output_length as f64 / input_length as f64 
        };
        
        let throughput_chars_per_ms = if nanos == 0 {
            0.0
        } else {
            (input_length as f64 * 1_000_000.0) / nanos as f64
        };

        Self {
            input_length,
            output_length,
            encoding_time_nanos: nanos,
            compression_ratio,
            throughput_chars_per_ms,
        }
    }
    
    pub fn duration(&self) -> Duration {
        Duration::from_nanos(self.encoding_time_nanos)
    }
    
    pub fn compression_percentage(&self) -> u32 {
        (self.compression_ratio * 100.0).round() as u32
    }
    
    pub fn format_summary(&self) -> String {
        format!(
            "{}% compression, {:.2}ms, {:.0} chars/ms", 
            self.compression_percentage(),
            self.encoding_time_nanos as f64 / 1_000_000.0,
            self.throughput_chars_per_ms
        )
    }
}

/// Timer for measuring encoding/decoding operations
pub struct MetricsTimer {
    start: Instant,
    input_length: usize,
}

impl MetricsTimer {
    pub fn new(input_length: usize) -> Self {
        Self {
            start: Instant::now(),
            input_length,
        }
    }
    
    pub fn finish(self, output_length: usize) -> EncodingMetrics {
        let duration = self.start.elapsed();
        EncodingMetrics::new(self.input_length, output_length, duration)
    }
}

/// Macro for optional metrics collection
/// When metrics feature is disabled, this compiles to zero cost
#[cfg(feature = "metrics")]
#[macro_export]
macro_rules! with_metrics {
    ($input_len:expr, $operation:expr) => {{
        let timer = $crate::metrics::MetricsTimer::new($input_len);
        let result = $operation;
        match &result {
            Ok(output) => {
                let metrics = timer.finish(output.len());
                (result, Some(metrics))
            }
            Err(_) => (result, None)
        }
    }};
}

#[cfg(not(feature = "metrics"))]
#[macro_export]
macro_rules! with_metrics {
    ($input_len:expr, $operation:expr) => {{
        ($operation, None)
    }};
}

/// Convenience function for timed encoding operations
#[cfg(feature = "metrics")]
pub fn timed_encode<F, R>(input: &str, operation: F) -> (R, Option<EncodingMetrics>)
where
    F: FnOnce() -> R,
    R: AsRef<str>,
{
    let timer = MetricsTimer::new(input.len());
    let result = operation();
    let metrics = timer.finish(result.as_ref().len());
    (result, Some(metrics))
}

#[cfg(not(feature = "metrics"))]
pub fn timed_encode<F, R>(_input: &str, operation: F) -> (R, Option<EncodingMetrics>)
where
    F: FnOnce() -> R,
    R: AsRef<str>,
{
    (operation(), None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_encoding_metrics() {
        let duration = Duration::from_millis(5);
        let metrics = EncodingMetrics::new(100, 120, duration);
        
        assert_eq!(metrics.input_length, 100);
        assert_eq!(metrics.output_length, 120);
        assert_eq!(metrics.compression_percentage(), 120);
        assert_eq!(metrics.duration(), duration);
    }

    #[test]
    fn test_metrics_timer() {
        let timer = MetricsTimer::new(50);
        std::thread::sleep(Duration::from_millis(1));
        let metrics = timer.finish(60);
        
        assert_eq!(metrics.input_length, 50);
        assert_eq!(metrics.output_length, 60);
        assert!(metrics.encoding_time_nanos > 0);
    }

    #[test]
    fn test_format_summary() {
        let duration = Duration::from_millis(2);
        let metrics = EncodingMetrics::new(100, 115, duration);
        let summary = metrics.format_summary();
        
        assert!(summary.contains("115%"));
        assert!(summary.contains("2.00ms"));
        assert!(summary.contains("chars/ms"));
    }
}