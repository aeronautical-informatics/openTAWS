@DO_367 @CLASS_C @PDA
Feature: Premature Descent Alerting (PDA)
	The PDA functionality monitors the final approach and
	alerts the crew if the aircraft is about to crash into the ground short of the runway.
	Detection of these conditions is based upon height above runway elevation or
	height above terrain/obstacles and distance to runway.

  Background:
    Given the aircraft is on a landing segment

  @ARMING_DISARMING @MOPS_263
  Scenario: Class C Equipment shall arm PDA within 5 NM of an airport.
    Given the distance to the nearest airport is less than 5 NM
    Then PDA shall be armed

  Rule: PDA is armed and not inhibited

    Background:
      Given PDA is not inhibited

    @ALERT_CRITERIA
    Scenario: Class C Equipment should not generate PDA alerts for normal visual flight rules (VFR) operations in the airport area.
      Given the distance to the nearest airport is less than 5 NM
      When the height above terrain is at least 800 feet
      Then a PDA alert is no emitted

    @ALERT_CRITERIA
    Scenario: Airplanes routinely operate in VFR conditions ... within 10-15 NM of the nearest airport, ... should not generate alerts ...
      Given the distance to the nearest airport is less than 10 NM
      When the height above terrain is at least 1000 feet
      Then a PDA alert is no emitted

    @ALERT_CRITERIA
    Scenario: Airplanes routinely operate in the visual segment of a circling approach ... should not generate PDA alerts ...
      Given the distance to the nearest airport is less than 2 NM
      And circling approach is selected
      When the height above terrain is at least 300 feet
      Then a PDA alert is no emitted

    @ALERT_CRITERIA @CAUTION @MOPS_264
    Scenario Outline: Class C Equipment shall provide a caution alert when ...
      When the distance to runway is between <min_distance_runway> and <max_distance_runway> NM
      And the height above terrain is between 10.0 and <max_height_ground> feet
      Then a PDA caution alert is emitted within 1.3 seconds

      Examples:
        | min_distance_runway | max_distance_runway | max_height_ground |
        |                 1.0 |                 1.8 |              80.0 |
        |                1.81 |                 2.3 |             150.0 |
        |                2.31 |                 5.0 |             170.0 |

  Rule: PDA is not-armed or inhibited

    @ALERT_CRITERIA @CAUTION @MOPS_265
    Scenario: Class C Equipment shall not provide a PDA caution alert when ... PDA is not armed.
      When the distance to the nearest airport is at least 5.1 NM
      And the height above terrain is between 10.0 and 170.0 feet
      Then PDA is not armed
      And a PDA caution is not emitted

    @ALERT_CRITERIA @CAUTION @MOPS_265
    Scenario: Class C Equipment shall not provide a PDA caution alert when ... PDA is inhibited.
      Given PDA is inhibited
      When the distance to runway is 1.0 NM
      And the height above terrain is 10.0 feet
      Then a PDA caution is no emitted

  Rule: PDA is armed and not-inhibited and an alert is active

    @AURAL_ALERT @CAUTION @MOPS_266
    Scenario: Class C Equipment shall be capable of generating or triggering an aural message of at least one of “Too Low” and “Too Low Terrain”.
      Given not implemented

    @VISUAL_ALERT @CAUTION @MOPS_267
    Scenario: Class C Equipment shall be capable of providing an output to trigger a yellow or amber indication.
      Given not implemented
