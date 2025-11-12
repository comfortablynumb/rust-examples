//! Binary application demonstrating workspace dependencies and features

use anyhow::Result;
use lib_core::{extra, json, Data};

#[cfg(feature = "all-features")]
use lib_core::advanced;

fn main() -> Result<()> {
    println!("=== Cargo Workspace Example - Binary App ===\n");

    // Create some sample data
    let data1 = Data::new(1, "Alice", 100.0);
    let data2 = Data::new(2, "Bob", 150.0);
    let data3 = Data::new(3, "Charlie", 200.0);

    println!("Created data items:");
    println!("  {:?}", data1);
    println!("  {:?}", data2);
    println!("  {:?}", data3);
    println!();

    // Validate data
    println!("Validating data...");
    data1.validate()?;
    data2.validate()?;
    data3.validate()?;
    println!("  All data valid!\n");

    // Calculate values
    println!("Calculated values:");
    println!("  Data 1: {}", data1.calculate());
    println!("  Data 2: {}", data2.calculate());
    println!("  Data 3: {}", data3.calculate());
    println!();

    // JSON serialization (feature: json)
    println!("=== JSON Serialization (feature: json) ===");
    let json_str = json::to_json_pretty(&data1)?;
    println!("Serialized data1:\n{}\n", json_str);

    let deserialized = json::from_json(&json_str)?;
    println!("Deserialized successfully: {:?}\n", deserialized);

    // Extra utilities (feature: extra)
    println!("=== Extra Utilities (feature: extra) ===");
    let items = vec![data1.clone(), data2.clone(), data3.clone()];

    let processed = extra::batch_process(&items);
    println!("Batch processed values: {:?}", processed);

    let max = extra::find_max(&items);
    println!("Maximum value: {:?}", max);

    let avg = extra::calculate_average(&items);
    println!("Average value: {}\n", avg);

    // Advanced features (feature: advanced, enabled via all-features)
    #[cfg(feature = "all-features")]
    {
        println!("=== Advanced Analysis (feature: advanced) ===");
        let analyzer = advanced::DataAnalyzer::new(items);
        let result = analyzer.analyze();

        println!("Analysis Results:");
        println!("  Count:   {}", result.count);
        println!("  Sum:     {:.2}", result.sum);
        println!("  Average: {:.2}", result.average);
        println!("  Min:     {:.2}", result.min);
        println!("  Max:     {:.2}", result.max);
        println!();
    }

    #[cfg(not(feature = "all-features"))]
    {
        println!("Advanced features not enabled. Build with --features all-features to enable.\n");
    }

    println!("=== Done! ===");
    Ok(())
}
