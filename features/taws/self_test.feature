Feature: Self-Test (MOPS_001)
  TAWS Equipment shall (TAWS_MOPS_001) include a self-test function that
  provides indications of equipment condition during operation. This self-test
  function consists of continuous and initiated tests.

  Scenario: Continuos Self-Test (MOPS_002, MOPS_003, MOPS_004)
    When a failure is detected by the continuos monitoring
    Then the TAWS reports the failure
    And disarms all affected functions

  # can be done using interaction with the crew
  Scenario: Initiated Self-Test  (MOPS_005)
    When the self-test is initiated
    Then the TAWS mus verify integrity of all fault reporting facilities of the test

  Scenario: Initiated Self-Test (MOPS_006)
    Given the self-test is initiated
    When the initiated self-test detects a failure
    Then the failure must be reported

  #Scenario: Automatic Arming (MOPS_007)

  Scenario: Input Data Smoothing
    When the rate of input data reduces or stagnates
    Then the delays to alert onset do not 

# vim: set ts=2 sw=2 expandtab: retab: expandtab #
