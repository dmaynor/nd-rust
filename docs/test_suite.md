# NetDisco Test Suite Documentation

## Overview

This document describes the test suite for the NetDisco-rust project. The test suite is designed to ensure the reliability, performance, and correctness of the network discovery and monitoring system.

## Test Categories

### 1. Unit Tests

Located in `tests/unit/`:

- **Network Discovery Tests**
  - `mac_tracking_test.rs`: Tests MAC address tracking functionality
  - `device_discovery_test.rs`: Tests device discovery operations
  - `topology_test.rs`: Tests network topology operations

- **Monitoring Tests**
  - `metrics_test.rs`: Tests metric collection and storage
  - `alerts_test.rs`: Tests alert rule evaluation and notification
  - `dashboard_test.rs`: Tests dashboard operations

- **User Management Tests**
  - `user_test.rs`: Tests user CRUD operations
  - `role_test.rs`: Tests role management
  - `permission_test.rs`: Tests permission system

### 2. Integration Tests

Located in `tests/integration/`:

- **Network Discovery Integration**
  - `device_discovery_test.rs`: Tests complete device discovery workflow
  - `topology_discovery_test.rs`: Tests topology mapping integration
  - `monitoring_integration_test.rs`: Tests monitoring system integration

### 3. End-to-End Tests

Located in `tests/e2e/`:

- **Complete System Tests**
  - `discovery_workflow_test.rs`: Tests complete discovery workflow
  - `monitoring_workflow_test.rs`: Tests complete monitoring workflow
  - `user_workflow_test.rs`: Tests complete user management workflow

### 4. Performance Tests

Located in `tests/performance/`:

- **Network Discovery Performance**
  - `network_discovery_perf_test.rs`: Tests discovery performance
  - Includes Criterion benchmarks for detailed analysis

## Test Configuration

### Environment Setup

1. Create a test configuration file at `tests/common/test_config.rs`:
   ```rust
   pub struct TestConfig {
       pub test_db_url: String,
       pub test_snmp_community: String,
       pub influx_url: String,
   }
   ```

2. Set up test database:
   ```rust
   async fn setup_test_db() -> Pool {
       // Initialize test database
   }
   ```

### Test Fixtures

Located in `tests/common/fixtures/`:

1. Device Fixtures:
   ```rust
   async fn create_test_device(db: &Pool) -> Device;
   async fn create_test_devices(db: &Pool, count: usize) -> Vec<Device>;
   async fn create_test_network(db: &Pool, devices: usize, connections: usize) -> Vec<Device>;
   ```

2. User Fixtures:
   ```rust
   async fn create_test_user(db: &Pool) -> User;
   async fn create_test_role(db: &Pool) -> Role;
   ```

## Running Tests

### Standard Test Suite

Run all tests:
```bash
cargo test
```

Run specific test categories:
```bash
cargo test --test unit
cargo test --test integration
cargo test --test e2e
```

### Performance Tests

Run performance tests:
```bash
cargo test --test performance
```

Run Criterion benchmarks:
```bash
cargo bench
```

## Test Coverage

To generate test coverage reports:

```bash
cargo tarpaulin --out Html
```

Coverage reports will be available in `target/tarpaulin/`.

## Writing New Tests

### Guidelines

1. **Test Organization**
   - Place tests in appropriate category directories
   - Use descriptive test names
   - Group related tests in the same file

2. **Test Structure**
   ```rust
   #[test]
   async fn test_something() {
       // 1. Setup
       let db = setup_test_db().await;
       
       // 2. Test execution
       let result = perform_operation().await;
       
       // 3. Assertions
       assert!(result.is_ok());
       
       // 4. Cleanup
       cleanup_test_db(&db).await;
   }
   ```

3. **Error Cases**
   - Test both success and failure scenarios
   - Verify error messages and types
   - Test edge cases and boundary conditions

4. **Performance Tests**
   - Define clear performance thresholds
   - Use appropriate test data sizes
   - Consider system resources and constraints

### Best Practices

1. **Database Management**
   - Always use test database
   - Clean up after tests
   - Use transactions for isolation

2. **Async Testing**
   - Use `#[tokio::test]` for async tests
   - Handle timeouts appropriately
   - Consider concurrent operations

3. **Mocking**
   - Use mock implementations for external services
   - Create realistic test data
   - Document mock behavior

4. **Documentation**
   - Document test purpose and requirements
   - Explain complex test scenarios
   - Update documentation when adding tests

## Continuous Integration

Tests are run automatically on:
- Pull request creation
- Push to main branch
- Daily scheduled runs

### CI Pipeline

1. **Setup**
   - Initialize test environment
   - Start required services

2. **Test Execution**
   - Run unit tests
   - Run integration tests
   - Run end-to-end tests
   - Run performance tests

3. **Reporting**
   - Generate test coverage report
   - Generate performance benchmarks
   - Archive test results

## Troubleshooting

Common issues and solutions:

1. **Database Connection Issues**
   - Verify test database URL
   - Check database permissions
   - Ensure cleanup after failed tests

2. **Performance Test Failures**
   - Check system resources
   - Verify test data size
   - Adjust timeouts if needed

3. **Network-Related Failures**
   - Check network connectivity
   - Verify SNMP configuration
   - Ensure test devices are accessible

## Contributing

When contributing new tests:

1. Follow existing test structure
2. Add appropriate documentation
3. Include performance considerations
4. Update test coverage
5. Verify CI pipeline success 