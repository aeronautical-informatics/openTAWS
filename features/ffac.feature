Feature: Five Hundred Foot Altitude Callout

  @MOPS_292
  Scenario: Five hundred foot above terrain
    Given FFAC is armed
    And FFAC is not inhibited
    And non-precision approach is selected
    And the height above terrain is greater or equal to 500 foot
    When the height above terrain is lower than 500 foot
    Then a FFAC annunciation alert is emitted within 1.3 seconds

  @MOPS_292
  Scenario: Five hundred foot above nearest runway elevation
    Given FFAC is armed
    And FFAC is not inhibited
    And non-precision approach is selected
    And the height above nearest runway elevation is greater or equal to 500 foot
    When the height above nearest runway elevation is lower than 500 foot
    Then a FFAC annunciation alert is emitted within 1.3 seconds