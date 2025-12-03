mod display_service;

#[cfg(test)]
mod tests {
    use display_service::DisplayService;

    use super::*;
    use crate::domain::*;

    #[test]
    fn test_display_service_initialization() {
        let display_service = DisplayService::new();
        assert!(display_service.is_initialized());
    }
}
