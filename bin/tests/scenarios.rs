#[test]
fn all_java_samples_work_correctly() -> Result<(), Box<dyn std::error::Error>> {
    let sample_names = get_samples_names()?;

    for sample_path in sample_names {
        let base = std::fs::read_to_string(format!("{}/base.java", sample_path.display()))?;
        let left = std::fs::read_to_string(format!("{}/left.java", sample_path.display()))?;
        let right = std::fs::read_to_string(format!("{}/right.java", sample_path.display()))?;

        let expected = std::fs::read_to_string(format!("{}/merge.java", sample_path.display()))?;
        let result = bin::run_tool_on_merge_scenario(model::Language::Java, &base, &left, &right)?;

        assert_eq!(expected, result.to_string())
    }

    Ok(())
}

fn get_samples_names() -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    std::fs::read_dir("tests/scenarios")?
        .filter(|sample| {
            sample
                .as_ref()
                .map(|sample| sample.path().is_dir())
                .unwrap_or(false)
        })
        .map(|sample| sample.map(|sample| sample.path()))
        .collect()
}
