#noinspection CucumberUndefinedStep
Feature: Mode 3: Negative Climb Rate or Altitude Loss
  The Mode 3 alert is intended to generate caution alerts when the aircraft
  loses altitude during take-off or go around. The loss of altitude can either
  be based on a negative altitude rate or it can be based on a measured loss of
  altitude.

  @MOPS_283
  Scenario: Mode Arming/Disarming
    Given take-off is selected
    Then Mode 3 shall be armed

  @MOPS_283
  Scenario: Mode Arming/Disarming
    Given go around is selected
    Then Mode 3 shall be armed

  @MOPS_284
  Scenario: Mode Arming/Disarming
    Given take-off is not selected
    And go around is not selected
    Then Mode 3 shall not be armed

  # MOPS_285: either alerting on Mode 1 or Mode 2 is sufficient, if both are
  # used again only one is sufficient

  #Rule: Method 1
  @MOPS_286
  Scenario Outline: Must Alert
    Given Mode 3 is armed
    And Mode 3 is not inhibited
    When the rate of descent is at least <rate_of_descent> feet per minute
    And the height above terrain is between 100 and <height> feet
    Then a Mode 3 caution alert is emitted within 1.3 seconds

    Examples:
      | rate_of_descent | height |
      | 207             | 60     |
      | 533             | 600    |
      | 534             | 600    |

  @MOPS_287
  Scenario: Must Not Alert when not Armed
    Given Mode 3 is not armed
    Then a Mode 3 caution alert is not emitted at all

  @MOPS_287
  Scenario: Must Not Alert when Inhibited
    Given Mode 3 is inhibited
    Then a Mode 3 caution alert is not emitted at all

  @MOPS_287
  Scenario Outline: Must Not Alert
    When the rate of descent is at most <rate_of_descent> feet per minute
    But the height above terrain is not between 10 and <height> feet
    Then a Mode 3 caution alert is not emitted at all


