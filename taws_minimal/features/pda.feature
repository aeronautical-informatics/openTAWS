#noinspection CucumberUndefinedStep
Feature: PDA: Premature Decent Alerting
  # TODO: Description

  @MOPS_263
  Scenario: Mode Arming/Disarming
    Given the plane is within 5 NM of an airport
    Then PDA shall be armed

  # TODO Add Criteria for PDA to be disarmed beyond 5 NM due to the risk of nuisance alerts (2.2.3.1.7.2)

  @MOPS_264
  Scenario Outline: Must Alert
    Given PDA is armed
    And PDA is not inhibited
    When the distance to the nearest airport is between 1.0 and <distance_to_airport> NM
    And the height above terrain is between 10 and <height> feet
    Then a PDA caution alert is emitted within 1.3 seconds

    Examples:
      | distance_to_airport | height |
      | 1.0                 | 80     |
      | 1.8                 | 150    |
      | 2.3                 | 170    |

  @MOPS_265
  Scenario: Must Not Alert when not Armed
    Given PDA is not armed
    Then a PDA caution alert is not emitted at all

  @MOPS_265
  Scenario: Must Not Alert when Inhibited
    Given PDA is inhibited
    Then a PDA caution alert is not emitted at all

  # MOPS_266 and MOPS_267 are not testable

# vim: set ts=2 sw=2 expandtab: retab: expandtab #
