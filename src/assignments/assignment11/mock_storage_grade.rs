//! Test cases for assignment11/mock_storage.rs

#[cfg(test)]
mod test_mock_storage {
    use crate::assignments::assignment11::mock_storage::*;

    #[test]
    fn test_mock_storage() {
        let mock_storage = MockStorage::new(100);

        let uploader1 = FileUploader::new(&mock_storage);
        let uploader2 = FileUploader::new(&mock_storage);

        let usage_analyzer = UsageAnalyzer::new(&mock_storage, 0.75);

        assert!(uploader1.upload("file1.txt", 20).is_ok());
        assert!(usage_analyzer.is_usage_under_bound());

        assert!(uploader2.upload("file2.txt", 30).is_ok());
        assert!(usage_analyzer.is_usage_under_bound());

        assert!(uploader1.upload("file3.txt", 40).is_ok());
        assert!(!usage_analyzer.is_usage_under_bound());

        assert_eq!(uploader2.upload("file4.txt", 50), Err(40));
        assert!(!usage_analyzer.is_usage_under_bound());

        assert!(uploader1.upload("file3.txt", 10).is_ok());
        assert!(usage_analyzer.is_usage_under_bound());
    }
}
